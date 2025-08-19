use std::{borrow::Cow, fmt::Display, str::FromStr};

use color_eyre::eyre::{Context, eyre};
use serde::{Deserialize, Deserializer};

const URL: &str = "https://lobste.rs";
// Treat pages as 1-indexed - while 0 works, it gives the same results as 1 so
// may as well skip it
const STARTING_PAGE: u8 = 1;

/// Modes used for selecting API endpoint to fetch data from. Inner values
/// represent the page number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Newest(u8),
    Hottest(u8),
    Active(u8),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Hottest(STARTING_PAGE)
    }
}

impl Mode {
    /// Get the URL for the endpoint corresponding to this mode.
    pub fn get_url(&self) -> String {
        match self {
            Self::Hottest(page) => format!("{URL}/page/{page}.json"),
            Self::Newest(page) => format!("{URL}/newest/page/{page}.json"),
            Self::Active(page) => format!("{URL}/active/page/{page}.json"),
        }
    }

    /// Increment the page for this mode, if possible. Returns true if the page
    /// changed.
    pub fn next_page(&mut self) -> bool {
        match self {
            Self::Hottest(page) | Self::Newest(page) | Self::Active(page) => {
                let prev = *page;
                *page = page.saturating_add(1);
                // (*page ^ *page) == 0
                *page != prev
            }
        }
    }

    /// Decrement the page for this mode, if possible. Returns true if the page
    /// changed.
    pub fn prev_page(&mut self) -> bool {
        match self {
            Self::Hottest(page) | Self::Newest(page) | Self::Active(page) => {
                if *page > STARTING_PAGE {
                    *page -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Cycle to the next mode.
    pub fn next_mode(&mut self) {
        *self = match self {
            Self::Hottest(_) => Self::Newest(STARTING_PAGE),
            Self::Newest(_) => Self::Active(STARTING_PAGE),
            Self::Active(_) => Self::Hottest(STARTING_PAGE),
        };
    }

    /// Cycle to the previous mode.
    pub fn prev_mode(&mut self) {
        *self = match self {
            Self::Newest(_) => Self::Hottest(STARTING_PAGE),
            Self::Hottest(_) => Self::Active(STARTING_PAGE),
            Self::Active(_) => Self::Newest(STARTING_PAGE),
        };
    }

    /// Get the page number stored in this mode, converting it to a 0-indexed
    /// value.
    pub fn get_page(&self) -> usize {
        let (Self::Hottest(page) | Self::Newest(page) | Self::Active(page)) = self;
        (*page as usize) - 1
    }
}

impl FromStr for Mode {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "newest" => Ok(Self::Newest(STARTING_PAGE)),
            "hottest" => Ok(Self::Hottest(STARTING_PAGE)),
            "active" => Ok(Self::Active(STARTING_PAGE)),
            _ => Err(eyre!("Not a valid mode")),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Hottest(_) => "Hottest",
                Self::Newest(_) => "Newest",
                Self::Active(_) => "Active",
            }
        )
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Cow::<'de, str>::deserialize(deserializer)?;
        Mode::from_str(&s)
            .context("Possible modes: newest, hottest, active")
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn mode_page_limits() {
        let start = STARTING_PAGE - 1;
        let get_page = |m: &Mode| m.get_page() as u8;

        let mut mode = Mode::default();
        assert_eq!(get_page(&mode), start);

        assert!(!mode.prev_page());
        assert_eq!(get_page(&mode), start);

        let mut page = start;
        for _ in 1..u8::MAX {
            page += 1;
            assert!(mode.next_page());
            assert_eq!(get_page(&mode), page);
        }

        assert!(!mode.next_page());
        assert_eq!(get_page(&mode), u8::MAX - 1);
    }

    #[test]
    fn changing_mode_resets_page() {
        let mut mode = Mode::default();
        assert!(matches!(mode, Mode::Hottest(STARTING_PAGE)));
        assert!(mode.next_page());

        mode.next_mode();
        assert_eq!(mode, Mode::Newest(STARTING_PAGE));
        assert!(mode.next_page());

        mode.next_mode();
        assert_eq!(mode, Mode::Active(STARTING_PAGE));
        assert!(mode.next_page());

        mode.next_mode();
        assert_eq!(mode, Mode::default());
        assert!(mode.next_page());

        mode.prev_mode();
        assert_eq!(mode, Mode::Active(STARTING_PAGE));
        assert!(mode.next_page());

        mode.prev_mode();
        assert_eq!(mode, Mode::Newest(STARTING_PAGE));
        assert!(mode.next_page());

        mode.prev_mode();
        assert_eq!(mode, Mode::default());
        assert!(mode.next_page());
    }
}
