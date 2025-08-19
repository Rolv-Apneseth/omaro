use ratatui::{Frame, layout::{Margin, Rect}, style::Style, widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState}};

use crate::config::Config;

pub fn render_scrollbar(
    frame: &mut Frame,
    area: Rect,
    scroll_state: &mut ScrollbarState,
    config: &Config,
) {
    if !config.ui.scrollbar.enable {
        return;
    }

    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .thumb_style(Style::default().fg(config.ui.scrollbar.fg_thumb))
            .track_symbol(None)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        scroll_state,
    );
}
