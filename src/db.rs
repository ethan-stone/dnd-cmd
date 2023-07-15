use super::character::Character;
use rusqlite::{Connection, Result};

pub fn create_connection(filename: String) -> Result<Connection> {
    let conn = Connection::open(filename)?;

    Ok(conn)
}

pub fn create_tables(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS character (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn insert_character(conn: &mut Connection, character: Character) -> Result<()> {
    conn.execute(
        "INSERT INTO character (name) VALUES (?1)",
        &[&character.name.to_string()],
    )?;

    Ok(())
}
