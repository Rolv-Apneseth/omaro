use ratatui::{Frame, layout::{Constraint::{Fill, Length}, Rect}, style::{Style, Stylize}, widgets::{Block, BorderType, Borders, Cell, Clear, Padding, Row, Table}};

use crate::{config::Config, utils::center_area};

const PADDING: u16 = 2;
const SPACING: u16 = 2;

const KEYS: [[&str; 2]; 17] = [
    ["    󰁅 / j", "Scroll down"],
    ["    󰁝 / k", "Scroll up"],
    ["  G / End", "Scroll to last"],
    [" g / Home", "Scroll to first"],
    ["    󰁍 / h", "Previous page"],
    ["    󰁔 / l", "Next page"],
    ["  L / Tab", "Next mode"],
    ["H / S+Tab", "Previous mode"],
    ["    Enter", "Open post"],
    ["        c", "Open comments"],
    ["        r", "Mark read"],
    ["        u", "Mark unread"],
    ["        K", "Toggle details"],
    ["   Escape", "Close popup"],
    ["   R / F5", "Refresh"],
    ["        q", "Quit"],
    ["   Ctrl+c", "Force quit"],
];
const HEIGHT: u16 = KEYS.len() as u16 + PADDING * 2 + 2;

pub fn render_keybinds_popup(frame: &mut Frame, area: Rect, config: &Config) {
    let width_key: u16 = KEYS[0][0].chars().count() as u16;
    let width_desc = KEYS.iter().map(|[_, d]| d.len()).max().unwrap() as u16;
    let width_borders = 2;
    let width_padding = PADDING * 2 * 2; // proportional padding
    let width = width_key + width_desc + width_borders + width_padding + SPACING;

    let area = center_area(area, Length(width), Length(HEIGHT));
    frame.render_widget(Clear, area);

    let mut block = Block::new()
        .padding(Padding::proportional(PADDING))
        .title_top(" Keybinds ".white());

    if config.ui.borders.enable {
        block = block
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.ui.borders.fg));
    }

    let widths = vec![Length(width_key), Fill(1)];
    let rows: Vec<Row> = KEYS
        .into_iter()
        .map(|[keys, desc]| Row::new([Cell::from(keys).bold(), Cell::from(desc)]))
        .collect();

    let popup = Table::new(rows, widths)
        .block(block)
        .column_spacing(SPACING);

    frame.render_widget(popup, area);
}
