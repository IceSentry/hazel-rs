// #![windows_subsystem = "windows"]

use hazel::{
    event::Event,
    input::VirtualKeyCode,
    layers::{debug_text::DebugTextLayer, iced_ui::IcedUiLayer, Layer},
    renderer::{
        buffer::{Vertex, VertexPos},
        pipeline::Pipeline,
        utils::{Shader, VertexArray},
    },
    Application, CommandEncoder, SwapChainOutput,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

struct ExampleLayer {
    triangle_pipeline: Option<Pipeline<Vertex>>,
    square_pipeline: Option<Pipeline<VertexPos>>,
}

impl ExampleLayer {
    fn new() -> Self {
        Self {
            triangle_pipeline: None,
            square_pipeline: None,
        }
    }
}

impl Layer for ExampleLayer {
    fn get_name(&self) -> String {
        String::from("example-layer")
    }

    fn on_attach(&mut self, app: &mut Application) {
        let triangle_vertex_array = {
            let vertices = &[
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.8, 0.2, 0.8, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.2, 0.3, 0.8, 1.0],
                },
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [0.8, 0.8, 0.2, 1.0],
                },
            ];

            let indices = &[0, 1, 2];

            VertexArray::create(&app.renderer.device, vertices, indices)
        };

        let square_vertex_array = {
            let vertices = &[
                VertexPos {
                    position: [-0.75, -0.75, 0.0],
                },
                VertexPos {
                    position: [0.75, -0.75, 0.0],
                },
                VertexPos {
                    position: [0.75, 0.75, 0.0],
                },
                VertexPos {
                    position: [-0.75, 0.75, 0.0],
                },
            ];

            let indices = &[0, 1, 2, 2, 3, 0];

            VertexArray::create(&app.renderer.device, vertices, indices)
        };

        let shader = Shader::compile(
            String::from(include_str!("assets/shaders/vert.glsl")),
            String::from(include_str!("assets/shaders/frag.glsl")),
        )
        .expect("failed to compile");

        let blue_shader = Shader::compile(
            String::from(include_str!("assets/shaders/vert_blue.glsl")),
            String::from(include_str!("assets/shaders/frag_blue.glsl")),
        )
        .expect("failed to compile");

        self.triangle_pipeline = Some(Pipeline::new(&app.renderer, &shader, triangle_vertex_array));
        self.square_pipeline = Some(Pipeline::new(
            &app.renderer,
            &blue_shader,
            square_vertex_array,
        ));
    }

    fn on_update(&mut self, app: &mut Application) {
        if app.input_context.is_key_pressed(VirtualKeyCode::A) {
            log::debug!("A poll");
        }
    }

    fn on_render(
        &mut self,
        app: &mut Application,
        encoder: &mut CommandEncoder,
        frame: &SwapChainOutput,
    ) {
        let mut render_pass = app.renderer.begin_render_pass(encoder, &frame);
        self.square_pipeline
            .as_mut()
            .unwrap()
            .draw(&mut render_pass);
        self.triangle_pipeline
            .as_mut()
            .unwrap()
            .draw(&mut render_pass);
    }

    fn on_event(&mut self, app: &mut Application, event: &Event) {
        match event {
            Event::KeyReleased(VirtualKeyCode::A) => log::debug!("A event"),
            Event::KeyReleased(VirtualKeyCode::Escape) => {
                log::debug!("Escape pressed");
                app.close();
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    configure_logging();

    let (mut app, mut layer_stack, mut event_loop) =
        Application::new("Sandbox", Some(PathBuf::from("imgui.ini")))?;

    layer_stack.push_layer(Rc::new(RefCell::new(ExampleLayer::new())));
    layer_stack.push_layer(Rc::new(RefCell::new(IcedUiLayer::new())));
    layer_stack.push_overlay(Rc::new(RefCell::new(DebugTextLayer::new())));

    app.run(&mut layer_stack, &mut event_loop)
}

fn configure_logging() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .filter_module("gfx_descriptor", log::LevelFilter::Warn)
        .filter_module("gfx_memory", log::LevelFilter::Warn)
        .filter_module("gfx_backend_vulkan", log::LevelFilter::Warn)
        .filter_module("iced_wgpu", log::LevelFilter::Warn)
        .init();

    log::trace!("Initialized logging");
}
