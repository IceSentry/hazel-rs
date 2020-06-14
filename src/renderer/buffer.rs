use std::marker::PhantomData;
use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexPos {
    pub position: [f32; 3],
}
unsafe impl bytemuck::Pod for VertexPos {}
unsafe impl bytemuck::Zeroable for VertexPos {}

impl VertexBufferLayout for VertexPos {
    fn descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float3],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl VertexBufferLayout for Vertex {
    fn descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float3, 1 => Float4],
        }
    }
}

pub trait VertexBufferLayout {
    fn descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}

pub struct VertexBuffer<T> {
    pub buffer: wgpu::Buffer,
    layout: PhantomData<T>,
}

impl<T> VertexBuffer<T>
where
    T: VertexBufferLayout + bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn create(device: &wgpu::Device, vertices: &[T]) -> Self {
        Self {
            buffer: device
                .create_buffer_with_data(bytemuck::cast_slice(vertices), wgpu::BufferUsage::VERTEX),
            layout: PhantomData,
        }
    }

    pub fn descriptor<'a>(&self) -> wgpu::VertexBufferDescriptor<'a> {
        T::descriptor()
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
