mod db;

use db::Database;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() {

    // Create app database.
    let database = Database::new().unwrap();
    database.create_table();

    let window = AppWindow::new().unwrap();

    // Create game for testing purposes. Just runs neofetch.
    database.add_game(
        String::from("Neofetch"),
        String::from("Linux"),
        String::from("neofetch"),
        1
    );
    let games: Vec<db::Game> = database.get().unwrap();
    

    // Get list of games to send to Slint to generate buttons.
    let mut game_names: Vec<String> = vec![];
    for game in games {
        game_names.push(game.name);
    }
    let game_names_ss: Vec<SharedString> = game_names.into_iter().map(Into::into).collect();
    let game_names_rc = ModelRc::new(VecModel::from(game_names_ss));
    window.set_games(game_names_rc);

    window.run().unwrap();
}
