use std::error::Error;

use anyhow::Result;
use diesel::{pg::Pg, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod models;
pub mod schema;
pub mod types;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub fn connect(url: &str) -> Result<PgConnection> {
    PgConnection::establish(url).map_err(|e| e.into())
}

pub fn run_migrations(
    conn: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
