use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

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

    pub fn desc<'a>(&self) -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float3, 1 => Float4],
        }
    }
}

pub struct IndexBuffer {
    pub buffer: wgpu::Buffer,
    pub count: u32,
    pub format: wgpu::IndexFormat,
}

impl IndexBuffer {
    pub fn create(device: &wgpu::Device, indices: &[u16]) -> Self {
        Self {
            buffer: device
                .create_buffer_with_data(bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX),
            count: indices.len() as u32,
            format: wgpu::IndexFormat::Uint16,
        }
    }
}
