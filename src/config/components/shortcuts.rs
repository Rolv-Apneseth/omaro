use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ShortcutsUiConfig {
    #[serde(default = "default_enable")]
    pub enable: bool,
    #[serde(default = "default_fg")]
    pub fg: Color,

    #[serde(default = "default_icon_left")]
    pub icon_left: char,
    #[serde(default = "default_icon_right")]
    pub icon_right: char,
}

impl Default for ShortcutsUiConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            fg: default_fg(),
            icon_left: default_icon_left(),
            icon_right: default_icon_right(),
        }
    }
}

fn default_enable() -> bool {
    true
}
fn default_fg() -> Color {
    Color::Magenta
}
fn default_icon_left() -> char {
    ''
}
fn default_icon_right() -> char {
    ''
}
