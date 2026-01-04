use std::io;

use crate::modules::{config::Config, revelation::{Revelation}, state::State};

pub struct Game {
    config: Config,
    revelations: Option<Vec<Vec<Revelation>>>
}

impl Game {
    pub fn new(config: Config) -> Game{
        Game { config, revelations: None }
    }
    pub fn new_game(&self){
        self.show_true_word();
        let mut solved = false;
        for _attempt in 0..4 {
            let guess = self.get_input();
            let revelation = self.config.check(guess);
            if Game::check_game_over(revelation){
                solved = true;
                break;
            }
        }
        if solved {
            println!("Congratulations");
        } else {
            println!("Too bad !!!");
        }
        self.show_true_word();
    }

    fn get_input(&self) -> String{
        println!("Write Guess:");
        loop {
            let mut guess = String::new();
            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            guess.retain(|c| !c.is_whitespace());
            if guess.as_bytes().iter().len() != 5 {
                println!("Word must be of 5 letters");
                continue;
            }
            if self.config.word_exists(&guess) {
                return guess;
            } else {
                println!("Word doesn't exist");
            }
        }
    }

    fn check_game_over(revelation: Vec<Revelation>) -> bool{
        revelation.iter().all(|f| f.state == State::Correct)
    }
    fn show_true_word(&self){
        println!("The word was: {:?}", self.config.chosen_word);
    }
}