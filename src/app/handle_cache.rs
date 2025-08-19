//! Cache-related functionality - store posts to and load posts from the caches.
use std::mem;

use super::App;

impl App {
    pub(super) fn load_page(&mut self, prev_page: usize, page: usize) -> bool {
        if self.cache_posts.len() <= page {
            self.cache_posts.extend(
                [Vec::new()]
                    .into_iter()
                    .cycle()
                    .take(page + 1 - self.cache_posts.len()),
            );
        }

        mem::swap(&mut self.posts, &mut self.cache_posts[prev_page]);
        mem::swap(&mut self.posts, &mut self.cache_posts[page]);

        !self.posts.is_empty()
    }

    pub(super) fn store_mode(&mut self) {
        let page = self.mode.get_page();
        if page < self.cache_posts.len() {
            mem::swap(&mut self.posts, &mut self.cache_posts[page]);
        } else {
            self.cache_posts.push(mem::take(&mut self.posts));
        }

        self.cache_modes
            .insert(self.mode.to_string(), mem::take(&mut self.cache_posts));
    }

    pub(super) fn load_mode(&mut self) -> bool {
        let page = self.mode.get_page();
        match self.cache_modes.get_mut(&self.mode.to_string()) {
            Some(v) => {
                mem::swap(&mut self.cache_posts, v);
                mem::swap(&mut self.posts, &mut self.cache_posts[page]);
            }
            None => return false,
        }

        !self.posts.is_empty()
    }
}
