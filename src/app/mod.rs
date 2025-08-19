mod action;
mod display;
mod handle_cache;
mod handle_events;
mod handle_posts;
mod navigate;
mod run;

use std::{sync::{Arc, atomic::{AtomicBool, AtomicU32}}, time::Duration};

use color_eyre::Result;
use hashbrown::HashMap;
use ratatui::widgets::{ListState, ScrollbarState};
use reqwest::blocking::{Client, ClientBuilder};

use crate::{config::Config, data::Post, database::DbPool, modes::Mode};

pub const TABLE_ROW_HEIGHT: usize = 2;

pub type DownloadedBytes = Arc<AtomicU32>;

#[derive(Debug)]
pub struct App {
    client: Arc<Client>,
    config: Config,
    db: Arc<DbPool>,
    mode: Mode,

    posts: Vec<Post>,
    cache_posts: Vec<Vec<Post>>,
    cache_modes: HashMap<String, Vec<Vec<Post>>>,
    posts_list_state: ListState,
    comments_list_state: ListState,
    posts_scroll_state: ScrollbarState,
    table_starts_at: u16,
    table_ends_at: u16,
    prev_size: (u16, u16),

    show_keybinds_popup: bool,
    show_details_popup: bool,

    is_loading: Arc<AtomicBool>,
    is_loading_comments: Arc<AtomicBool>,
    is_running: Arc<AtomicBool>,
    downloaded: DownloadedBytes,

    pub exit_code: i32,
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"),);

impl App {
    pub fn new(db: DbPool, config: Config) -> Result<Self> {
        let client = ClientBuilder::default()
            .timeout(Duration::from_secs(20))
            .user_agent(USER_AGENT)
            .build()
            .expect("failed to build HTTP client");

        let mode = config.default_mode.clone();

        Ok(App {
            client: client.into(),
            db: db.into(),
            config,
            mode,

            is_running: AtomicBool::new(true).into(),
            is_loading: AtomicBool::new(true).into(),
            posts_list_state: ListState::default().with_selected(Some(0)),
            posts_scroll_state: ScrollbarState::new(25 * TABLE_ROW_HEIGHT),
            comments_list_state: ListState::default().with_selected(Some(0)),
            // Capacity only needs to cover the available modes
            cache_modes: HashMap::with_capacity(3),

            show_keybinds_popup: Default::default(),
            show_details_popup: Default::default(),
            is_loading_comments: Default::default(),
            exit_code: Default::default(),
            posts: Default::default(),
            cache_posts: Default::default(),
            prev_size: Default::default(),
            table_starts_at: Default::default(),
            table_ends_at: Default::default(),
            downloaded: Default::default(),
        })
    }
}
