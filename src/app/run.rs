use std::{panic, sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{Receiver, RecvTimeoutError, Sender, channel}}, thread::{self, JoinHandle}, time::Duration};

use color_eyre::{Result, eyre::Context};
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use reqwest::blocking::Client;

use super::{App, DownloadedBytes, action::{Action, DatabaseAction}, handle_posts::{fetch_post_details, fetch_posts}};
use crate::{database::{DbPool, get_db_connection, queries::{mark_post_read, mark_post_unread, update_posts}}, modes::Mode};

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let (tx_actions, rx_actions) = channel::<Action>();
        let (tx_load, rx_load) = channel::<Mode>();
        let (tx_load_comments, rx_load_comments) = channel::<String>();
        let (tx_db, rx_db) = channel::<DatabaseAction>();

        let mut handles = Vec::new();

        // Handle terminal events
        let is_running = Arc::clone(&self.is_running);
        let tx_actions_clone = tx_actions.clone();
        handles.push(
            thread::Builder::new()
                .name("term_events".into())
                .spawn(move || on_thread_events(&is_running, tx_actions_clone))?,
        );

        // Handle fetching posts
        let is_running = Arc::clone(&self.is_running);
        let db = Arc::clone(&self.db);
        let downloaded = Arc::clone(&self.downloaded);
        let client = Arc::clone(&self.client);
        let tx_actions_clone = tx_actions.clone();
        handles.push(
            thread::Builder::new()
                .name("fetch_posts".into())
                .spawn(move || {
                    on_thread_posts(
                        &is_running,
                        db,
                        client,
                        rx_load,
                        tx_actions_clone,
                        downloaded,
                    )
                    .context("post fetching thread")
                })?,
        );

        // Handle loading post details
        let is_running = Arc::clone(&self.is_running);
        let downloaded = Arc::clone(&self.downloaded);
        let client = Arc::clone(&self.client);
        handles.push(
            thread::Builder::new()
                .name("fetch_post_details".into())
                .spawn(move || {
                    on_thread_post_details(
                        &is_running,
                        client,
                        rx_load_comments,
                        tx_actions,
                        downloaded,
                    )
                })?,
        );

        // Handle database operations
        let is_running = Arc::clone(&self.is_running);
        let db = Arc::clone(&self.db);
        handles.push(
            thread::Builder::new()
                .name("db_operations".into())
                .spawn(move || on_thread_db(&is_running, db, rx_db))?,
        );

        // Run main thread
        let mut res = self.main_loop(
            terminal,
            &tx_load,
            &tx_load_comments,
            &tx_db,
            rx_actions,
            &mut handles,
        );
        self.is_running.store(false, Ordering::Relaxed);

        // Join and check for errors in other threads
        for handle in handles {
            match handle.join() {
                Ok(r) => match r {
                    Ok(_) => {}
                    Err(e) => {
                        // TODO: replace with combining reports once <https://github.com/eyre-rs/eyre/pull/208> is merged
                        // For now, simply prioritise errors from the other threads
                        res = Err(e);
                    }
                },
                Err(p) => panic::resume_unwind(p),
            }
        }

        res
    }

    fn main_loop(
        &mut self,
        terminal: &mut DefaultTerminal,
        tx_load: &Sender<Mode>,
        tx_load_details: &Sender<String>,
        tx_db: &Sender<DatabaseAction>,
        rx_actions: Receiver<Action>,
        handles: &mut Vec<JoinHandle<Result<()>>>,
    ) -> Result<()> {
        // Init - draw frame and start loading posts
        terminal.draw(|frame| self.draw(frame))?;

        if self.posts.is_empty() {
            self.load_posts(tx_load)?;
        }

        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            // Check for early returns (errors, panics) in other threads
            for i in 0..handles.len() {
                if handles[i].is_finished() {
                    let h = handles.remove(i);
                    match h.join() {
                        Ok(r) => return r,
                        Err(p) => panic::resume_unwind(p),
                    }
                }
            }

            let event = match rx_actions.recv_timeout(Duration::from_millis(50)) {
                Ok(ev) => ev,
                Err(e) => match e {
                    // Check `is_running` again
                    RecvTimeoutError::Timeout => continue,
                    RecvTimeoutError::Disconnected => return Ok(()),
                },
            };

            match event {
                Action::LoadPosts(posts) => {
                    self.posts = posts;
                    self.is_loading.store(false, Ordering::Relaxed);
                    self.first_row();
                }
                Action::LoadPostDetails(post_details) => {
                    if let Some(post) = self
                        .posts
                        .iter_mut()
                        .find(|p| p.short_id == post_details.short_id)
                    {
                        post.comments = post_details.comments;
                        self.is_loading_comments.store(false, Ordering::Relaxed)
                    }
                }
                Action::Key(ev) => self.handle_key_event(ev, tx_load, tx_load_details, tx_db)?,
                Action::Mouse(ev) => {
                    self.handle_mouse_event(ev, tx_load, tx_load_details, tx_db)?
                }
                // Reset selection to first element to scroll whole
                // table into view if scaling up
                Action::Resize(_, h) => {
                    if h > self.prev_size.1 {
                        self.first_row();
                    }
                }
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }
}

fn on_thread_events(is_running: &Arc<AtomicBool>, tx_actions: Sender<Action>) -> Result<()> {
    while is_running.load(Ordering::Relaxed) {
        // Check `is_running` again
        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        #[allow(clippy::single_match)]
        match event::read().context("could not read event")? {
            Event::Key(ev) => tx_actions.send(Action::Key(ev))?,
            Event::Mouse(ev) => tx_actions.send(Action::Mouse(ev))?,
            Event::Resize(w, h) => tx_actions.send(Action::Resize(w, h))?,
            _ => {}
        };
    }
    Ok(())
}

fn on_thread_posts(
    is_running: &Arc<AtomicBool>,
    db: Arc<DbPool>,
    client: Arc<Client>,
    rx_load: Receiver<Mode>,
    tx_actions: Sender<Action>,
    mut downloaded: DownloadedBytes,
) -> Result<()> {
    while is_running.load(Ordering::Relaxed) {
        match rx_load.recv_timeout(Duration::from_millis(50)) {
            Err(e) => match e {
                // Check `is_running` again
                RecvTimeoutError::Timeout => continue,
                RecvTimeoutError::Disconnected => return Ok(()),
            },
            Ok(mode) => {
                let mut posts = fetch_posts(&client, &mode, &mut downloaded)?;
                let conn = get_db_connection(&db)?;
                update_posts(&conn, &mut posts)?;
                tx_actions.send(Action::LoadPosts(posts))?;
            }
        }
    }
    Ok(())
}

fn on_thread_post_details(
    is_running: &Arc<AtomicBool>,
    client: Arc<Client>,
    rx_load_details: Receiver<String>,
    tx_actions: Sender<Action>,
    mut downloaded: DownloadedBytes,
) -> Result<()> {
    while is_running.load(Ordering::Relaxed) {
        match rx_load_details.recv_timeout(Duration::from_millis(50)) {
            Err(e) => match e {
                // Check `is_running` again
                RecvTimeoutError::Timeout => continue,
                RecvTimeoutError::Disconnected => return Ok(()),
            },
            Ok(url) => {
                let details = fetch_post_details(&client, &url, &mut downloaded)?;
                tx_actions.send(Action::LoadPostDetails(details))?;
            }
        }
    }
    Ok(())
}

fn on_thread_db(
    is_running: &Arc<AtomicBool>,
    db: Arc<DbPool>,
    rx_db: Receiver<DatabaseAction>,
) -> Result<()> {
    while is_running.load(Ordering::Relaxed) {
        let action = match rx_db.recv_timeout(Duration::from_millis(50)) {
            Err(e) => match e {
                // Check `is_running` again
                RecvTimeoutError::Timeout => continue,
                RecvTimeoutError::Disconnected => return Ok(()),
            },
            Ok(action) => action,
        };

        let conn = get_db_connection(&db)?;
        match action {
            DatabaseAction::MarkPostRead(id) => {
                mark_post_read(&conn, id)?;
            }
            DatabaseAction::MarkPostUnread(id) => {
                mark_post_unread(&conn, id)?;
            }
        }
    }

    Ok(())
}
