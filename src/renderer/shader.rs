use super::{buffer::VertexBufferLayout, primitives::VertexArray, renderer_api::RendererApi};
use anyhow::{Context, Result};

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

    pub fn create_pipeline<T>(
        &self,
        renderer: &RendererApi,
        vertex_array: &VertexArray<T>,
        samples: u32,
    ) -> wgpu::RenderPipeline
    where
        T: VertexBufferLayout + bytemuck::Pod + bytemuck::Zeroable,
    {
        let vs_module = renderer.device.create_shader_module(&self.vertex_data);
        let fs_module = renderer.device.create_shader_module(&self.fragment_data);

        renderer
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &renderer.pipeline_layout,
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
                    format: renderer.sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: vertex_array.index_buffer.format,
                    vertex_buffers: &[vertex_array.vertex_buffer.descriptor()],
                },
                sample_count: samples,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            })
    }
}
