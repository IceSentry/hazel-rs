use derive_new::new;
use std::collections::HashSet;
pub use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

#[derive(new)]
pub struct InputContext {
    #[new(value = "HashSet::new()")]
    keys_pressed: HashSet<VirtualKeyCode>,
    #[new(value = "None")]
    key_released: Option<VirtualKeyCode>,

    #[new(value = "HashSet::new()")]
    mouse_pressed: HashSet<MouseButton>,
    #[new(value = "(0.0, 0.0)")]
    pub mouse_position: (f64, f64),
}

impl InputContext {
    pub fn update(&mut self, event: &winit::event::Event<()>) {
        #[allow(clippy::single_match)]
        match event {
            winit::event::Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                self.keys_pressed.insert(keycode);
                            }
                            ElementState::Released => {
                                self.keys_pressed.remove(&keycode);
                                self.key_released = Some(keycode);
                            }
                        };
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    match state {
                        ElementState::Pressed => self.mouse_pressed.insert(*button),
                        ElementState::Released => self.mouse_pressed.remove(button),
                    };
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_position.0 = position.x;
                    self.mouse_position.1 = position.y;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: VirtualKeyCode) -> bool {
        if let Some(key_released) = self.key_released {
            if key_released == key {
                return true;
            }
        }
        false
    }
}
