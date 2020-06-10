use hazel_rs_lib::{layers::Layer, Application, Event, ImguiLayer};
use log::{info, trace};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_update(&self, _ctx: &mut Application) {
        // info!("ExampleLayer update");
    }

    fn on_event(&self, _ctx: &mut Application, event: &Event<()>) {
        // trace!("{:?}", event);
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (app, mut layer_stack, mut event_loop) = hazel_rs_lib::create_app("Sandbox")?;
    let layer = Box::new(ExampleLayer {});
    layer_stack.push_layer(layer, &mut app.borrow_mut());
    layer_stack.push_layer(Box::new(ImguiLayer {}), &mut app.borrow_mut());
    hazel_rs_lib::run(app, &mut layer_stack, &mut event_loop)
}
