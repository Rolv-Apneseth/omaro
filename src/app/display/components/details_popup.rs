use chrono::Utc;
use ratatui::{Frame, layout::{Constraint::{Fill, Length, Percentage}, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Borders, Cell, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Row, Table}};
use textwrap::wrap;

use crate::{config::Config, data::Post, utils::{center_area, human_duration}};

pub fn render_details_popup(
    frame: &mut Frame,
    area: Rect,
    config: &Config,
    post: &Post,
    is_loading: bool,
    list_state: &mut ListState,
) {
    frame.render_widget(Clear, area);

    let mut block = Block::new()
        .padding(Padding::proportional(2))
        .title_top(" Details ".white());

    if config.ui.borders.enable {
        block = block
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.ui.borders.fg));
    }

    frame.render_widget(block, area);

    if is_loading {
        let area = center_area(area, Percentage(100), Length(1));
        frame.render_widget(Line::from("Loading...").centered(), area);
        return;
    }

    let rows = vec![
        Row::new([Cell::from("Title".bold()), Cell::from(post.title.clone())]),
        Row::new([
            Cell::from("User".bold()),
            Cell::from(post.submitter_user.clone()),
        ]),
        Row::new([
            Cell::from("Score".bold()),
            Cell::from(post.score.to_string()),
        ]),
        Row::new([
            Cell::from("Posted".bold()),
            Cell::from(post.created_at.naive_local().to_string()),
        ]),
        Row::new([Cell::from("Tags".bold()), Cell::from(post.tags.join(", "))]),
        Row::new([Cell::from("ID".bold()), Cell::from(post.short_id.clone())]),
    ];

    let [details, comments] = Layout::vertical([Length(rows.len() as u16), Fill(1)])
        .vertical_margin(2)
        .horizontal_margin(2)
        .spacing(1)
        .areas(area);

    let table = Table::new(rows, vec![Length(10), Percentage(100)]);
    frame.render_widget(table, details);

    let max_width = (comments.width as usize).saturating_sub(4);
    let max_lines = comments.height.saturating_sub(3);

    let items: Vec<ListItem> = post
        .comments
        .iter()
        .map(|comment| {
            let (score_fg, score_icon) = if comment.score < 0 {
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
            let since_comment = Utc::now().naive_utc() - comment.created_at.naive_utc();
            let since_comment = human_duration(since_comment);

            let indented = if comment.depth == 0 {
                String::new()
            } else {
                "  ".repeat(comment.depth)
            };
            let max_width = max_width.saturating_sub(indented.len());

            let first_line = Line::from_iter([
                Span::from(indented.clone()),
                Span::from(format!(
                    "{} {}  ",
                    config.ui.submitted_user.icon, comment.commenting_user
                ))
                .fg(config.ui.submitted_user.fg),
                Span::from(format!("{} {}  ", score_icon, comment.score)).fg(score_fg),
                Span::from(format!(
                    "{} {}",
                    config.ui.submitted_elapsed.icon, since_comment
                ))
                .fg(config.ui.submitted_elapsed.fg),
            ]);

            let mut text = Text::from(first_line);

            let mut lines = 1;
            for l in wrap(comment.comment_plain.trim(), max_width).iter() {
                if lines >= max_lines {
                    text.push_line(Line::from(format!("{indented}...")));
                    break;
                }

                text.push_line(format!("{indented}{}", l.trim().to_owned()));
                lines += 1;
            }

            text.push_line(Line::default());

            ListItem::from(text)
        })
        .collect();

    let popup = List::new(items)
        .scroll_padding(1)
        .highlight_spacing(HighlightSpacing::Never)
        .highlight_style(Style::default().bold())
        .block(
            Block::default()
                .padding(Padding::horizontal(2))
                .title_top(Line::from_iter([
                    Span::from("Comments").bold().white(),
                    Span::from(format!(" ({})", post.comment_count)),
                ])),
        );
    frame.render_stateful_widget(popup, comments, list_state);
}
