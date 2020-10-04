use crate::Frame;
use buffer::VertexBufferLayout;
use derive_new::new;
use primitives::VertexArray;
use renderer_api::RendererApi;
use shader::Shader;

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

    pub fn submit<T>(&mut self, shader: &Shader, vertex_array: &VertexArray<T>, frame: &Frame)
    where
        T: VertexBufferLayout + bytemuck::Pod + bytemuck::Zeroable,
    {
        // FIXME I should probably cache this somehow
        let pipeline = shader.create_pipeline(&self.api, vertex_array, 1);

        let mut render_pass = self
            .api
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                }],
                depth_stencil_attachment: None,
            });
        render_pass.set_pipeline(&pipeline);
        // TODO uniforms
        render_pass.set_vertex_buffer(0, &vertex_array.vertex_buffer.buffer, 0, 0);
        render_pass.set_index_buffer(&vertex_array.index_buffer.buffer, 0, 0);
        render_pass.draw_indexed(0..vertex_array.index_buffer.count, 0, 0..1);
    }
}
