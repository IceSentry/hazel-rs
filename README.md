# hazel-rs

This is a WIP rust port of the Hazel engine

## Layers

The main concept of the engine is based on using layers and a layer stack. When creating an app you can add any layer that you want.

Overlays are layers that are added after layers on the stack.

### Lifecycles

The lifecycles events are called on each layers in the order they were added to the stack.

- `on_attach(&mut self, _app: &mut Application)`
  - called before application start
- `on_detach(&mut self, _app: &mut Application)`
  - called after application is closed
- `on_update(&mut self, _app: &mut Application)`
  - called before render
- `on_render(&mut self, _app: &mut Application, _encoder: &mut wgpu::CommandEncoder, _frame: &wgpu::SwapChainOutput,)`
  - This is called right after the clear screen and before calling queue.submit() on wgpu
- `on_event(&mut self, _app: &mut Application, _event: &Event<()>)`
  - This is called when the winit event_loop gets an event that isn't already handled by the engine (i.e. closing on X)

There are a few preconfigured layers to get you started:

- DebugTextLayer: Displays some basic debug info like frametime and fps
- ImguiLayer: Display anything related to imgui
- IcedLayer: used to create game UIs. It is intended as an example only.

## Example usage

```rust
use hazel_rs_lib::{
    layers::{debug_text::DebugTextLayer, iced_ui::IcedUiLayer, imgui::ImguiLayer, Layer},
    Application, Event,
};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_update(&mut self, _app: &mut Application) {
    }

    fn on_event(&mut self, _app: &mut Application, _event: &Event<()>) {
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (mut app, mut layer_stack, mut event_loop) = hazel_rs_lib::create_app("Sandbox", true)?;

    layer_stack.push_layer(Box::new(ExampleLayer {}));
    layer_stack.push_layer(Box::new(IcedUiLayer::new()));
    layer_stack.push_overlay(Box::new(DebugTextLayer::new()));
    layer_stack.push_overlay(Box::new(ImguiLayer::new()));

    app.run(&mut layer_stack, &mut event_loop)
}
```

## Logging

Currently the lib initializes a logging framework, this will be removed in the future
