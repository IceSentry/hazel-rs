use crate::Application;
use winit::event::Event;

pub trait Layer {
    fn on_attach(&self, _app: &mut Application) {}
    fn on_detach(&self, _app: &mut Application) {}
    fn on_update(&self, _app: &mut Application) {}
    fn on_render(
        &self,
        _app: &mut Application,
        _encoder: &mut wgpu::CommandEncoder,
        _frame: &wgpu::SwapChainOutput,
    ) {
    }
    fn on_event(&self, _app: &mut Application, _event: &Event<()>) {}
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

    pub fn push_layer(&mut self, layer: Box<dyn Layer>, app: &mut Application) {
        layer.on_attach(app);
        self.layers.insert(self.layer_insert, layer);
        self.layer_insert += 1;
    }

    pub fn push_overlay(&mut self, layer: Box<dyn Layer>, app: &mut Application) {
        layer.on_attach(app);
        self.layers.push(layer);
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
