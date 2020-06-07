use imgui::{im_str, Condition, Context, FontSource, Ui};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::{Duration, Instant};
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};
use winit::window::Window;

pub struct ImguiState {
    pub context: Context,
    pub platform: WinitPlatform,
}

impl ImguiState {
    pub fn new(window: &winit::window::Window, scale_factor: f64) -> Self {
        let mut imgui = Context::create();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);
        imgui.set_ini_filename(None);

        let font_size = (13.0 * scale_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / scale_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        Self {
            context: imgui,
            platform,
        }
    }

    pub fn prepare(&mut self, window: &winit::window::Window, delta_t: Duration) -> Ui {
        self.platform
            .prepare_frame(self.context.io_mut(), &window)
            .expect("Failed to prepare frame");
        let ui = self.context.frame();

        {
            imgui::Window::new(im_str!("Debug info"))
                .position([0.0, 0.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Frametime: {:?}", delta_t));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(im_str!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0],
                        mouse_pos[1]
                    ));
                });
        }

        self.platform.prepare_render(&ui, &window);
        ui
    }
}

pub struct Renderer {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub last_frame: Instant,
    pub last_frame_duration: Duration,
    scale_factor: f64,
    clear_color: wgpu::Color,
    surface: wgpu::Surface,
    device: wgpu::Device,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    imgui_renderer: imgui_wgpu::Renderer,
    render_format: wgpu::TextureFormat,
    queue: wgpu::Queue,
}

impl Renderer {
    pub async fn new(window: &Window, imgui_context: &mut imgui::Context) -> Self {
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

        let (device, mut queue) = adapter
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
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let imgui_renderer =
            imgui_wgpu::Renderer::new(imgui_context, &device, &mut queue, sc_desc.format, None);

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
            imgui_renderer,
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

    pub fn render(&mut self, ui: imgui::Ui, delta_t: Duration) {
        let frame = match self.swap_chain.get_next_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("dropped frame: {:?}", e);
                return;
            }
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: self.clear_color,
                }],
                depth_stencil_attachment: None,
            });
        }

        self.render_debug_text(&mut encoder, &frame, delta_t);

        self.imgui_renderer
            .render(ui.render(), &self.device, &mut encoder, &frame.view)
            .expect("Imgui rendering failed");

        self.queue.submit(&[encoder.finish()]);
    }

    fn render_debug_text(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SwapChainOutput,
        delta_t: Duration,
    ) {
        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("assets/Inconsolata-Regular.ttf"))
                .expect("Load font");
        let mut glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&self.device, self.render_format);

        glyph_brush.queue(Section {
            text: vec![Text::new(&format!("{:?}", delta_t))],
            ..Section::default()
        });

        let curr_fps = 1.0 / delta_t.as_secs_f64();
        let last_fps = 1.0 / self.last_frame_duration.as_secs_f64();

        glyph_brush.queue(Section {
            text: vec![Text::new(&format!(
                "{:.0}fps",
                last_fps * 0.9 + curr_fps * 0.1
            ))],
            screen_position: (0.0, 20.0),
            ..Section::default()
        });

        glyph_brush
            .draw_queued(
                &self.device,
                encoder,
                &frame.view,
                self.size.width,
                self.size.height,
            )
            .expect("Draw queued");
    }
}
