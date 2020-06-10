pub mod layers;
mod renderer;

use layers::LayerStack;
use renderer::{render, Renderer};

use futures::executor::block_on;
use log::{info, trace};
use std::time::{Duration, Instant};
pub use winit::event::Event;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
    window::{Window, WindowBuilder},
};

pub struct Application {
    pub name: String,
    pub running: bool,
    pub delta_t: Duration,
    scale_factor: f64,
    window: Box<Window>,
    renderer: Renderer,
}

impl Application {
    pub fn run(
        &mut self,
        layer_stack: &mut LayerStack,
        event_loop: &mut EventLoop<()>,
    ) -> Result<(), anyhow::Error> {
        layer_stack.on_attach(self);

        self.running = true;

        trace!("Application started");

        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { ref event, .. } => {
                    if handle_close(event) {
                        self.running = false;
                        *control_flow = ControlFlow::Exit;
                    }
                    handle_resize(event, self);
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let delta_t = self.renderer.last_frame.elapsed();
                    self.renderer.last_frame = Instant::now();

                    self.delta_t = delta_t;
                    for layer in layer_stack.layers.iter_mut() {
                        layer.on_update(self);
                    }

                    render(self, layer_stack);
                    self.renderer.last_frame_duration = delta_t;
                }
                _ => {}
            }

            for layer in layer_stack.layers.iter_mut() {
                layer.on_event(self, &event);
            }
        });

        layer_stack.on_detach(self);

        trace!("Application stopped");
        Ok(())
    }
}

pub fn create_app(name: &str) -> Result<(Application, LayerStack, EventLoop<()>), anyhow::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .filter_module("gfx_descriptor", log::LevelFilter::Warn)
        .filter_module("gfx_memory", log::LevelFilter::Warn)
        .filter_module("gfx_backend_vulkan", log::LevelFilter::Warn)
        .init();
    trace!("Initialized logging");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(name).build(&event_loop)?;

    trace!("Window created");

    let renderer = block_on(Renderer::new(&window));

    trace!("Renderer created");

    let layer_stack = LayerStack::new();

    Ok((
        Application {
            name: String::from(name),
            window: Box::new(window),
            running: false,
            scale_factor: 1.0,
            delta_t: Duration::default(),
            renderer,
        },
        layer_stack,
        event_loop,
    ))
}

fn handle_close(event: &WindowEvent) -> bool {
    match event {
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    state: ElementState::Pressed,
                    ..
                },
            ..
        }
        | WindowEvent::CloseRequested => {
            info!("The close button was pressed; stopping");
            true
        }
        _ => false,
    }
}

fn handle_resize(event: &WindowEvent, app: &mut Application) {
    match event {
        WindowEvent::Resized(physical_size) => {
            app.renderer.resize(*physical_size, None);
        }
        WindowEvent::ScaleFactorChanged {
            new_inner_size,
            scale_factor,
            ..
        } => {
            app.renderer.resize(**new_inner_size, Some(*scale_factor));
        }
        _ => {}
    }
}
