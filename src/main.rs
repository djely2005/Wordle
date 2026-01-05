use std::{env, io};
pub mod modules;
use modules::config::Config;
use modules::game::Game;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let mut game = Game::new(config);
    ratatui::run(|terminal| game.run(terminal))
    // game.new_game();
}
