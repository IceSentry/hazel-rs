use hazel::{
    event::Event,
    input::VirtualKeyCode,
    layers::{debug_text::DebugTextLayer, iced_ui::IcedUiLayer, imgui::ImguiLayer, Layer},
    Application,
};

use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::path::Path;

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_update(&mut self, app: &mut Application) {
        if app.input_context.is_key_pressed(VirtualKeyCode::A) {
            log::debug!("A poll");
        }
    }

    fn on_event(&mut self, app: &mut Application, event: &Event) {
        match event {
            Event::KeyReleased(VirtualKeyCode::A) => log::debug!("A event"),
            Event::KeyReleased(VirtualKeyCode::Escape) => {
                log::debug!("Escape pressed");
                app.quit();
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    configure_logging(true)?;

    let (mut app, mut layer_stack, mut event_loop) = hazel::create_app("Sandbox")?;

    layer_stack.push_layer(Box::new(ExampleLayer {}));
    layer_stack.push_layer(Box::new(IcedUiLayer::new()));
    layer_stack.push_overlay(Box::new(DebugTextLayer::new()));
    layer_stack.push_overlay(Box::new(ImguiLayer::new()));

    app.run(&mut layer_stack, &mut event_loop)
}

fn configure_logging(use_env_logger: bool) -> anyhow::Result<(), SetLoggerError> {
    if use_env_logger {
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .filter_module("wgpu", log::LevelFilter::Warn)
            .filter_module("gfx_descriptor", log::LevelFilter::Warn)
            .filter_module("gfx_memory", log::LevelFilter::Warn)
            .filter_module("gfx_backend_vulkan", log::LevelFilter::Warn)
            .filter_module("iced_wgpu", log::LevelFilter::Warn)
            .init();
    } else {
        let level = log::LevelFilter::Trace;
        let file_path = Path::new("log/out.log");
        let pattern = "[{d(%Y-%m-%d %H:%M:%S)} {h({l:<5})} {t}] {m}{n}";

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .target(Target::Stdout)
            .build();

        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .append(false)
            .build(file_path)
            .expect("Failed to build log file logger");

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stdout", Box::new(stdout)),
            )
            .logger(Logger::builder().build("wgpu", LevelFilter::Warn))
            .logger(Logger::builder().build("wgpu_core", LevelFilter::Warn))
            .logger(Logger::builder().build("gfx_descriptor", LevelFilter::Warn))
            .logger(Logger::builder().build("gfx_memory", LevelFilter::Warn))
            .logger(Logger::builder().build("gfx_backend_vulkan", LevelFilter::Warn))
            .logger(Logger::builder().build("iced_wgpu", LevelFilter::Warn))
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Trace),
            )
            .expect("Failed to build logging config");

        let _handle = log4rs::init_config(config)?;
    }

    log::trace!("Initialized logging");

    Ok(())
}
