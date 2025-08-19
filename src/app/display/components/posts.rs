use chrono::Utc;
use ratatui::{Frame, layout::Rect, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{List, ListItem, ListState}};

use crate::{app::handle_events::SHORTCUT_KEYS, config::Config, data::Post, utils::{human_duration, truncate}};

const COLUMN_SPACING: u16 = 1;
const SHORTCUT_WIDTH: u16 = 5;

pub fn render_posts(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    posts: &[Post],
    config: &Config,
) {
    let max_width = usize::from(
        area.width
            - COLUMN_SPACING
            - if config.ui.shortcuts.enable {
                SHORTCUT_WIDTH
            } else {
                0
            },
    );
    let selected = list_state.selected().unwrap_or_default();

    let rows = posts.iter().enumerate().map(|(i, post)| {
        let is_selected = selected == i;
        let if_not_read = |c: Color| {
            if post.is_read { Color::DarkGray } else { c }
        };

        let since_post = Utc::now().naive_utc() - post.created_at.naive_utc();

        let text_color = if is_selected {
            if post.is_read {
                Color::Gray
            } else {
                Color::White
            }
        } else {
            if_not_read(Color::Gray)
        };

        let mut second_line = Line::default();
        let mut first_line = Line::default();

        let title = truncate(&post.title, max_width);
        if config.ui.shortcuts.enable {
            let shortcut = format!(
                "{}{}{}  ",
                config.ui.shortcuts.icon_left,
                SHORTCUT_KEYS[i] as char,
                config.ui.shortcuts.icon_right
            );
            debug_assert_eq!(shortcut.chars().count() as u16, SHORTCUT_WIDTH);

            first_line.push_span(Span::from(shortcut).fg(if_not_read(config.ui.shortcuts.fg)));
            second_line.push_span(Span::from("     "));
        }
        first_line.push_span(Span::from(title).fg(text_color));

        if config.ui.score_count.enable {
            let (fg, icon) = if post.score < 0 {
                (
                    config.ui.score_count.fg_negative,
                    config.ui.score_count.icon_negative,
                )
            } else {
                (
                    config.ui.score_count.fg_positive,
                    config.ui.score_count.icon_positive,
                )
            };

            second_line.push_span(
                Span::from(format!("  {icon} {:<4}", post.score))
                    .bold()
                    .fg(if_not_read(fg)),
            );
        }

        if config.ui.comment_count.enable {
            second_line.push_span(
                Span::from(format!(
                    " {} {:<4}",
                    config.ui.comment_count.icon, post.comment_count
                ))
                .fg(config.ui.comment_count.fg),
            );
        }

        if config.ui.submitted_elapsed.enable {
            second_line.push_span(
                Span::from(format!(
                    "{} {:<10}",
                    config.ui.submitted_elapsed.icon,
                    human_duration(since_post)
                ))
                .fg(config.ui.submitted_elapsed.fg),
            );
        }

        if config.ui.submitted_user.enable {
            second_line.push_span(
                Span::from(format!(
                    "{} {}",
                    config.ui.submitted_user.icon, post.submitter_user
                ))
                .fg(config.ui.submitted_user.fg),
            );
        }

        ListItem::from(Text::from_iter([first_line, second_line]))
    });

    let table = List::from_iter(rows)
        .scroll_padding(1)
        .highlight_style(Style::default().bold());

    frame.render_stateful_widget(table, area, list_state);
}
