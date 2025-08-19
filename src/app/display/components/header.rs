use ratatui::{Frame, layout::Rect, style::Stylize, text::Text};

use crate::config::Config;

pub fn render_header(frame: &mut Frame, area: Rect, config: &Config) {
    if !config.ui.header.enable_ascii_header {
        return;
    }

    let header = Text::from(config.ui.header.text_ascii_header.as_str())
        .fg(config.ui.header.fg_ascii_header)
        .centered();

    frame.render_widget(header, area);
}
