use super::utils::Vertex;

pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
}

impl VertexBuffer {
    pub fn create(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        Self {
            buffer: device
                .create_buffer_with_data(bytemuck::cast_slice(vertices), wgpu::BufferUsage::VERTEX),
        }
    }
}

pub struct IndexBuffer {
    pub buffer: wgpu::Buffer,
    pub count: u32,
}

impl IndexBuffer {
    pub fn create(device: &wgpu::Device, indices: &[u16]) -> Self {
        Self {
            buffer: device
                .create_buffer_with_data(bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX),
            count: indices.len() as u32,
        }
    }
}
