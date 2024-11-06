use rusqlite::{Connection, Result, params};

// Open a SQLite database connection
pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("./bmi_records.db")?;
    Ok(conn)
}

// Create the HealthData table with a unique entry_date constraint
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS HealthData (
            id INTEGER PRIMARY KEY,
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
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM HealthData WHERE entry_date = DATE('now')")?;
    let already_exists: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(already_exists > 0)
}

// Insert or update the record for today
pub fn upsert_record(conn: &Connection, bmi: f64, height: f64, weight: f64, category: &str) -> Result<()> {
    conn.execute(
        "
        INSERT INTO HealthData (entry_date, bmi, height, weight, category)
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
