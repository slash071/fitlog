use rusqlite::{Connection, Result, params};
use std::fs;
use dirs;
use colored::Colorize;

// Open a SQLite database connection
pub fn open_db() -> Result<Connection> {
    // Retrieve the configuration directory path and join the fitlog/bmi_records.db path
    let config_dir = dirs::config_dir().ok_or_else(|| {
        rusqlite::Error::InvalidPath("Could not find configuration directory".into())
    })?;
    let db_dir = config_dir.join("fitlog");      // Directory path
    let db_file = db_dir.join("bmi_records.db"); // Database file path

    // Check if the directory exists; if not, create it
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).map_err(|e| {
            rusqlite::Error::InvalidPath(format!("Failed to create directory: {:?}", e).into())
        })?;
        println!("{}", "> Database initialized.".truecolor(223, 142, 29));
    }

    Connection::open(db_file)
}

// Create the BMIData table with a unique entry_date constraint
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS BMIData (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            entry_date TEXT NOT NULL DEFAULT CURRENT_DATE UNIQUE,
            bmi REAL NOT NULL,
            height REAL NOT NULL,
            weight REAL NOT NULL,
            category TEXT
        )",
        [],
    )?;
    Ok(())
}

// Check if a record for today already exists
pub fn record_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM BMIData WHERE entry_date = DATE('now'))")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}

// Insert or update the record for today
pub fn upsert_record(conn: &Connection, bmi: f64, height: f64, weight: f64, category: &str) -> Result<()> {
    conn.execute(
        "
        INSERT INTO BMIData (entry_date, bmi, height, weight, category)
        VALUES (DATE('now'), ?1, ?2, ?3, ?4)
        ON CONFLICT(entry_date) DO UPDATE SET
            bmi = excluded.bmi,
            height = excluded.height,
            weight = excluded.weight,
            category = excluded.category
        ",
        params![bmi, height, weight, category],
    )?;
    Ok(())
}
