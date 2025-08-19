mod components;

use std::sync::atomic::Ordering;

use components::{render_container, render_details_popup, render_header, render_keybinds_popup, render_posts, render_scrollbar};
use ratatui::{Frame, layout::{Constraint::{Length, Max, Min, Percentage}, Layout}, text::Line};

use super::App;
use crate::utils::center_area;

impl App {
    pub(super) fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        self.prev_size = (area.width, area.height);

        // Enforce a minimum size
        if area.width < 60 || area.height < 20 {
            let area = center_area(frame.area(), Percentage(100), Length(1));
            frame.render_widget(Line::from("Terminal too small").centered(), area);
            return;
        };

        let is_loading = self.is_loading.load(Ordering::Relaxed);

        let area = center_area(area, Max(100), Percentage(100));
        let [area] = Layout::vertical([Percentage(100)]).areas(area);
        render_container(frame, area, &self.config, &self.mode, &self.downloaded);
        render_scrollbar(frame, area, &mut self.posts_scroll_state, &self.config);

        let body = if self.config.ui.header.enable_ascii_header {
            let header_lines = self.config.ui.header.text_ascii_header.lines().count() as u16;
            let [header, body] = Layout::vertical([Min(header_lines), Percentage(100)])
                .spacing(2)
                .vertical_margin(2)
                .horizontal_margin(4)
                .areas(area);
            render_header(frame, header, &self.config);

            body
        } else {
            Layout::vertical([Percentage(100)])
                .vertical_margin(2)
                .areas::<1>(area)[0]
        };

        let body = center_area(body, Max(60), Max((self.posts.len() * 2).max(3) as u16));

        // Loading - return early
        if is_loading {
            let body = center_area(body, Percentage(100), Length(1));
            frame.render_widget(Line::from("Loading...").centered(), body);
            return;
        }

        // No posts - return early
        if self.posts.is_empty() {
            let body = center_area(body, Percentage(100), Length(1));
            frame.render_widget(Line::from("No Results").centered(), body);
            return;
        }

        // Update table position
        self.table_starts_at = body.top();
        self.table_ends_at = body.bottom() - 2;

        render_posts(
            frame,
            body,
            &mut self.posts_list_state,
            &self.posts,
            &self.config,
        );

        if self.show_keybinds_popup {
            render_keybinds_popup(frame, area, &self.config);
        } else if self.show_details_popup
            && let Some(index) = self.posts_list_state.selected()
        {
            assert!(
                index < self.posts.len(),
                "trying to open post out of bounds"
            );

            let area = center_area(area, Percentage(90), Percentage(90));
            let post = &self.posts[index];
            let is_loading = self.is_loading_comments.load(Ordering::Relaxed);

            render_details_popup(
                frame,
                area,
                &self.config,
                post,
                is_loading,
                &mut self.comments_list_state,
            );
        };
    }
}
