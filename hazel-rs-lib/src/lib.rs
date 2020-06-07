mod renderer;

use renderer::{ImguiState, Renderer};

use futures::executor::block_on;
use log::{info, trace};
use std::time::Instant;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub trait Application {
    fn run(&self);
}

pub struct Game {
    name: String,
}

impl Application for Game {
    fn run(&self) {
        trace!("Game started!");

        use winit::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        };

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&self.name)
            .build(&event_loop)
            .unwrap();

        trace!("Window created");

        let mut imgui_state = ImguiState::new(&window, 1.0);
        let mut renderer = block_on(Renderer::new(&window, &mut imgui_state.context));

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent { ref event, .. } => match event {
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
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size, None);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        scale_factor,
                        ..
                    } => {
                        renderer.resize(**new_inner_size, Some(*scale_factor));
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let delta_t = renderer.last_frame.elapsed();
                    renderer.last_frame = Instant::now();

                    // TODO update

                    let ui = imgui_state.prepare(&window, delta_t);
                    renderer.render(ui, delta_t);
                    renderer.last_frame_duration = delta_t;
                }
                _ => {}
            }
            imgui_state
                .platform
                .handle_event(imgui_state.context.io_mut(), &window, &event);
        });
    }
}

pub fn create_app(name: &str) -> Game {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .filter_module("gfx_descriptor", log::LevelFilter::Warn)
        .filter_module("gfx_memory", log::LevelFilter::Warn)
        .filter_module("gfx_backend_vulkan", log::LevelFilter::Warn)
        .init();
    trace!("Initialized logging");
    Game {
        name: String::from(name),
    }
}
