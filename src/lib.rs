pub mod event;
pub mod input;
pub mod layers;
mod renderer;

use event::process_event;
use input::InputContext;
use layers::{imgui::ImguiLayer, LayerStack};
use renderer::{
    buffer::{IndexBuffer, Vertex, VertexBuffer},
    utils::{Mesh, Shader},
    Renderer,
};

pub use imgui::Ui;

use anyhow::Result;
use futures::executor::block_on;
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
    window::{Window, WindowBuilder},
};

pub struct Application {
    pub name: String,
    pub running: bool,
    pub delta_t: Duration,
    pub input_context: InputContext,
    pub v_sync: bool,
    window: Box<Window>,
    renderer: Renderer,
    render_pipeline: wgpu::RenderPipeline,
    mesh: Mesh,
}

impl Application {
    /// Create a new Application
    /// It returns the new application with a layer_stack and an event_loop
    /// you most likely don't have to touch the event_loop and can simply pass it to the run function
    ///
    /// These are returned and then passed to the run function to make lifetimes easier to manage
    /// Example usage:
    /// ```rust
    ///     use hazel::Application;
    ///     fn main() -> anyhow::Result<()> {
    ///         let (mut app, mut layer_stack, mut event_loop) =
    ///         Application::new("Example", None)?;
    ///         app.run(&mut layer_stack, &mut event_loop)
    ///     }
    /// ```
    pub fn new(
        name: &str,
        imgui_ini_path: Option<PathBuf>,
    ) -> Result<(Self, LayerStack, EventLoop<()>)> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(name).build(&event_loop)?;
        let v_sync = true;

        log::trace!("Window created");

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        };
        let renderer = block_on(Renderer::new(&window, clear_color, v_sync))?;

        log::trace!("Renderer created");

        // FIXME this should probably not be here
        let mesh = Mesh {
            vertex_buffer: VertexBuffer::create(&renderer.device, VERTICES),
            index_buffer: IndexBuffer::create(&renderer.device, INDICES),
        };

        let shader = Shader::compile(
            String::from(include_str!("assets/shaders/vert.glsl")),
            String::from(include_str!("assets/shaders/frag.glsl")),
        )?;

        log::trace!("Shaders compiled");

        let render_pipeline = shader.create_pipeline(
            &renderer.device,
            &renderer.sc_desc,
            &renderer.pipeline_layout,
            &mesh.vertex_buffer,
            &mesh.index_buffer,
            1,
        );

        log::trace!("Render pipeline created");

        let mut layer_stack = LayerStack::new();
        // FIXME push the overlay in the run() fn to make sure it's the last one
        layer_stack.push_overlay(Rc::new(RefCell::new(ImguiLayer::new(
            imgui_ini_path,
            v_sync,
        ))));

        log::trace!("Application created");

        Ok((
            Application {
                name: String::from(name),
                window: Box::new(window),
                running: false,
                delta_t: Duration::default(),
                renderer,
                input_context: InputContext::new(),
                v_sync,
                mesh,
                render_pipeline,
            },
            layer_stack,
            event_loop,
        ))
    }

    /// The main loop
    /// You need to call this otherwise nothing will happen
    /// This is where event are handled and this is where every licecycle hook is called
    pub fn run(
        &mut self,
        layer_stack: &mut LayerStack,
        event_loop: &mut EventLoop<()>,
    ) -> Result<(), anyhow::Error> {
        layer_stack.on_attach(self);

        self.running = true;

        log::info!("Application started");

        event_loop.run_return(|event, _, control_flow| {
            *control_flow = if self.running {
                ControlFlow::Poll
            } else {
                ControlFlow::Exit
            };

            layer_stack.on_winit_event(self, &event);

            if let Some(event) = process_event(self, &event) {
                layer_stack.on_event(self, &event);
            }

            match event {
                winit::event::Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    self.close();
                }
                winit::event::Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    self.delta_t = self.renderer.last_frame.elapsed();
                    self.renderer.last_frame = Instant::now();

                    layer_stack.on_update(self);

                    if let Ok((mut encoder, frame)) = self.renderer.begin_render() {
                        {
                            let mut render_pass =
                                self.renderer.begin_render_pass(&mut encoder, &frame);
                            render_pass.set_pipeline(&self.render_pipeline);
                            render_pass.set_vertex_buffer(0, &self.mesh.vertex_buffer.buffer, 0, 0);
                            render_pass.set_index_buffer(&self.mesh.index_buffer.buffer, 0, 0);
                            render_pass.draw_indexed(0..self.mesh.index_buffer.count, 0, 0..1);
                        }

                        layer_stack.on_imgui_render(self);
                        layer_stack.on_wgpu_render(self, &mut encoder, &frame);

                        self.renderer.submit(encoder);
                    }

                    self.renderer.last_frame_duration = self.delta_t;
                }
                _ => {}
            }
        });

        layer_stack.on_detach(self);

        log::info!("Application stopped");
        Ok(())
    }

    pub fn close(&mut self) {
        log::info!("Close requested");
        log::info!("Application stopping");
        self.running = false;
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2];
