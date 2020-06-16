use super::buffer::{IndexBuffer, VertexBuffer, VertexBufferLayout};
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

pub struct VertexArray<T> {
    pub vertex_buffer: VertexBuffer<T>,
    pub index_buffer: IndexBuffer,
}

impl<T> VertexArray<T>
where
    T: VertexBufferLayout + bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn create(device: &wgpu::Device, vertices: &[T], indices: &[u16]) -> Self {
        Self {
            vertex_buffer: VertexBuffer::create(device, vertices),
            index_buffer: IndexBuffer::create(device, indices),
        }
    }
}
