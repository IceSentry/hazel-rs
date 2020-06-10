pub mod debug_text;
pub mod iced_ui;
pub mod imgui;

use crate::{event::Event, Application};

pub trait Layer {
    fn on_attach(&mut self, _app: &mut Application) {}
    fn on_detach(&mut self, _app: &mut Application) {}
    fn on_update(&mut self, _app: &mut Application) {}
    fn on_render(
        &mut self,
        _app: &mut Application,
        _encoder: &mut wgpu::CommandEncoder,
        _frame: &wgpu::SwapChainOutput,
    ) {
    }
    fn on_event(&mut self, _app: &mut Application, _event: &Event) {}
    fn on_winit_event(&mut self, _app: &mut Application, _event: &winit::event::Event<()>) {}
}

#[allow(dead_code)]
#[allow(clippy::borrowed_box)]
fn eq<T: ?Sized>(left: &Box<T>, right: &Box<T>) -> bool {
    let left: *const T = left.as_ref();
    let right: *const T = right.as_ref();
    left == right
}

#[derive(Default)]
pub struct LayerStack {
    pub layers: Vec<Box<dyn Layer>>,
    layer_insert: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            layer_insert: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.layer_insert, layer);
        self.layer_insert += 1;
    }

    pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn on_attach(&mut self, app: &mut Application) {
        for layer in self.layers.iter_mut() {
            layer.on_attach(app);
        }
    }

    pub fn on_detach(&mut self, app: &mut Application) {
        for layer in self.layers.iter_mut() {
            layer.on_detach(app);
        }
    }

    pub fn on_update(&mut self, app: &mut Application) {
        for layer in self.layers.iter_mut() {
            layer.on_update(app);
        }
    }

    pub fn on_event(&mut self, app: &mut Application, event: &Event) {
        for layer in self.layers.iter_mut() {
            layer.on_event(app, event);
        }
    }

    pub fn on_winit_event(&mut self, app: &mut Application, event: &winit::event::Event<()>) {
        for layer in self.layers.iter_mut() {
            layer.on_winit_event(app, event);
        }
    }

    #[allow(dead_code)]
    #[allow(clippy::borrowed_box)]
    fn pop_layer(&mut self, layer: &Box<dyn Layer>) {
        let index = self
            .layers
            .iter()
            .position(|l| eq(l, layer))
            .expect("Layer was not found");
        self.layers.remove(index);
        self.layer_insert -= 1;
    }

    #[allow(dead_code)]
    #[allow(clippy::borrowed_box)]
    fn pop_overlay(&mut self, layer: &Box<dyn Layer>) {
        let index = self
            .layers
            .iter()
            .position(|l| eq(l, layer))
            .expect("Overlay was not found");
        self.layers.remove(index);
    }
}
