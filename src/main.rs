mod db;

use db::{Database, push_games};

slint::include_modules!();

fn main() {

    // Create app database.
    let database = Database::new().unwrap();
    database.create_table();

    // Create slint window.
    let window = AppWindow::new().unwrap();

    // Init vector of games.
    let games: Vec<db::Game> = database.get().unwrap();
    
    // Get list of games to send to Slint to generate buttons.
    let game_names_slint = push_games(games);
    window.set_games(game_names_slint);

    // Add game via slint GUI.
    window.on_add_game(move |name, platform, launch| {
        database.add_game(String::from(name.clone()), String::from(platform.clone()), String::from(launch.clone()));
    });

    window.run().unwrap();
}
