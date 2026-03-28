use anyhow::Result;
use rusqlite::Connection;

/// A saved transcription record.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub text: String,
    pub duration_ms: i64,
    pub model: String,
    pub created_at: String,
    pub app_name: Option<String>,
    pub word_count: i32,
}

/// Query parameters for history search.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryQuery {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Insert a new history entry.
pub fn insert(conn: &Connection, entry: &HistoryEntry) -> Result<()> {
    conn.execute(
        "INSERT INTO history (id, text, duration_ms, model, created_at, app_name, word_count)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            entry.id,
            entry.text,
            entry.duration_ms,
            entry.model,
            entry.created_at,
            entry.app_name,
            entry.word_count,
        ],
    )?;
    Ok(())
}

/// Query history entries with optional search filter.
pub fn query(conn: &Connection, q: &HistoryQuery) -> Result<Vec<HistoryEntry>> {
    let limit = q.limit.unwrap_or(50);
    let offset = q.offset.unwrap_or(0);

    let mut entries = Vec::new();

    if let Some(search) = &q.search {
        let pattern = format!("%{}%", search);
        let mut stmt = conn.prepare(
            "SELECT id, text, duration_ms, model, created_at, app_name, word_count
             FROM history WHERE text LIKE ?1
             ORDER BY created_at DESC LIMIT ?2 OFFSET ?3",
        )?;
        let rows = stmt.query_map(rusqlite::params![pattern, limit, offset], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                text: row.get(1)?,
                duration_ms: row.get(2)?,
                model: row.get(3)?,
                created_at: row.get(4)?,
                app_name: row.get(5)?,
                word_count: row.get(6)?,
            })
        })?;
        for row in rows {
            entries.push(row?);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, text, duration_ms, model, created_at, app_name, word_count
             FROM history ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
        )?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                text: row.get(1)?,
                duration_ms: row.get(2)?,
                model: row.get(3)?,
                created_at: row.get(4)?,
                app_name: row.get(5)?,
                word_count: row.get(6)?,
            })
        })?;
        for row in rows {
            entries.push(row?);
        }
    }

    Ok(entries)
}

/// Delete a history entry by ID.
pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM history WHERE id = ?1", rusqlite::params![id])?;
    Ok(())
}
