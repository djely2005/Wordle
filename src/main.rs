use rand::rng;
use rand::seq::IndexedRandom;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
struct Revealation {
    letter: char,
    index: usize,
    state: State,
}

impl Revealation {
    fn get_correct(
        true_word: &mut [u8],
        guessed_letter: &u8,
        guessed_index: usize,
    ) -> Option<Revealation> {
        let mut state = State::Wrong;
        let mut found_index = 0;
        for i in 0..5 {
            if true_word[i] == *guessed_letter && guessed_index == i {
                state = State::Correct;
                found_index = i;
                break;
            }
        }
        match state {
            State::Correct => {
                true_word[found_index] = 0x20;
                return Some(Revealation {
                    letter: *guessed_letter as char,
                    index: guessed_index,
                    state,
                });
            }
            _ => return None,
        }
    }
    fn get_incorrect(
        true_word: &mut [u8],
        guessed_letter: &u8,
        guessed_index: usize,
    ) -> Revealation {
        let mut state = State::Wrong;
        let mut found_index = 0;
        for i in 0..5 {
            if true_word[i] == *guessed_letter && guessed_index != i {
                state = State::Change;
                found_index = i;
                break;
            }
        }
        true_word[found_index] = 0x20;
        return Revealation {
            letter: *guessed_letter as char,
            index: guessed_index,
            state,
        };
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
            chosen_word: chosen_word,
        }
    }
    fn check(&self, guessed_word: String) -> Vec<Revealation> {
        let mut true_word = self.chosen_word.as_bytes().to_vec().clone();
        let correct_revelations: Vec<Revealation> = guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .map(|c| Revealation::get_correct(&mut true_word, c.1, c.0))
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect();
        let else_revelations: Vec<Revealation> = guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(index, _)| !correct_revelations.iter().any(|x| x.index == *index))
            .map(|c| Revealation::get_incorrect(&mut true_word, c.1, c.0))
            .collect();
        let mut revelations = [correct_revelations, else_revelations].concat();
        revelations.sort_by(|a, b| a.index.cmp(&b.index));
        revelations
    }
    fn word_exists(&self, guessed_word: &String) -> bool {
        self.content.iter().any(|f| f == guessed_word)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("The chosen Word Is: {}", config.chosen_word);
    let mut solved = false;

    for _attempt in 0..4 {
        let mut guess: String;
        loop {
            println!("Write Guess");
            guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess.retain(|c| !c.is_whitespace());
            let letter_count = guess.as_bytes().iter().len();
            if letter_count != 5 {
                println!("Word must be of 5 letters");
                continue;
            }
            if config.word_exists(&guess) {
                break;
            } else {
                println!("Word doesn't exist");
            }
        }
        let revealations = config.check(guess);
        if revealations.iter().all(|f| f.state == State::Correct) {
            solved = true;
            break;
        }
    }
    if solved {
        println!("Congratulations");
    } else {
        println!("Too bad !")
    }
}
