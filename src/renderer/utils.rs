use super::buffer::{IndexBuffer, VertexBuffer};
use anyhow::{Context, Result};
use wgpu::vertex_attr_array;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn buffer_desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float3, 1 => Float3],
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
}

#[derive(Clone)]
pub struct Shader {
    vertex_data: Vec<u32>,
    fragment_data: Vec<u32>,
}

impl Shader {
    pub fn compile(vertex_src: String, fragment_src: String) -> Result<Self> {
        let mut compiler = shaderc::Compiler::new().expect("Failed to initialize shaderc compiler");
        let vs_spirv = compiler
            .compile_into_spirv(
                &vertex_src,
                shaderc::ShaderKind::Vertex,
                "shader.vert",
                "main",
                None,
            )
            .context("Failed to compile vert shader")?;
        let fs_spirv = compiler
            .compile_into_spirv(
                &fragment_src,
                shaderc::ShaderKind::Fragment,
                "shader.frag",
                "main",
                None,
            )
            .context("Failed to compile frag shader")?;

        let vertex_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8()))
            .context("Failed to read vertex shader spirv")?;
        let fragment_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8()))
            .context("Failed to read fragment shader spirv")?;

        Ok(Self {
            vertex_data,
            fragment_data,
        })
    }

    pub fn create_pipeline(
        &self,
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        pipeline_layout: &wgpu::PipelineLayout,
        samples: u32,
    ) -> wgpu::RenderPipeline {
        let vs_module = device.create_shader_module(&self.vertex_data);
        let fs_module = device.create_shader_module(&self.fragment_data);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::buffer_desc()],
            },
            sample_count: samples,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }
}