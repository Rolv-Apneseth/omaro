use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct HeaderConfig {
    #[serde(default = "default_enable_ascii_header")]
    pub enable_ascii_header: bool,
    #[serde(default = "default_fg_ascii_header")]
    pub fg_ascii_header: Color,
    #[serde(default = "default_text_ascii_header")]
    pub text_ascii_header: String,
    #[serde(default = "default_enable_border_header")]
    pub enable_border_header: bool,
    #[serde(default = "default_fg_border_header")]
    pub fg_border_header: Color,
    #[serde(default = "default_text_border_header")]
    pub text_border_header: String,
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            enable_ascii_header: default_enable_ascii_header(),
            fg_ascii_header: default_fg_ascii_header(),
            text_ascii_header: default_text_ascii_header(),
            enable_border_header: default_enable_border_header(),
            fg_border_header: default_fg_border_header(),
            text_border_header: default_text_border_header(),
        }
    }
}

fn default_enable_ascii_header() -> bool {
    true
}
fn default_fg_ascii_header() -> Color {
    Color::White
}
fn default_text_ascii_header() -> String {
    String::from(
        " \
 ██████╗ ███╗   ███╗ █████╗ ██████╗  ██████╗
██╔═══██╗████╗ ████║██╔══██╗██╔══██╗██╔═══██╗
██║   ██║██╔████╔██║███████║██████╔╝██║   ██║
██║   ██║██║╚██╔╝██║██╔══██║██╔══██╗██║   ██║
╚██████╔╝██║ ╚═╝ ██║██║  ██║██║  ██║╚██████╔╝
 ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝\
    ",
    )
}
fn default_enable_border_header() -> bool {
    false
}
fn default_fg_border_header() -> Color {
    Color::White
}
fn default_text_border_header() -> String {
    String::from("Omaro")
}
