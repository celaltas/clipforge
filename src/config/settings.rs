use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub window_width: u32,
    pub window_height: u32,
    pub max_history_items: usize,
    pub poll_interval: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_width: 1000,
            window_height: 700,
            max_history_items: 10,
            poll_interval: 1000,
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Settings {{\n    window_width: {},\n    window_height: {},\n    max_history_items: {},\n    poll_interval: {}\n}}",
            self.window_width, self.window_height, self.max_history_items, self.poll_interval
        )
    }
}
