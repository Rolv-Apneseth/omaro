use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer};

fn deserialize_date_from_str<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let date = chrono::DateTime::parse_from_str(&s, "%+").unwrap();
    Ok(date)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Post {
    pub short_id: String,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub created_at: DateTime<FixedOffset>,
    pub title: String,
    pub url: String,
    pub score: i32,
    pub comment_count: u32,
    pub submitter_user: String,
    pub tags: Vec<String>,
    pub short_id_url: String,
    pub comments_url: String,
    // description_plain: String,
    // flags: u32,
    // user_is_author

    // Custom properties
    #[serde(default)]
    pub is_read: bool,

    #[serde(default)]
    pub comments: Vec<PostComment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostDetails {
    pub short_id: String,
    pub comments: Vec<PostComment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostComment {
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub created_at: DateTime<FixedOffset>,
    pub score: i32,
    pub comment_plain: String,
    pub depth: usize,
    pub commenting_user: String,
    pub url: String,
    // #[serde(deserialize_with = "deserialize_date_from_str")]
    // pub last_edited_at: DateTime<FixedOffset>,
    // pub flags: u32,
    // pub parent_comment: Option<String>,
    // pub short_id: String,
    // pub comment: String,
    // pub is_deleted: bool,
    // pub is_moderated: bool,
}
