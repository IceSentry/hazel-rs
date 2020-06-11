use mint::Point2;
use winit::event::{MouseButton, VirtualKeyCode};

pub enum Event {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    MouseButtonPressed(MouseButton, Point2<f64>),
    MouseButtonReleased(MouseButton, Point2<f64>),
    WindowResize,
    ScaleFactorChanged,
}
