use std::time::Duration;

use freya_animation::prelude::*;
use freya_core::prelude::Color;
use freya_sdk::timeout::*;

// Duration of the animation
const ANIMATION_TIME: Duration = Duration::from_millis(100);
// Delay between animations
const WAIT_ANIMATION_TIME: Duration = Duration::from_millis(750);
// Time until the animation is started since the last reset (key down, mouse click)
const ANIMATION_TIMEOUT: Duration = Duration::from_millis(500);

/// A hook that manages the cursor blink animation with a typing timeout.
/// When the user types, the cursor stays visible. After the timeout elapses,
/// the cursor starts blinking again.
pub fn use_cursor_blink(enable: bool, color: Color) -> (Timeout, Color) {
    let movement_timeout = use_timeout(|| ANIMATION_TIMEOUT);

    let cursor_blink = use_animation_with_dependencies(
        &(enable, movement_timeout.elapsed()),
        |conf, (enable, movement_elapsed)| {
            // Only animate it when focused and there has been no recent movement
            // If that's not the case, then it will be reset to the initial value,
            // 255, and thus show statically
            if *enable && *movement_elapsed {
                conf.on_creation(OnCreation::Run);
                conf.on_change(OnChange::Rerun);
                conf.on_finish(OnFinish::reverse_with_delay(WAIT_ANIMATION_TIME));
            }
            AnimNum::new(255., 0.).duration(ANIMATION_TIME)
        },
    );

    let cursor_color = color.with_a(cursor_blink.get().value() as u8);

    (movement_timeout, cursor_color)
}
