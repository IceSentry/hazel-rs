use super::Layer;
use crate::{Application, Event};
use derive_new::new;
use imgui::{im_str, Condition, FontSource};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use log::trace;

struct ImguiState {
    pub context: Box<imgui::Context>,
    pub platform: Box<WinitPlatform>,
    pub renderer: Box<imgui_wgpu::Renderer>,
}

#[derive(new)]
pub struct ImguiLayer {
    #[new(default)]
    state: Option<ImguiState>,
}

impl Layer for ImguiLayer {
    fn on_attach(&mut self, app: &mut Application) {
        self.state = Some(ImguiState::new(app));
        trace!("imgui-layer attached")
    }

    fn on_event(&mut self, app: &mut Application, event: &Event<()>) {
        let state = self.state.as_mut().unwrap();
        state
            .platform
            .handle_event(state.context.io_mut(), &app.window, &event);
    }

    fn on_render(
        &mut self,
        app: &mut Application,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SwapChainOutput,
    ) {
        let delta_t = app.delta_t;
        let state = self.state.as_mut().unwrap();
        state
            .platform
            .prepare_frame(state.context.io_mut(), &app.window)
            .expect("Failed to prepare frame");

        state.context.io_mut().delta_time = delta_t.as_secs_f32();
        let ui = state.context.frame();

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

        state.platform.prepare_render(&ui, &app.window);

        state
            .renderer
            .render(ui.render(), &app.renderer.device, encoder, &frame.view)
            .expect("imgui rendering failed");
    }
}

impl ImguiState {
    fn new(app: &mut Application) -> Self {
        let mut imgui = imgui::Context::create();
        imgui.style_mut().use_dark_colors();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &app.window, HiDpiMode::Default);
        imgui.set_ini_filename(None);

        let font_size = (13.0 * app.scale_factor) as f32;
        let io = imgui.io_mut();
        io.font_global_scale = (1.0 / app.scale_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer = imgui_wgpu::Renderer::new(
            &mut imgui,
            &app.renderer.device,
            &mut app.renderer.queue,
            app.renderer.sc_desc.format,
            Some(app.renderer.clear_color),
        );

        Self {
            context: Box::new(imgui),
            platform: Box::new(platform),
            renderer: Box::new(renderer),
        }
    }
}
