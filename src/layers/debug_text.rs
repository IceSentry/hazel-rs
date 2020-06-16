use super::Layer;
use crate::Application;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

#[derive(Default)]
pub struct DebugTextLayer {
    glyph_brush: Option<GlyphBrush<()>>,
}

impl DebugTextLayer {
    pub fn new() -> Self {
        Self { glyph_brush: None }
    }
}

impl Layer for DebugTextLayer {
    fn get_name(&self) -> String {
        String::from("debug-text-layer")
    }

    fn on_attach(&mut self, app: &mut Application) {
        let font = match ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../assets/Inconsolata-Regular.ttf"
        )) {
            Ok(font) => font,
            Err(_) => {
                log::error!("Font failed to load");
                return;
            }
        };

        self.glyph_brush = Some(
            GlyphBrushBuilder::using_font(font)
                .build(&app.renderer.api.device, app.renderer.api.sc_desc.format),
        );
    }

    fn on_render(&mut self, app: &mut Application, frame: &wgpu::SwapChainOutput) {
        let glyph_brush = self.glyph_brush.as_mut().unwrap();
        glyph_brush.queue(Section {
            text: vec![Text::new(&format!("Hello world from: {:?}", app.name))],
            ..Section::default()
        });

        if glyph_brush
            .draw_queued(
                &app.renderer.api.device,
                &mut app.renderer.api.encoder,
                &frame.view,
                app.renderer.api.size.width,
                app.renderer.api.size.height,
            )
            .is_err()
        {
            log::error!("Failed to draw debug_text");
        }
    }
}
