use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BordersConfig {
    #[serde(default = "default_enable")]
    pub enable: bool,
    #[serde(default = "default_fg")]
    pub fg: Color,
}

impl Default for BordersConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            fg: default_fg(),
        }
    }
}

fn default_enable() -> bool {
    true
}
fn default_fg() -> Color {
    Color::Magenta
}
