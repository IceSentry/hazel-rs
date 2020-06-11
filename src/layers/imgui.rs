use super::Layer;
use crate::Application;
use derive_new::new;
use imgui::{im_str, Condition, FontSource, Ui};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::PathBuf;

#[derive(new)]
pub struct ImguiLayer {
    #[new(default)]
    state: Option<ImguiState>,
    #[new(value = "false")]
    show_demo_window: bool,
    ini_path: Option<PathBuf>,
}

impl Layer for ImguiLayer {
    fn get_name(&self) -> String {
        String::from("imgui-layer")
    }

    fn on_attach(&mut self, app: &mut Application) {
        self.state = Some(ImguiState::new(app, self.ini_path.clone()));
        log::trace!("imgui-layer attached");
    }

    fn on_winit_event(&mut self, app: &mut Application, event: &winit::event::Event<()>) {
        if let Some(ImguiState {
            platform, context, ..
        }) = self.state.as_mut()
        {
            platform.handle_event(context.io_mut(), &app.window, &event);
        }
    }

    fn on_update(&mut self, app: &mut Application) {
        let delta_t = app.delta_t;
        if let Some(ImguiState {
            platform, context, ..
        }) = self.state.as_mut()
        {
            platform
                .prepare_frame(context.io_mut(), &app.window)
                .expect("Failed to prepare frame");

            context.io_mut().delta_time = delta_t.as_secs_f32();
        }
    }

    fn on_render(
        &mut self,
        app: &mut Application,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SwapChainOutput,
    ) {
        let delta_t = app.delta_t;
        if let Some(ImguiState {
            platform,
            context,
            renderer,
        }) = self.state.as_mut()
        {
            let ui = context.frame();

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

            ui.show_demo_window(&mut self.show_demo_window);

            platform.prepare_render(&ui, &app.window);
            renderer
                .render(ui.render(), &app.renderer.device, encoder, &frame.view)
                .expect("imgui rendering failed");
        }
    }
}

struct ImguiState {
    context: Box<imgui::Context>,
    platform: Box<WinitPlatform>,
    renderer: Box<imgui_wgpu::Renderer>,
}

impl ImguiState {
    fn new(app: &mut Application, ini_path: Option<PathBuf>) -> Self {
        let mut imgui = imgui::Context::create();
        imgui.style_mut().use_dark_colors();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &app.window, HiDpiMode::Default);
        imgui.set_ini_filename(ini_path);

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
            None,
        );

        Self {
            context: Box::new(imgui),
            platform: Box::new(platform),
            renderer: Box::new(renderer),
        }
    }
}
