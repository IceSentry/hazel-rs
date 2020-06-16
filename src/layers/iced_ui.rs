use super::Layer;
use crate::{renderer::RenderCommand, Application};
use derive_new::new;
use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};
use iced_winit::{
    program, slider, winit, Align, Color, Column, Command, Debug, Element, Length, Program, Row,
    Size, Slider, Text,
};
use winit::event::{Event, ModifiersState, WindowEvent};

struct LayerState {
    renderer: Renderer,
    state: program::State<Controls>,
    viewport: Viewport,
    debug: Debug,
}

#[derive(new)]
pub struct IcedUiLayer {
    #[new(default)]
    state: Option<LayerState>,
}

impl Layer for IcedUiLayer {
    fn get_name(&self) -> String {
        String::from("iced-ui-layer")
    }

    fn on_attach(&mut self, app: &mut Application) {
        let physical_size = app.window.inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            app.window.scale_factor(),
        );

        let clear_color = Color {
            r: app.renderer.api.clear_color.r as f32,
            g: app.renderer.api.clear_color.g as f32,
            b: app.renderer.api.clear_color.b as f32,
            a: app.renderer.api.clear_color.a as f32,
        };
        let controls = Controls::new(clear_color);

        let mut debug = Debug::new();
        let mut renderer =
            Renderer::new(Backend::new(&app.renderer.api.device, Settings::default()));

        let state =
            program::State::new(controls, viewport.logical_size(), &mut renderer, &mut debug);

        self.state = Some(LayerState {
            viewport,
            debug,
            renderer,
            state,
        });
    }

    fn on_update(&mut self, app: &mut Application) {
        if let Some(LayerState {
            state,
            viewport,
            renderer,
            debug,
        }) = self.state.as_mut()
        {
            if !state.is_queue_empty() {
                state.update(None, viewport.logical_size(), renderer, debug);
                let program = state.program();
                let [r, g, b, a] = program.background_color.into_linear();
                app.renderer.send(RenderCommand::SetClearColor([
                    r as f64, g as f64, b as f64, a as f64,
                ]));
            }
        }
    }

    fn on_render(&mut self, app: &mut Application, frame: &wgpu::SwapChainOutput) {
        let layer_state = match self.state.as_mut() {
            Some(it) => it,
            _ => return,
        };
        let mouse_interaction = layer_state.renderer.backend_mut().draw(
            &app.renderer.api.device,
            &mut app.renderer.api.encoder,
            &frame.view,
            &layer_state.viewport,
            layer_state.state.primitive(),
            &layer_state.debug.overlay(),
        );
        app.window
            .set_cursor_icon(iced_winit::conversion::mouse_interaction(mouse_interaction));
    }

    fn on_winit_event(&mut self, app: &mut Application, event: &Event<()>) {
        let layer_state = match self.state.as_mut() {
            Some(it) => it,
            _ => return,
        };
        let mut modifiers = ModifiersState::default();
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::ModifiersChanged(new_modifiers) => {
                    modifiers = *new_modifiers;
                }
                WindowEvent::Resized(new_size) => {
                    layer_state.viewport = Viewport::with_physical_size(
                        Size::new(new_size.width, new_size.height),
                        app.window.scale_factor(),
                    );
                }
                _ => {}
            };

            if let Some(event) =
                iced_winit::conversion::window_event(&event, app.window.scale_factor(), modifiers)
            {
                layer_state.state.queue_event(event);
            }
        }
    }
}

#[derive(Default)]
pub struct Controls {
    background_color: Color,
    sliders: [slider::State; 3],
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
}

impl Controls {
    pub fn new(background_color: Color) -> Controls {
        Controls {
            background_color,
            sliders: Default::default(),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Program for Controls {
    type Renderer = iced_wgpu::Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message, iced_wgpu::Renderer> {
        let [r, g, b] = &mut self.sliders;
        let background_color = self.background_color;

        let sliders = Row::new()
            .width(Length::Units(500))
            .spacing(20)
            .push(Slider::new(r, 0.0..=1.0, background_color.r, move |r| {
                Message::BackgroundColorChanged(Color {
                    r,
                    ..background_color
                })
            }))
            .push(Slider::new(g, 0.0..=1.0, background_color.g, move |g| {
                Message::BackgroundColorChanged(Color {
                    g,
                    ..background_color
                })
            }))
            .push(Slider::new(b, 0.0..=1.0, background_color.b, move |b| {
                Message::BackgroundColorChanged(Color {
                    b,
                    ..background_color
                })
            }));

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::End)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::End)
                    .push(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(Text::new("Background color").color(Color::WHITE))
                            .push(sliders)
                            .push(
                                Text::new(format!("{:?}", background_color))
                                    .size(14)
                                    .color(Color::WHITE),
                            ),
                    ),
            )
            .into()
    }
}
