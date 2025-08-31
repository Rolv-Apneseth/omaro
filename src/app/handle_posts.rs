use std::sync::{atomic::Ordering, mpsc::Sender};

use color_eyre::{Result, Section, eyre::Context};
use reqwest::blocking::Client;

use super::{App, DownloadedBytes, action::DatabaseAction};
use crate::{data::{Post, PostDetails}, modes::Mode};
pub(super) fn fetch_posts(
    client: &Client,
    mode: &Mode,
    downloaded: &mut DownloadedBytes,
) -> Result<Vec<Post>> {
    let url = mode.get_url();
    let req = client.get(url).build().context("failed to build request")?;

    let resp = client
        .execute(req)
        .context("failed requesting posts")
        .suggestion("check internet access")?;

    if let Some(len) = resp.content_length() {
        let _ = downloaded.fetch_add(len as u32, Ordering::Relaxed);
    };

    resp.json::<Vec<Post>>()
        .context("failed converting posts from JSON - maybe the format has changed?")
}

pub(super) fn fetch_post_details(
    client: &Client,
    url: impl AsRef<str>,
    downloaded: &mut DownloadedBytes,
) -> Result<PostDetails> {
    let url = format!("{}.json", url.as_ref());

    let req = client
        .get(&url)
        .build()
        .context("failed to build request")?;

    let resp = client
        .execute(req)
        .context("failed requesting post details")?;

    if let Some(len) = resp.content_length() {
        let _ = downloaded.fetch_add(len as u32, Ordering::Relaxed);
    };

    resp.json::<PostDetails>().context(format!(
        "failed converting post details from JSON. Url: {url}"
    ))
}

impl App {
    pub(super) fn load_posts(&self, tx: &Sender<Mode>) -> Result<()> {
        self.is_loading.store(true, Ordering::Relaxed);
        tx.send(self.mode.clone())
            .context("load posts channel is closed")
    }

    pub(super) fn load_post_comments(&self, tx: &Sender<String>) -> Result<()> {
        let Some(post) = self.current_post() else {
            return Ok(());
        };

        self.is_loading_comments.store(true, Ordering::Relaxed);
        let url = post.short_id_url.clone();
        tx.send(url).context("load post details channel is closed")
    }

    pub(super) fn open_post(&mut self, index: usize, tx: &Sender<DatabaseAction>) -> Result<()> {
        assert!(
            index < self.posts.len(),
            "trying to open post out of bounds"
        );

        let Some(post) = self.current_post() else {
            return Ok(());
        };

        if post.url.is_empty() {
            self.open_post_comments()?;
        } else {
            open::that_detached(&post.url).context("failed to launch link opener")?;
        };

        self.mark_post_read(index, tx)
    }

    pub(super) fn mark_post_read(
        &mut self,
        index: usize,
        tx: &Sender<DatabaseAction>,
    ) -> Result<()> {
        assert!(
            index < self.posts.len(),
            "trying to open post out of bounds"
        );

        // Mark read straight away - DB status will only matter the next time the
        // program is launched
        let post = &mut self.posts[index];
        post.is_read = true;

        tx.send(DatabaseAction::MarkPostRead(post.short_id.clone()))
            .context("mark post read channel is closed")
    }

    pub(super) fn mark_post_unread(
        &mut self,
        index: usize,
        tx: &Sender<DatabaseAction>,
    ) -> Result<()> {
        assert!(
            index < self.posts.len(),
            "trying to open post out of bounds"
        );

        // Mark unread straight away - DB status will only matter the next time the
        // program is launched
        let post = &mut self.posts[index];
        post.is_read = false;

        tx.send(DatabaseAction::MarkPostUnread(post.short_id.clone()))
            .context("mark post read channel is closed")
    }

    pub(super) fn open_post_comments(&mut self) -> Result<()> {
        if let Some(post) = self.current_post() {
            open::that_detached(&post.comments_url).context("failed to launch link opener")?;
        }

        Ok(())
    }

    pub(super) fn open_comment(&mut self) -> Result<()> {
        let (Some(post), Some(index)) = (self.current_post(), self.comments_list_state.selected())
        else {
            return Ok(());
        };
        assert!(
            index < post.comments.len(),
            "trying to open comment out of bounds"
        );

        open::that_detached(&post.comments[index].url).context("failed to launch link opener")?;

        Ok(())
    }

    pub(super) fn current_post(&self) -> Option<&Post> {
        let index = self.posts_list_state.selected()?;
        assert!(
            index < self.posts.len(),
            "trying to open post out of bounds"
        );

        Some(&self.posts[index])
    }
}
