use crate::{layers::LayerStack, Application};
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
    pub render_format: wgpu::TextureFormat,
    surface: wgpu::Surface,
    scale_factor: f64,
    swap_chain: wgpu::SwapChain,
}

impl Renderer {
    pub async fn new(window: &Window, v_sync: bool) -> Self {
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
        .expect("Failed to request adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;

        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            // We use wgpu::TextureFormat::Bgra8UnormSrgb because that's the format
            // that's guaranteed to be natively supported by the swapchains of all the APIs/platforms
            format: render_format,
            width: size.width,
            height: size.height,
            present_mode: if v_sync {
                wgpu::PresentMode::Fifo
            } else {
                wgpu::PresentMode::Mailbox
            },
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            size,
            last_frame: Instant::now(),
            last_frame_duration: Instant::now().elapsed(),
            scale_factor: 1.0,
            clear_color: wgpu::Color {
                r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            surface,
            device,
            sc_desc,
            swap_chain,
            render_format,
            queue,
        }
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

    pub fn toggle_v_sync(&mut self, enabled: bool) {
        self.sc_desc.present_mode = if enabled {
            wgpu::PresentMode::Fifo
        } else {
            wgpu::PresentMode::Mailbox
        };

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

pub fn render(app: &mut Application, layer_stack: &mut LayerStack) {
    let frame = match app.renderer.swap_chain.get_next_texture() {
        Ok(frame) => frame,
        Err(e) => {
            log::error!("dropped frame: {:?}", e);
            return;
        }
    };

    let mut encoder = app
        .renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    clear(&frame.view, &mut encoder, app.renderer.clear_color);

    layer_stack.on_imgui_render(app);
    layer_stack.on_wgpu_render(app, &mut encoder, &frame);

    app.renderer.queue.submit(&[encoder.finish()]);
}

pub fn clear<'a>(
    target: &'a wgpu::TextureView,
    encoder: &'a mut wgpu::CommandEncoder,
    clear_color: wgpu::Color,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: target,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color,
        }],
        depth_stencil_attachment: None,
    })
}
