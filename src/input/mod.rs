use derive_new::new;
use std::collections::HashSet;
pub use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

#[derive(new)]
pub struct InputContext {
    #[new(value = "HashSet::new()")]
    keys_pressed: HashSet<VirtualKeyCode>,
    #[new(value = "None")]
    key_released: Option<VirtualKeyCode>,
}

impl InputContext {
    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    return match input.state {
                        ElementState::Pressed => {
                            self.set_key(keycode, true);
                        }
                        ElementState::Released => {
                            self.set_key(keycode, false);
                            self.key_released = Some(keycode);
                        }
                    };
                }
            }
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        }
    }

    pub fn set_key(&mut self, key: VirtualKeyCode, pressed: bool) {
        if pressed {
            self.keys_pressed.insert(key);
        } else {
            self.keys_pressed.remove(&key);
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
