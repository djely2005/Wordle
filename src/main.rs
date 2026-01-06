use std::io;
pub mod modules;
use modules::game::Game;
fn main() -> io::Result<()> {
    let mut game = Game::start();
    ratatui::run(|terminal| game.run(terminal))
}
