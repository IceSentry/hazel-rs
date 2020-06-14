pub mod event;
pub mod input;
pub mod layers;
mod renderer;

use layers::{imgui::ImguiLayer, LayerStack};
use renderer::Renderer;

use event::Event;
use futures::executor::block_on;
use input::InputContext;

pub use imgui::Ui;

use anyhow::Result;
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};
use winit::{
    event::{ElementState, WindowEvent},
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
    scale_factor: f64,
    window: Box<Window>,
    renderer: Renderer,
}

impl Application {
    pub fn quit(&mut self) {
        self.running = false;
    }

    fn process_event(&mut self, event: &winit::event::Event<()>) -> Option<Event> {
        self.input_context.update(&event);

        if let winit::event::Event::WindowEvent { ref event, .. } = event {
            match event {
                WindowEvent::Resized(physical_size) => {
                    self.renderer.resize(*physical_size, None);
                    return Some(Event::WindowResize);
                }
                WindowEvent::ScaleFactorChanged {
                    new_inner_size,
                    scale_factor,
                    ..
                } => {
                    self.renderer.resize(**new_inner_size, Some(*scale_factor));
                    return Some(Event::ScaleFactorChanged);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        return match input.state {
                            ElementState::Pressed => Some(Event::KeyPressed(keycode)),
                            ElementState::Released => Some(Event::KeyReleased(keycode)),
                        };
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    return match state {
                        ElementState::Pressed => Some(Event::MouseButtonPressed(
                            *button,
                            self.input_context.mouse_position,
                        )),
                        ElementState::Released => Some(Event::MouseButtonReleased(
                            *button,
                            self.input_context.mouse_position,
                        )),
                    };
                }
                _ => {}
            }
        }
        None
    }

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

            if let Some(event) = self.process_event(&event) {
                layer_stack.on_event(self, &event);
            }

            match event {
                winit::event::Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    log::info!("Close requested; stopping");
                    *control_flow = ControlFlow::Exit;
                }
                winit::event::Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    self.delta_t = self.renderer.last_frame.elapsed();
                    self.renderer.last_frame = Instant::now();

                    layer_stack.on_update(self);

                    if let Ok((mut encoder, frame)) = self.renderer.begin_render() {
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
}

pub fn create_app(
    name: &str,
    imgui_ini_path: Option<PathBuf>,
) -> Result<(Application, LayerStack, EventLoop<()>)> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(name).build(&event_loop)?;
    let v_sync = true;

    log::trace!("Window created");

    let renderer = block_on(Renderer::new(&window, v_sync))?;

    log::trace!("Renderer created");

    let mut layer_stack = LayerStack::new();
    layer_stack.push_overlay(Rc::new(RefCell::new(ImguiLayer::new(
        imgui_ini_path,
        v_sync,
    ))));

    Ok((
        Application {
            name: String::from(name),
            window: Box::new(window),
            running: false,
            scale_factor: 1.0,
            delta_t: Duration::default(),
            renderer,
            input_context: InputContext::new(),
            v_sync,
        },
        layer_stack,
        event_loop,
    ))
}
