use super::{buffer::VertexBufferLayout, primitives::VertexArray, shader::Shader, Renderer};

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

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        // TODO
        // render_pass.set_bind_group(0, bind_group, offsets)
        render_pass.set_vertex_buffer(0, &self.vertex_array.vertex_buffer.buffer, 0, 0);
        render_pass.set_index_buffer(&self.vertex_array.index_buffer.buffer, 0, 0);
        render_pass.draw_indexed(0..self.vertex_array.index_buffer.count, 0, 0..1);
    }
}
