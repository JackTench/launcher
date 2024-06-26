use rusqlite::{Connection, Result};
use slint::{ModelRc, SharedString, VecModel};

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
    
    // Get all games in the database.
    // Returns a vector of the Game struct.
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

    // Creates the schema in the database on first run.
    pub fn create_table(&self) {
        self.conn.execute_batch(include_str!("sql/schema.sql")).unwrap();
    }

    // Add game to database.
    pub fn add_game(&self, name: String, platform: String, launch: String) {
        self.conn.execute(
            include_str!("sql/new_game.sql"),
            rusqlite::params![name, platform, launch, 0],
        ).unwrap();
    }
}

pub fn push_games(game_vector: Vec<Game>) -> ModelRc<SharedString> {
    let mut game_names: Vec<String> = vec![];
    for game in game_vector {
        game_names.push(game.name);
    }
    let game_names_ss: Vec<SharedString> = game_names.into_iter().map(Into::into).collect();
    let game_names_rc: ModelRc<SharedString> = ModelRc::new(VecModel::from(game_names_ss));
    game_names_rc
}