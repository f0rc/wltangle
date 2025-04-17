use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum WindowPosition {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Fullscreen,
    Center,
}

pub struct WindowManager {
    window_states: HashMap<String, WindowPosition>,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            window_states: HashMap::new(),
        }
    }

    pub fn calculate_geometry(&self, position: WindowPosition, 
                             screen_width: i32, screen_height: i32) -> (i32, i32, i32, i32) {
        match position {
            WindowPosition::Left => (0, 0, screen_width / 2, screen_height),
            WindowPosition::Right => (screen_width / 2, 0, screen_width / 2, screen_height),
            WindowPosition::Top => (0, 0, screen_width, screen_height / 2),
            WindowPosition::Bottom => (0, screen_height / 2, screen_width, screen_height / 2),
            WindowPosition::TopLeft => (0, 0, screen_width / 2, screen_height / 2),
            WindowPosition::TopRight => (screen_width / 2, 0, screen_width / 2, screen_height / 2),
            WindowPosition::BottomLeft => (0, screen_height / 2, screen_width / 2, screen_height / 2),
            WindowPosition::BottomRight => (screen_width / 2, screen_height / 2, screen_width / 2, screen_height / 2),
            WindowPosition::Fullscreen => (0, 0, screen_width, screen_height),
            WindowPosition::Center => {
                let width = screen_width * 2 / 3;
                let height = screen_height * 2 / 3;
                let x = (screen_width - width) / 2;
                let y = (screen_height - height) / 2;
                (x, y, width, height)
            }
        }
    }

    pub fn position_window(&mut self, window_id: &str, position: WindowPosition) {
        self.window_states.insert(window_id.to_string(), position);
    }
}