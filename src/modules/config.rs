use crate::modules::revelation::Revelation;
use rand::rng;
use rand::seq::IndexedRandom;
use std::fs;

pub struct Config {
    pub _file_path: String,
    pub content: Vec<String>,
    pub chosen_word: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
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
    pub fn check(&self, guessed_word: String) -> Vec<Revelation> {
        let mut true_word = self.chosen_word.as_bytes().to_vec().clone();
        let correct_revelations: Vec<Revelation> = guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .map(|c| Revelation::get_correct(&mut true_word, c.1, c.0))
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect();
        let else_revelations: Vec<Revelation> = guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(index, _)| !correct_revelations.iter().any(|x| x.index == *index))
            .map(|c| Revelation::get_incorrect(&mut true_word, c.1, c.0))
            .collect();
        let mut revelations = [correct_revelations, else_revelations].concat();
        revelations.sort_by(|a, b| a.index.cmp(&b.index));
        revelations
    }
    pub fn word_exists(&self, guessed_word: &String) -> bool {
        self.content.iter().any(|f| f == guessed_word)
    }
}
