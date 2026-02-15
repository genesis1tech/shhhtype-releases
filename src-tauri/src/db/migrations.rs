use anyhow::Result;
use rusqlite::Connection;

/// Run all database migrations.
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS history (
            id TEXT PRIMARY KEY,
            text TEXT NOT NULL,
            duration_ms INTEGER NOT NULL,
            model TEXT NOT NULL,
            created_at TEXT NOT NULL,
            app_name TEXT,
            word_count INTEGER NOT NULL DEFAULT 0
        );

        CREATE INDEX IF NOT EXISTS idx_history_created_at
            ON history(created_at DESC);
        ",
    )?;

    log::info!("Database migrations complete");
    Ok(())
}
