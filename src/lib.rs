pub mod event;
pub mod input;
pub mod layers;
pub mod renderer;

use event::process_event;
use input::InputContext;
use layers::{imgui::ImguiLayer, LayerStack};
use renderer::{orthographic_camera::OrthographicCamera, renderer_api::RendererApi, Renderer};

pub use imgui::Ui;

use anyhow::Result;
use futures::executor::block_on;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use wgpu::SwapChainOutput;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub type Frame = SwapChainOutput;

pub struct Application {
    pub name: String,
    pub delta_t: Duration,
    pub input_context: InputContext,
    pub v_sync: bool,
    pub renderer: Renderer,
    close_requested: bool,
    camera: OrthographicCamera,
    window: Box<Window>,
    imgui_ini_path: Option<PathBuf>,
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
        let window = WindowBuilder::new()
            .with_title(name)
            .with_visible(false)
            .build(&event_loop)?;
        let v_sync = true;

        log::trace!("Window created");

        let camera = OrthographicCamera::new(-1.0, 1.0, -1.0, 1.0);

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        };

        let renderer = {
            let renderer_api = block_on(RendererApi::new(&window, clear_color, v_sync))?;
            Renderer::new(renderer_api)
        };

        log::trace!("Renderer created");

        let layer_stack = LayerStack::new();

        log::trace!("Application created");

        Ok((
            Application {
                name: String::from(name),
                window: Box::new(window),
                delta_t: Duration::default(),
                renderer,
                input_context: InputContext::new(),
                v_sync,
                camera,
                imgui_ini_path,
                close_requested: false,
            },
            layer_stack,
            event_loop,
        ))
    }

    pub fn close(&mut self) {
        log::info!("Close requested");

        self.close_requested = true;
    }
}

/// The main loop
/// You need to call this otherwise nothing will happen
/// This is where event are handled and this is where every licecycle hook is called
pub fn run(app: Application, layer_stack: LayerStack, event_loop: EventLoop<()>) {
    let mut app = app;
    let mut layer_stack = layer_stack;

    layer_stack.push_overlay(Box::new(ImguiLayer::new()));
    layer_stack.on_attach(&mut app);

    app.window.set_visible(true);
    log::info!("Application started");

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            app.close();
        }
        winit::event::Event::MainEventsCleared => {
            app.delta_t = app.renderer.api.last_frame.elapsed();
            app.renderer.api.last_frame = Instant::now();

            layer_stack.on_update(&mut app);

            layer_stack.on_before_render(&mut app);

            if let Ok(frame) = app.renderer.api.begin_render() {
                layer_stack.on_imgui_render(&mut app);
                layer_stack.on_render(&mut app, &frame);

                app.renderer.api.end_render();
            }

            app.renderer.api.last_frame_duration = app.delta_t;

            if app.close_requested {
                log::info!("Application stopping");
                layer_stack.on_detach(&mut app);
                log::info!("Application stopped");
                app.close_requested = false;
                *control_flow = ControlFlow::Exit;
            }
        }
        _ => {
            layer_stack.on_winit_event(&mut app, &event);

            if let Some(event) = process_event(&mut app, &event) {
                layer_stack.on_event(&mut app, &event);
            }
        }
    });
}
