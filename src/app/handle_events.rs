use std::sync::{atomic::Ordering, mpsc::Sender};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

use super::{App, TABLE_ROW_HEIGHT, action::DatabaseAction};
use crate::modes::Mode;

// TODO: support custom keybinds?
//
// pub fn get_shortcut_keys(count: usize, used_keys: &[u8]) -> Vec<u8> {
//     let mut res = Vec::with_capacity(count);
//
//     for k in ALL_KEYS {
//         if used_keys.contains(k) {
//             continue;
//         }
//
//         res.push(*k);
//         if res.len() == count {
//             break;
//         }
//     }
//
//     res
// }

pub const SHORTCUT_KEYS: &[u8] = b"123456789abdefimnopstvwxyz;:$&*/|\\^`'\"[]()<>";

impl App {
    pub(super) fn handle_key_event(
        &mut self,
        ev: KeyEvent,
        tx_posts: &Sender<Mode>,
        tx_details: &Sender<String>,
        tx_db: &Sender<DatabaseAction>,
    ) -> Result<()> {
        // Ctrl+c
        if ev.modifiers.contains(KeyModifiers::CONTROL) && matches!(ev.code, KeyCode::Char('c')) {
            self.exit_code = 1;
            self.is_running.store(false, Ordering::Release);
        }

        match ev.code {
            KeyCode::Char('q') => self.is_running.store(false, Ordering::Release),

            // POPUPS
            KeyCode::Esc => {
                self.show_details_popup = false;
                self.show_keybinds_popup = false;
            }
            KeyCode::Char('?') => {
                self.show_keybinds_popup = !self.show_keybinds_popup;
                return Ok(());
            }
            KeyCode::Char('K') => {
                if !self.posts.is_empty() {
                    self.load_post_comments(tx_details)?;
                    self.comments_list_state.select(Some(0));
                    self.show_details_popup = !self.show_details_popup;
                }
            }

            // FUNCTIONALITY
            KeyCode::Enter => {
                if self.show_details_popup {
                    self.open_comment()?;
                } else if let Some(selected) = self.posts_list_state.selected() {
                    self.open_post(selected, tx_db)?;
                }
            }
            KeyCode::Char('c') => {
                self.open_post_comments()?;
            }

            KeyCode::Char('r') => {
                if let Some(selected) = self.posts_list_state.selected() {
                    self.mark_post_read(selected, tx_db)?;
                }
            }
            KeyCode::Char('u') => {
                if let Some(selected) = self.posts_list_state.selected() {
                    self.mark_post_unread(selected, tx_db)?;
                }
            }

            KeyCode::Char('R') | KeyCode::F(5) => {
                if self.show_details_popup {
                    self.load_post_comments(tx_details)?;
                } else {
                    self.load_posts(tx_posts)?;
                }
            }

            // NAVIGATION
            KeyCode::Char('j') | KeyCode::Down => {
                self.next_row();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous_row();
            }

            KeyCode::Char('h') | KeyCode::Left | KeyCode::PageUp => {
                if !self.show_details_popup {
                    self.previous_page(tx_posts)?
                }
            }
            KeyCode::Char('l') | KeyCode::Right | KeyCode::PageDown => {
                if !self.show_details_popup {
                    self.next_page(tx_posts)?
                }
            }

            KeyCode::Char('g') | KeyCode::Home => self.first_row(),
            KeyCode::Char('G') | KeyCode::End => self.last_row(),

            KeyCode::Char('H') | KeyCode::Tab => self.next_mode(tx_posts)?,
            KeyCode::Char('L') | KeyCode::BackTab => self.prev_mode(tx_posts)?,

            KeyCode::Char(c) => self.handle_shortcut_key(c, tx_db)?,

            // For testing
            // KeyCode::Char('p') => panic!("User triggered panic"),
            // KeyCode::Char('e') => bail!("User triggered error"),
            _ => {}
        };

        // Close popup when any (other) keybind is pressed
        self.show_keybinds_popup = false;

        Ok(())
    }

    pub(super) fn handle_mouse_event(
        &mut self,
        ev: MouseEvent,
        tx_posts: &Sender<Mode>,
        tx_details: &Sender<String>,
        tx_db: &Sender<DatabaseAction>,
    ) -> Result<()> {
        let max = self.posts.len();
        let get_hovered = || {
            let start = self.table_starts_at;
            let offset = self.posts_list_state.offset();
            if ev.row >= self.table_starts_at && ev.row <= self.table_ends_at {
                let index = (ev.row - start) as usize / TABLE_ROW_HEIGHT + offset;
                if index < max {
                    return Some(index);
                }
            }
            None
        };

        match ev.kind {
            MouseEventKind::ScrollDown => self.next_row(),
            MouseEventKind::ScrollUp => self.previous_row(),
            MouseEventKind::ScrollLeft => self.previous_page(tx_posts)?,
            MouseEventKind::ScrollRight => self.next_page(tx_posts)?,
            MouseEventKind::Moved => {
                if !self.show_details_popup
                    && let Some(i) = get_hovered()
                {
                    self.posts_list_state.select(Some(i));
                }
            }
            MouseEventKind::Down(button) => {
                if let Some(i) = get_hovered() {
                    match button {
                        MouseButton::Left => {
                            if self.show_details_popup {
                                self.open_comment()?;
                            } else {
                                self.open_post(i, tx_db)?;
                            }
                        }
                        MouseButton::Right => {
                            if !self.posts.is_empty() {
                                self.load_post_comments(tx_details)?;
                                self.show_details_popup = !self.show_details_popup;
                                self.comments_list_state.select(Some(0));
                            }
                        }
                        MouseButton::Middle => self.open_post_comments()?,
                    };
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_shortcut_key(&mut self, c: char, tx_db: &Sender<DatabaseAction>) -> Result<()> {
        if let Some((index, _)) = SHORTCUT_KEYS
            .iter()
            .enumerate()
            .find(|(_, cc)| c == **cc as char)
            && index < self.posts.len()
        {
            self.open_post(index, tx_db)?;
        }

        Ok(())
    }
}
