use std::sync::mpsc::Sender;

use color_eyre::Result;

use super::{App, TABLE_ROW_HEIGHT};
use crate::modes::Mode;

impl App {
    pub(super) fn first_row(&mut self) {
        if self.posts.is_empty() {
            return;
        }

        if self.show_details_popup {
            self.comments_list_state.select_first();
            return;
        }

        self.posts_list_state.select_first();
        self.posts_scroll_state = self.posts_scroll_state.position(0);
    }

    pub(super) fn last_row(&mut self) {
        if self.posts.is_empty() {
            return;
        }

        if self.show_details_popup {
            self.comments_list_state.select_last();
            return;
        }

        self.posts_list_state.select_last();
        self.posts_scroll_state = self
            .posts_scroll_state
            .position(self.posts.len() * TABLE_ROW_HEIGHT);
    }

    pub(super) fn next_row(&mut self) {
        if self.posts.is_empty() {
            return;
        }

        if self.show_details_popup {
            self.comments_list_state.select_next();
            return;
        }

        let i = match self.posts_list_state.selected() {
            Some(i) => {
                if i >= self.posts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.posts_list_state.select(Some(i));
        self.posts_scroll_state = self.posts_scroll_state.position(i * TABLE_ROW_HEIGHT);
    }

    pub(super) fn previous_row(&mut self) {
        if self.posts.is_empty() {
            return;
        }

        if self.show_details_popup {
            self.comments_list_state.select_previous();
            return;
        }

        let i = match self.posts_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.posts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.posts_list_state.select(Some(i));
        self.posts_scroll_state = self.posts_scroll_state.position(i * TABLE_ROW_HEIGHT);
    }

    pub(super) fn next_page(&mut self, tx: &Sender<Mode>) -> Result<()> {
        // Wait to load the current posts - or there are no more posts so don't proceed
        if self.posts.is_empty() {
            return Ok(());
        }

        if self.mode.next_page() {
            let page = self.mode.get_page();
            let prev_page = page - 1;
            if !self.load_page(prev_page, page) {
                self.load_posts(tx)?;
            };

            self.first_row();
        }
        Ok(())
    }

    pub(super) fn previous_page(&mut self, tx: &Sender<Mode>) -> Result<()> {
        if self.mode.prev_page() {
            let page = self.mode.get_page();
            let prev_page = page + 1;
            if !self.load_page(prev_page, page) {
                self.load_posts(tx)?;
            };

            self.first_row();
        }
        Ok(())
    }

    pub(super) fn next_mode(&mut self, tx: &Sender<Mode>) -> Result<()> {
        self.first_row();
        self.store_mode();
        self.mode.next_mode();

        if !self.load_mode() {
            self.load_posts(tx)?;
        }

        Ok(())
    }
    pub(super) fn prev_mode(&mut self, tx: &Sender<Mode>) -> Result<()> {
        self.first_row();
        self.store_mode();
        self.mode.prev_mode();

        if !self.load_mode() {
            self.load_posts(tx)?;
        }

        Ok(())
    }
}
