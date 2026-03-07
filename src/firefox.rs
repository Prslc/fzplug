use crate::ui;
use crate::utils;
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

pub enum Mode {
    Bookmarks,
    History,
}

pub fn search_and_launch(mode: Mode, search_text: &str) -> Result<()> {
    // Path to Firefox database
    let db_path = utils::get_firefox_db_path()?;

    // Copy to temporary file to avoid DB lock
    let tmp_file = NamedTempFile::new()?;
    fs::copy(&db_path, tmp_file.path())?;

    let conn = Connection::open(tmp_file.path())?;

    // SQL clauses depending on mode
    let (join_clause, order_clause) = match mode {
        Mode::Bookmarks => (
            "FROM moz_bookmarks JOIN moz_places ON moz_bookmarks.fk = moz_places.id",
            "ORDER BY moz_bookmarks.dateAdded DESC",
        ),
        Mode::History => (
            "FROM moz_places JOIN moz_historyvisits ON moz_places.id = moz_historyvisits.place_id",
            "ORDER BY moz_historyvisits.visit_date DESC",
        ),
    };

    let limit_clause = if search_text.is_empty() {
        "LIMIT 50"
    } else {
        ""
    };

    let search_pattern = format!("%{}%", search_text);

    let sql = format!(
        "SELECT moz_places.title, moz_places.url
         {}
         WHERE moz_places.url <> ''
         AND (?1 = '' OR moz_places.title LIKE ?2 OR moz_places.url LIKE ?2)
         {}
         {}",
        join_clause, order_clause, limit_clause
    );

    let mut stmt = conn.prepare(&sql)?;

    // Query results
    let rows = stmt.query_map([search_text, &search_pattern], |row| {
        let title: Option<String> = row.get(0)?;
        let url: String = row.get(1)?;
        Ok((title, url))
    })?;

    let mut choices_str = String::new();
    for row in rows {
        if let Ok((title, url)) = row {
            let title = title.unwrap_or_default();
            if title.is_empty() {
                choices_str.push_str(&format!("{}\n", url));
            } else {
                choices_str.push_str(&format!("{} | {}\n", title, url));
            }
        }
    }

    if choices_str.is_empty() {
        return Ok(());
    }

    // Launch selection menu
    if let Some(selected) = ui::dmenu("Firefox: ", &choices_str) {
        if let Some(url) = selected.rsplit('|').next() {
            let clean_url = url.trim();
            Command::new("xdg-open")
                .arg(clean_url)
                .spawn()
                .context("Failed to open URL")?;
        }
    }

    Ok(())
}
