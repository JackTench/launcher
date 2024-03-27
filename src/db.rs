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

    pub fn create_table(&self) {
        self.conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    }

    pub fn add_game(&self, game: &Game) {
        self.conn.execute(
            include_str!("sql/new_game.sql"),
            rusqlite::params![game.name, game.platform, game.launch, game.times],
        ).unwrap();
    }

}