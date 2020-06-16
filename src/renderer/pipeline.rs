use super::{buffer::VertexBufferLayout, primitives::VertexArray, shader::Shader, Renderer};
use crate::Application;

pub struct Pipeline<T> {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_array: VertexArray<T>,
}

impl<T> Pipeline<T>
where
    T: VertexBufferLayout + bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new(renderer: &Renderer, shader: &Shader, vertex_array: VertexArray<T>) -> Self {
        Self {
            render_pipeline: shader.create_pipeline(renderer, &vertex_array, 1),
            vertex_array,
        }
    }

    pub fn draw(&self, app: &mut Application, frame: &wgpu::SwapChainOutput) {
        let mut render_pass = app
            .renderer
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

        render_pass.set_pipeline(&self.render_pipeline);
        // TODO
        // render_pass.set_bind_group(0, bind_group, offsets)
        render_pass.set_vertex_buffer(0, &self.vertex_array.vertex_buffer.buffer, 0, 0);
        render_pass.set_index_buffer(&self.vertex_array.index_buffer.buffer, 0, 0);
        render_pass.draw_indexed(0..self.vertex_array.index_buffer.count, 0, 0..1);
    }
}
