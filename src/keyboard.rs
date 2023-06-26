use pixel_canvas::canvas::CanvasInfo;
use pixel_canvas::input::glutin::event::{ElementState, VirtualKeyCode};
use pixel_canvas::input::{Event, WindowEvent};

pub struct KeyboardState {
    pub scancode: u32,
    pub state: ElementState,
    pub virtual_key_code: VirtualKeyCode,
}

impl KeyboardState {
    /// Create a KeyboardState. For use with the `state` method.
    pub fn new() -> Self {
        Self {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_key_code: VirtualKeyCode::Key0,
        }
    }

    /// Handle input for the keyboard. For use with the `input` method.
    pub fn handle_input(
        _info: &CanvasInfo,
        keyboard: &mut KeyboardState,
        event: &Event<()>,
    ) -> bool {
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                keyboard.scancode = input.scancode;
                keyboard.state = input.state;
                match input.virtual_keycode {
                    Some(code) => keyboard.virtual_key_code = code,
                    _ => (),
                }
                true
            }
            _ => false,
        }
    }

    pub fn key_pressed(&self) -> Option<VirtualKeyCode> {
        if self.state == ElementState::Pressed {
            return Some(self.virtual_key_code);
        }
        return None;
    }
}
