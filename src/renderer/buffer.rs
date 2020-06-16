use std::marker::PhantomData;

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
