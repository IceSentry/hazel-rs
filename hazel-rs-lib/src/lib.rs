mod imgui;
pub mod layers;
mod renderer;

use layers::LayerStack;
use renderer::{render, Renderer};

pub use crate::imgui::ImguiLayer;
use crate::imgui::ImguiState;
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
    scale_factor: f64,
    imgui_state: Option<ImguiState>,
    pub delta_t: Duration,
    window: Box<Window>,
    renderer: Renderer,
}

pub fn run(
    app: &mut Application,
    layer_stack: &mut LayerStack,
    event_loop: &mut EventLoop<()>,
) -> Result<(), anyhow::Error> {
    app.running = true;

    trace!("Game started!");

    #[allow(clippy::while_immutable_condition)]
    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { ref event, .. } => {
                if handle_close(event) {
                    app.running = false;
                    *control_flow = ControlFlow::Exit;
                } else {
                    handle_resize(event, &mut app.renderer);
                }
            }
            Event::MainEventsCleared => {
                app.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let delta_t = app.renderer.last_frame.elapsed();
                app.renderer.last_frame = Instant::now();

                app.delta_t = delta_t;
                for layer in layer_stack.layers.iter() {
                    layer.on_update(app);
                }

                render(app, layer_stack);
                app.renderer.last_frame_duration = delta_t;
            }
            _ => {}
        }

        for layer in layer_stack.layers.iter() {
            layer.on_event(app, &event);
        }
    });

    Ok(())
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
            imgui_state: None,
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

fn handle_resize(event: &WindowEvent, renderer: &mut Renderer) {
    match event {
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
    }
}
