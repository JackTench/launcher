use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;

slint::include_modules!();

struct Game {
    id: i32,
    name: String,
    platform: String,
    launch: String,
    times: i32,
}

fn main() {
    let window = AppWindow::new().unwrap();

    // Create game for testing purposes. Just runs neofetch.
    let mut games: Vec<Game> = Vec::new();
    games.push(Game {
        id: 1,
        name: String::from("Test Game"),
        platform: String::from("PC"),
        launch: String::from("neofetch"),
        times: 1,
    });

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
