use std::{path::{Path, PathBuf}, sync::LazyLock, time::Duration};

use color_eyre::{Result, eyre::Context};
use dirs::data_local_dir;
use include_dir::{Dir, include_dir};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;
use rusqlite_migration::Migrations;

pub mod queries;

/// A database connection pool
pub type DbPool = Pool<SqliteConnectionManager>;
/// A connection to the database
pub type DbConnection = PooledConnection<SqliteConnectionManager>;

// DB MIGRATIONS, DEFINED IN ./migrations
static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/database/migrations");
static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

// DB FILE PATH
pub static DEFAULT_DB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    data_local_dir()
        .expect("could not identify data directory")
        .join("omaro.db")
});

/// Get a database connection. Make sure the DB is initialised somewhere first
/// before calling this function.
pub fn get_db_connection(pool: &DbPool) -> Result<DbConnection> {
    pool.get()
        .context("failed to open a connection to the database")
}

/// Initialise the database, creating the file and running migrations if needed
pub fn init_db(path_db: &Path) -> Result<DbPool> {
    // SETUP POOL
    let manager = SqliteConnectionManager::file(path_db).with_flags(OpenFlags::default());
    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(5))
        .min_idle(Some(2))
        .max_size(5)
        .build(manager)
        .context("failed DB connection pool init")?;

    // INIT DB
    let mut conn = get_db_connection(&pool)?;
    conn.pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
        .context("failed to apply PRAGMA: journal mode")?;
    MIGRATIONS
        .to_latest(&mut conn)
        .context("failed to apply migrations")?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations() {
        assert!(MIGRATIONS.validate().is_ok());
        // insta::assert_debug_snapshot!(MIGRATIONS);
    }
}
