use crossterm::event::{KeyEvent, MouseEvent};

use crate::data::{Post, PostDetails};

#[derive(Debug)]
pub enum Action {
    #[allow(dead_code)]
    Resize(u16, u16),
    Key(KeyEvent),
    Mouse(MouseEvent),
    LoadPosts(Vec<Post>),
    LoadPostDetails(PostDetails),
}

#[derive(Debug)]
pub enum DatabaseAction {
    MarkPostRead(String),
    MarkPostUnread(String),
}
