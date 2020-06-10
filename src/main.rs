use hazel_rs_lib::{
    layers::{debug_text::DebugTextLayer, imgui::ImguiLayer, Layer},
    Application, Event,
};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_update(&mut self, _ctx: &mut Application) {
        // info!("ExampleLayer update");
    }

    fn on_event(&mut self, _ctx: &mut Application, _event: &Event<()>) {
        // trace!("{:?}", event);
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (mut app, mut layer_stack, mut event_loop) = hazel_rs_lib::create_app("Sandbox")?;

    layer_stack.push_layer(Box::new(ExampleLayer {}));
    layer_stack.push_layer(Box::new(ImguiLayer {}));
    layer_stack.push_layer(Box::new(DebugTextLayer::new()));

    app.run(&mut layer_stack, &mut event_loop)
}
