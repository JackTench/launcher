use rusqlite::{Connection, Result};

pub struct Game {
    pub id: i32,
    pub name: String,
    pub platform: String,
    pub launch: String,
    pub times: i32,
}

pub struct Database {
    conn: Connection,
}

impl Database {

    // Create database.
    pub fn new() -> Result<Self> {
        let conn = Connection::open("launcher.db")?;
        Ok(Database { conn })
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute_batch(include_str!("sql/schema.sql"))?;
        Ok(())
    }

}