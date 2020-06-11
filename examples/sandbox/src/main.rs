use hazel::{
    event::Event,
    input::VirtualKeyCode,
    layers::{debug_text::DebugTextLayer, iced_ui::IcedUiLayer, Layer},
    Application,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn get_name(&self) -> String {
        String::from("example-layer")
    }

    fn on_update(&mut self, app: &mut Application) {
        if app.input_context.is_key_pressed(VirtualKeyCode::A) {
            log::debug!("A poll");
        }
    }

    fn on_event(&mut self, app: &mut Application, event: &Event) {
        match event {
            Event::KeyReleased(VirtualKeyCode::A) => log::debug!("A event"),
            Event::KeyReleased(VirtualKeyCode::Escape) => {
                log::debug!("Escape pressed");
                app.quit();
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    configure_logging();

    let (mut app, mut layer_stack, mut event_loop) =
        hazel::create_app("Sandbox", Some(PathBuf::from("imgui.ini")))?;

    layer_stack.push_layer(Rc::new(RefCell::new(ExampleLayer {})));
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
