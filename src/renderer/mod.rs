pub mod buffer;
pub mod orthographic_camera;
pub mod pipeline;
pub mod primitives;
pub mod shader;

use anyhow::{anyhow, Context, Result};
use std::time::{Duration, Instant};
use winit::window::Window;

pub struct Renderer {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub last_frame: Instant,
    pub last_frame_duration: Duration,
    pub clear_color: wgpu::Color,
    pub device: wgpu::Device,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub queue: wgpu::Queue,
    pub pipeline_layout: wgpu::PipelineLayout,
    surface: wgpu::Surface,
    scale_factor: f64,
    swap_chain: wgpu::SwapChain,
    pub encoder: wgpu::CommandEncoder,
}

impl Renderer {
    pub async fn new(
        window: &Window,
        clear_color: wgpu::Color,
        v_sync: bool,
    ) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY, // Vulakn + Metal + DX12 + WebGPU
        )
        .await
        .context("Failed to request adapter")?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            // We use wgpu::TextureFormat::Bgra8UnormSrgb because that's the format
            // that's guaranteed to be natively supported by the swapchains of all the APIs/platforms
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: if v_sync {
                wgpu::PresentMode::Fifo
            } else {
                wgpu::PresentMode::Mailbox
            },
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
        });

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        Ok(Self {
            size,
            last_frame: Instant::now(),
            last_frame_duration: Instant::now().elapsed(),
            scale_factor: window.scale_factor(),
            clear_color,
            surface,
            device,
            sc_desc,
            swap_chain,
            queue,
            pipeline_layout,
            encoder,
        })
    }

    pub fn begin_render(&mut self) -> Result<wgpu::SwapChainOutput> {
        // this might potentially generate an infinite loop of dropped frame
        // maybe panic after a few dropped frame?
        let frame = self
            .swap_chain
            .get_next_texture()
            .map_err(|e| anyhow!("{:?}", e))
            .context("Dropped frame")?;

        Ok(frame)
    }

    pub fn end_render(&mut self) {
        let next_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        let encoder = std::mem::replace(&mut self.encoder, next_encoder);

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: Option<f64>) {
        if let Some(scale_factor) = scale_factor {
            self.scale_factor = scale_factor;
        }
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn set_v_sync(&mut self, enabled: bool) {
        self.sc_desc.present_mode = if enabled {
            wgpu::PresentMode::Fifo
        } else {
            wgpu::PresentMode::Mailbox
        };

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    /// If the clear_color is set to none it will use the default renderer clear_color
    pub fn clear(&mut self, frame: &wgpu::SwapChainOutput, clear_color: Option<[f64; 4]>) {
        let [r, g, b, a] = {
            match clear_color {
                Some(clear_color) => clear_color,
                None => [
                    self.clear_color.r,
                    self.clear_color.g,
                    self.clear_color.b,
                    self.clear_color.a,
                ],
            }
        };
        let _ = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color { r, g, b, a },
            }],
            depth_stencil_attachment: None,
        });
    }
}
