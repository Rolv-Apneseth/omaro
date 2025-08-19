use color_eyre::{Result, eyre::Context};
use rusqlite::{Connection, params};

use crate::data::Post;

pub fn mark_post_read(conn: &Connection, id: impl AsRef<str>) -> Result<()> {
    conn.execute(include_str!("./insert_post.sql"), params![id.as_ref()])
        .map(|_| ())
        .context("failed to execute: insert post")
}

pub fn mark_post_unread(conn: &Connection, id: impl AsRef<str>) -> Result<()> {
    if let Err(e) = conn.execute(include_str!("./delete_post.sql"), params![id.as_ref()])
        && !matches!(e, rusqlite::Error::QueryReturnedNoRows)
    {
        return Err(e).context("failed to execute: delete post");
    }

    Ok(())
}

pub fn update_posts(conn: &Connection, posts: &mut [Post]) -> Result<()> {
    let mut stmt = conn
        .prepare(include_str!("./get_post.sql"))
        .context("failed to prepare: get post")?;

    for p in posts.iter_mut() {
        match stmt.query_one(params![p.short_id.as_str()], |_| Ok(())) {
            Ok(_) => p.is_read = true,
            Err(e) => {
                if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                    continue;
                }

                return Err(e).context(format!(
                    "error getting post from the database: '{}'",
                    p.title
                ));
            }
        }
    }

    Ok(())
}
