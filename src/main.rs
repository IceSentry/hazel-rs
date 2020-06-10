use hazel_rs_lib::{layers::Layer, Application, Event, ImguiLayer};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_update(&self, _ctx: &mut Application) {
        // info!("ExampleLayer update");
    }

    fn on_event(&self, _ctx: &mut Application, _event: &Event<()>) {
        // trace!("{:?}", event);
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (mut app, mut layer_stack, mut event_loop) = hazel_rs_lib::create_app("Sandbox")?;
    let layer = Box::new(ExampleLayer {});
    layer_stack.push_layer(layer, &mut app);
    layer_stack.push_layer(Box::new(ImguiLayer {}), &mut app);
    hazel_rs_lib::run(&mut app, &mut layer_stack, &mut event_loop)
}
