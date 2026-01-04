use std::env;
pub mod modules;
use modules::config::Config;
use modules::game::Game;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let game = Game::new(config);

    game.new_game();
}
