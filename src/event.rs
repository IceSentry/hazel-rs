use winit::event::{MouseButton, VirtualKeyCode};

pub enum Event {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    MouseButtonPressed(MouseButton, f64, f64),
    MouseButtonReleased(MouseButton, f64, f64),
    WindowResize,
    ScaleFactorChanged,
}
