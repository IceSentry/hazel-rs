pub mod debug_text;
pub mod iced_ui;
pub mod imgui;

use crate::{event::Event, Application};
use std::{cell::RefCell, rc::Rc};

pub trait Layer {
    fn get_name(&self) -> String;
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

type LayerRef = Rc<RefCell<dyn Layer>>;

#[derive(Default)]
pub struct LayerStack {
    pub layers: Vec<LayerRef>,
    layer_insert: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            layer_insert: 0,
        }
    }

    pub fn push_layer(&mut self, layer: LayerRef) {
        self.layers.insert(self.layer_insert, layer);
        self.layer_insert += 1;
    }

    pub fn push_overlay(&mut self, layer: LayerRef) {
        self.layers.push(layer);
    }

    pub fn on_attach(&mut self, app: &mut Application) {
        for layer in self.layers.iter() {
            layer.borrow_mut().on_attach(app);
        }
    }

    pub fn on_detach(&mut self, app: &mut Application) {
        for layer in self.layers.iter() {
            layer.borrow_mut().on_detach(app);
        }
    }

    pub fn on_update(&mut self, app: &mut Application) {
        for layer in self.layers.iter() {
            layer.borrow_mut().on_update(app);
        }
    }

    pub fn on_event(&mut self, app: &mut Application, event: &Event) {
        for layer in self.layers.iter() {
            layer.borrow_mut().on_event(app, event);
        }
    }

    pub fn on_winit_event(&mut self, app: &mut Application, event: &winit::event::Event<()>) {
        for layer in self.layers.iter() {
            layer.borrow_mut().on_winit_event(app, event);
        }
    }

    pub fn on_render(
        &mut self,
        app: &mut Application,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SwapChainOutput,
    ) {
        for layer in self.layers.iter() {
            layer.borrow_mut().on_render(app, encoder, frame);
        }
    }

    #[allow(dead_code)]
    #[allow(clippy::borrowed_box)]
    fn pop_layer(&mut self, layer: &LayerRef) {
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
    fn pop_overlay(&mut self, layer: &LayerRef) {
        let index = self
            .layers
            .iter()
            .position(|l| eq(l, layer))
            .expect("Overlay was not found");
        self.layers.remove(index);
    }
}

#[allow(dead_code)]
#[allow(clippy::borrowed_box)]
fn eq<T: ?Sized>(left: &Rc<RefCell<T>>, right: &Rc<RefCell<T>>) -> bool {
    let left: *const T = left.as_ptr();
    let right: *const T = right.as_ptr();
    left == right
}
