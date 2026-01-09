use crate::modules::revelation::Revelation;
use rand::rng;
use rand::seq::IndexedRandom;

#[derive(Default)]
pub struct Config {
    pub _file_path: String,
    pub content: Vec<String>,
    pub chosen_word: String,
}

impl Config {
    pub fn new(file_path: String, contents: String) -> Config {
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
    fn get_correct_revelations(&self, guessed_word: &str, true_word: &mut Vec<u8>) -> Vec<Revelation> {
        guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .map(|c| Revelation::get_correct(true_word, c.1, c.0))
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect()
    }
    fn get_incorrect_revelation(&self, guessed_word: &str, true_word: &mut Vec<u8>, correct_revelations: &Vec<Revelation>) -> Vec<Revelation> {
        guessed_word
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(index, _)| !correct_revelations.iter().any(|x| x.index == *index))
            .map(|c| Revelation::get_incorrect(true_word, c.1, c.0))
            .collect()
    }

    pub fn check(&self, guessed_word: &str) -> Vec<Revelation> {
        let mut true_word = self.chosen_word.as_bytes().to_vec().clone();
        let correct_revelations = self.get_correct_revelations(guessed_word, &mut true_word);
        let else_revelations: Vec<Revelation> = self.get_incorrect_revelation(guessed_word, &mut true_word, &correct_revelations);

        let mut revelations = [correct_revelations, else_revelations].concat();
        revelations.sort_by(|a, b| a.index.cmp(&b.index));
        revelations
    }

    pub fn word_exists(&self, guessed_word: &str) -> bool {
        self.content.iter().any(|f| f == guessed_word)
    }
}
