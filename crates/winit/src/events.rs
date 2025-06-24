use freya_elements::{
    Force,
    MouseButton,
    TouchPhase,
};

pub fn map_winit_touch_force(event: winit::event::Force) -> Force {
    match event {
        winit::event::Force::Calibrated {
            force,
            max_possible_force,
            altitude_angle,
        } => Force::Calibrated {
            force,
            max_possible_force,
            altitude_angle,
        },
        winit::event::Force::Normalized(f) => Force::Normalized(f),
    }
}

pub fn map_winit_mouse_button(event: winit::event::MouseButton) -> MouseButton {
    match event {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
        winit::event::MouseButton::Other(o) => MouseButton::Other(o),
    }
}

pub fn map_winit_touch_phase(event: winit::event::TouchPhase) -> TouchPhase {
    match event {
        winit::event::TouchPhase::Started => TouchPhase::Started,
        winit::event::TouchPhase::Moved => TouchPhase::Moved,
        winit::event::TouchPhase::Ended => TouchPhase::Ended,
        winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
    }
}
