use rand::rng;
use rand::seq::IndexedRandom;
use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
enum State {
    Correct,
    Wrong,
    Change,
}

struct Config {
    _file_path: String,
    content: Vec<String>,
    chosen_word: String,
}
#[derive(Debug)]
struct Revealation {
    letter: char,
    index: usize,
    state: State,
}

impl Revealation {
    fn new(config: &Config, guessed_letter: &u8, guessed_index: usize) -> Revealation {
        let mut state = State::Wrong;
        for i in 0..5 {
            if config.chosen_word.as_bytes()[i] == *guessed_letter {
                if guessed_index == i.try_into().unwrap() {
                    state = State::Correct;
                    return Revealation {
                        letter: *guessed_letter as char,
                        index: guessed_index,
                        state,
                    };
                } else {
                    state = State::Change
                }
            }
        }
        Revealation {
            letter: *guessed_letter as char,
            index: guessed_index,
            state,
        }
    }
}

impl Config {
    fn new(args: &[String]) -> Config {
        let file_path = args[1].clone();

        let contents = fs::read_to_string(&file_path).expect("Couldn't Read File");
        let words: Vec<String> = contents
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let mut rng = rng();
        let chosen_word = words
            .choose(&mut rng)
            .expect("Couldn't Choose A Word")
            .clone();
        Config {
            _file_path: file_path,
            content: words,
            chosen_word,
        }
    }
    fn check(&self, guessed_word: String) -> Vec<Revealation> {
        let revelation: Vec<Revealation> = guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .map(|c| Revealation::new(self, c.1, c.0))
            .collect();
        revelation
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("The chosen Word Is: {}", config.chosen_word);

    for _attempt in 0..5 {
        let mut guess = String::new();
        println!("Write Guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess.retain(|c| !c.is_whitespace());
        let test = config.check(guess);
        println!("{:?}", test)
    }
}
