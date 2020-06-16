pub mod debug_text;
pub mod iced_ui;
pub mod imgui;

use crate::Ui;
use crate::{event::Event, Application, Frame};

pub trait Layer {
    fn get_name(&self) -> String;
    /// Called before starting to poll events
    fn on_attach(&mut self, _app: &mut Application) {}
    /// Called before closing the application
    fn on_detach(&mut self, _app: &mut Application) {}
    /// Called once per frame
    fn on_update(&mut self, _app: &mut Application) {}
    fn on_render(&mut self, _app: &mut Application, _frame: &Frame) {}
    /// Called right before on_render for setting up things like imgui context
    fn on_before_render(&mut self, _app: &mut Application) {}
    /// Called on hazel events
    fn on_event(&mut self, _app: &mut Application, _event: &Event) {}
    /// Called before hazel handle events, this is only to support external integrations like iced_winit
    fn on_winit_event(&mut self, _app: &mut Application, _event: &winit::event::Event<()>) {}
    /// Called before on_render to setup custom imgui windows
    fn on_imgui_render(&mut self, _app: &mut Application, _ui: &Ui) {}
}

type LayerRef = Box<dyn Layer>;

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
        for layer in self.layers.iter_mut() {
            layer.on_attach(app);
            log::trace!("{} attached", layer.get_name());
        }
    }

    pub fn on_detach(&mut self, app: &mut Application) {
        for layer in self.layers.iter_mut() {
            layer.on_detach(app);
            log::trace!("{} detached", layer.get_name());
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

    /// This needs to be called before on_wgpu_render
    /// otherwise the imgui_layer won't have anything to render
    pub fn on_imgui_render(&mut self, app: &mut Application) {
        unsafe {
            if let Some(ui) = imgui::current_ui() {
                for layer in self.layers.iter_mut() {
                    layer.on_imgui_render(app, ui);
                }
            }
        }
    }

    pub fn on_before_render(&mut self, app: &mut Application) {
        for layer in self.layers.iter_mut() {
            layer.on_before_render(app);
        }
    }

    pub fn on_render(&mut self, app: &mut Application, frame: &wgpu::SwapChainOutput) {
        for layer in self.layers.iter_mut() {
            layer.on_render(app, frame);
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
fn eq<T: ?Sized>(left: &Box<T>, right: &Box<T>) -> bool {
    let left: *const T = left.as_ref();
    let right: *const T = right.as_ref();
    left == right
}
