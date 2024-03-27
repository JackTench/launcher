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

    pub fn get(&self) -> Result<Vec<Game>> {
        let mut statement = self.conn.prepare(
            include_str!("sql/get_all.sql")
        )?;
        let rows = statement.query_map([], |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                platform: row.get(2)?,
                launch: row.get(3)?,
                times: row.get(4)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn create_table(&self) {
        self.conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    }

    pub fn add_game(&self, name: String, platform: String, launch: String, times: i32) {
        self.conn.execute(
            include_str!("sql/new_game.sql"),
            rusqlite::params![name, platform, launch, times],
        ).unwrap();
    }
}