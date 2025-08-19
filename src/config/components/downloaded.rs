use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct DownloadedConfig {
    #[serde(default = "default_enable")]
    pub enable: bool,
    #[serde(default = "default_fg")]
    pub fg: Color,
    #[serde(default = "default_icon")]
    pub icon: String,
}

impl Default for DownloadedConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            fg: default_fg(),
            icon: default_icon(),
        }
    }
}

fn default_enable() -> bool {
    true
}
fn default_fg() -> Color {
    Color::Gray
}
fn default_icon() -> String {
    String::from("󰇚")
}
