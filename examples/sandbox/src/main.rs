use hazel::{
    event::Event,
    input::VirtualKeyCode,
    layers::{debug_text::DebugTextLayer, iced_ui::IcedUiLayer, imgui::ImguiLayer, Layer},
    Application,
};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_update(&mut self, app: &mut Application) {
        if app.input_context.is_key_pressed(VirtualKeyCode::A) {
            log::trace!("A poll");
        }
    }

    fn on_event(&mut self, app: &mut Application, event: &Event) {
        match event {
            Event::KeyReleased(VirtualKeyCode::A) => log::trace!("A event"),
            Event::KeyReleased(VirtualKeyCode::Escape) => {
                log::trace!("Escape pressed");
                app.quit();
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (mut app, mut layer_stack, mut event_loop) = hazel::create_app("Sandbox", true)?;

    layer_stack.push_layer(Box::new(ExampleLayer {}));
    layer_stack.push_layer(Box::new(IcedUiLayer::new()));
    layer_stack.push_overlay(Box::new(DebugTextLayer::new()));
    layer_stack.push_overlay(Box::new(ImguiLayer::new()));

    app.run(&mut layer_stack, &mut event_loop)
}
