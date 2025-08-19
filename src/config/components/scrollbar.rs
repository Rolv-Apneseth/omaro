use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ScrollbarConfig {
    #[serde(default = "default_enable")]
    pub enable: bool,
    #[serde(default = "default_fg_thumb")]
    pub fg_thumb: Color,
}

impl Default for ScrollbarConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            fg_thumb: default_fg_thumb(),
        }
    }
}

fn default_enable() -> bool {
    false
}
fn default_fg_thumb() -> Color {
    Color::Gray
}
