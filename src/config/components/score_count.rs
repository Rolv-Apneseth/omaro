use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ScoreCountConfig {
    #[serde(default = "default_enable")]
    pub enable: bool,
    #[serde(default = "default_fg_positive")]
    pub fg_positive: Color,
    #[serde(default = "default_fg_negative")]
    pub fg_negative: Color,
    #[serde(default = "default_icon_positive")]
    pub icon_positive: char,
    #[serde(default = "default_icon_negative")]
    pub icon_negative: char,
}

impl Default for ScoreCountConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            fg_positive: default_fg_positive(),
            fg_negative: default_fg_negative(),
            icon_positive: default_icon_positive(),
            icon_negative: default_icon_negative(),
        }
    }
}

fn default_enable() -> bool {
    true
}
fn default_fg_positive() -> Color {
    Color::Yellow
}
fn default_fg_negative() -> Color {
    Color::Red
}
fn default_icon_positive() -> char {
    ''
}
fn default_icon_negative() -> char {
    ''
}
