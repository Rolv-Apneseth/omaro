use std::sync::atomic::{AtomicU32, Ordering};

use ratatui::{Frame, layout::Rect, style::{Style, Stylize}, text::{Line, Span}, widgets::{Block, BorderType, Borders}};

use crate::{config::{Config, DownloadedConfig, HeaderConfig, KeybindHintsConfig}, modes::Mode, utils::human_bytes};

fn downloaded(config: &DownloadedConfig, bytes: u32) -> Line<'_> {
    Line::from(format!(" {} {} ", config.icon, human_bytes(bytes)))
        .right_aligned()
        .fg(config.fg)
}

fn keybind_hints(config: &KeybindHintsConfig) -> Line<'_> {
    Line::from(vec![
        Span::from(" "),
        Span::from(format!("{}q{}", config.icon_left, config.icon_right)).bold(),
        Span::from(" quit"),
        Span::from(" | "),
        Span::from(format!("{}?{}", config.icon_left, config.icon_right)).bold(),
        Span::from(" keybinds"),
        Span::from(" "),
    ])
    .right_aligned()
    .fg(config.fg)
}

fn border_header(config: &HeaderConfig) -> Line<'_> {
    Line::from(
        format!(" {} ", config.text_border_header)
            .fg(config.fg_border_header)
            .bold(),
    )
}

pub fn render_container(
    frame: &mut Frame,
    area: Rect,
    config: &Config,
    mode: &Mode,
    bytes_downloaded: &AtomicU32,
) {
    let mut block = Block::new();

    if config.ui.mode_info.enable {
        let (Mode::Newest(n) | Mode::Hottest(n) | Mode::Active(n)) = mode;
        let mode_info = Line::default()
            .left_aligned()
            .fg(config.ui.mode_info.fg)
            .spans([
                Span::from(" Mode: ").bold(),
                Span::from(mode.to_string()),
                Span::from("  "),
                Span::from("Page: ").bold(),
                Span::from(format!("{n}  ")),
            ]);
        block = block.title_bottom(mode_info);
    }

    if config.ui.borders.enable {
        block = block
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.ui.borders.fg));
    }

    if config.ui.header.enable_border_header {
        block = block.title_top(border_header(&config.ui.header));
    };

    if config.ui.downloaded.enable {
        let bytes = bytes_downloaded.load(Ordering::Relaxed);
        block = block.title_top(downloaded(&config.ui.downloaded, bytes));
    };

    if config.ui.keybind_hints.enable {
        block = block.title_bottom(keybind_hints(&config.ui.keybind_hints));
    }

    frame.render_widget(block, area);
}
