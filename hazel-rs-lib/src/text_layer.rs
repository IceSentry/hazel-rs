// use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};

// let data = RenderData {
//     device: self.device,
//     encoder,
//     frame,
// };

// self.render_debug_text(&mut data, app);

// fn render_debug_text(&self, data: &mut RenderData, app: &mut Application) {
//     let font =
//         ab_glyph::FontArc::try_from_slice(include_bytes!("assets/Inconsolata-Regular.ttf"))
//             .expect("Load font");
//     let mut glyph_brush =
//         GlyphBrushBuilder::using_font(font).build(&self.device, self.render_format);

//     glyph_brush.queue(Section {
//         text: vec![Text::new(&format!("{:?}", app.delta_t))],
//         ..Section::default()
//     });

//     let curr_fps = 1.0 / app.delta_t.as_secs_f64();
//     let last_fps = 1.0 / self.last_frame_duration.as_secs_f64();

//     glyph_brush.queue(Section {
//         text: vec![Text::new(&format!(
//             "{:.0}fps",
//             last_fps * 0.9 + curr_fps * 0.1
//         ))],
//         screen_position: (0.0, 20.0),
//         ..Section::default()
//     });

//     glyph_brush
//         .draw_queued(
//             &data.device,
//             &mut data.encoder,
//             &data.frame.view,
//             self.size.width,
//             self.size.height,
//         )
//         .expect("Draw queued");
// }
