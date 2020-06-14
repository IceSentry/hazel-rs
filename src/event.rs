use crate::Application;
use mint::Point2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode, WindowEvent};

pub enum Event {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    MouseButtonPressed(MouseButton, Point2<f64>),
    MouseButtonReleased(MouseButton, Point2<f64>),
    WindowResize,
    ScaleFactorChanged,
}

pub fn process_event(app: &mut Application, event: &winit::event::Event<()>) -> Option<Event> {
    app.input_context.update(&event);

    if let winit::event::Event::WindowEvent { ref event, .. } = event {
        match event {
            WindowEvent::Resized(physical_size) => {
                app.renderer.resize(*physical_size, None);
                return Some(Event::WindowResize);
            }
            WindowEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
                ..
            } => {
                app.renderer.resize(**new_inner_size, Some(*scale_factor));
                return Some(Event::ScaleFactorChanged);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    return match input.state {
                        ElementState::Pressed => Some(Event::KeyPressed(keycode)),
                        ElementState::Released => Some(Event::KeyReleased(keycode)),
                    };
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                return match state {
                    ElementState::Pressed => Some(Event::MouseButtonPressed(
                        *button,
                        app.input_context.mouse_position,
                    )),
                    ElementState::Released => Some(Event::MouseButtonReleased(
                        *button,
                        app.input_context.mouse_position,
                    )),
                };
            }
            _ => {}
        }
    }
    None
}
