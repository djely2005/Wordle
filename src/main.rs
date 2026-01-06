use std::io;
pub mod modules;
use modules::config::Config;
use modules::game::Game;

const FILE_PATH: &str = "./valid-wordle-words.txt";
const WORDS: &str = include_str!("../valid-wordle-words.txt");
fn main() -> io::Result<()> {
    let config = Config::new(FILE_PATH.to_string(), WORDS.to_string());
    let mut game = Game::new(config);
    ratatui::run(|terminal| game.run(terminal))
}
