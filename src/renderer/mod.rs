use crate::Frame;
use derive_new::new;
use renderer_api::RendererApi;

pub mod buffer;
pub mod orthographic_camera;
pub mod pipeline;
pub mod primitives;
pub mod renderer_api;
pub mod shader;

pub enum RenderCommand<'a> {
    Clear(&'a Frame),
    SetClearColor([f64; 4]),
    // DrawIndexed, // TODO make a pipeline trait?
}

#[derive(new)]
pub struct Renderer {
    pub api: RendererApi,
    #[new(value = "[0.0; 4]")]
    clear_color: [f64; 4],
}

impl Renderer {
    pub fn begin_scene(&mut self) {}

    pub fn end_scene(&mut self) {}

    pub fn send(&mut self, command: RenderCommand) {
        match command {
            RenderCommand::Clear(frame) => {
                self.api.clear(&frame, Some(self.clear_color));
            }
            RenderCommand::SetClearColor(color) => self.clear_color = color,
        }
    }
}
