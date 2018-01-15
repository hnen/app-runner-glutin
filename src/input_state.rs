use std::collections::HashSet;

pub struct InputState {
    pub buttons_down : HashSet<u32>,
    pub buttons_down_last_frame : HashSet<u32>,
}

impl InputState {
    pub fn button_pressed(&self, scancode : u32) -> bool {
        self.buttons_down.contains(&scancode) && !self.buttons_down_last_frame.contains(&scancode)
    }
}

