use std::io;
pub mod modules;
use modules::config::Config;
use modules::game::Game;

const FILE_PATH: &str = "./valid-wordle-words.txt";
const WORDS: &str = include_str!("../valid-wordle-words.txt");
fn main() -> io::Result<()> {
    let mut game = Game::start();
    ratatui::run(|terminal| game.run(terminal))
}
