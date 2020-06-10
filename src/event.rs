use winit::event::VirtualKeyCode;

pub enum Event {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    WindowResize,
    ScaleFactorChanged,
}
