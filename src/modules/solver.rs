use crate::modules::revelation::WordRevelation;

use super::{revelation::LetterRevelation, state::State};

use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct LetterConstraint {
    pub min: usize,
    pub max: Option<usize>,
}

#[derive(Default)]
pub struct Solver {
    pub possibilities: Vec<String>,
    pub revelations: Vec<LetterRevelation>,
    pub constraints: HashMap<char, LetterConstraint>,
}

impl Solver {
    pub fn new(possibilities: Vec<String>) -> Solver {
        Solver {
            possibilities,
            revelations: vec![],
            constraints: HashMap::new(),
        }
    }

    fn calculate_probability_of_round(&self, number_of_possible_word_before_guess: usize) -> f64 {
        (self.number_of_possibilities() as f64) / (number_of_possible_word_before_guess as f64)
    }

    fn simulate_revelation(&mut self) {
        let old_possibilities = &self.possibilities;
    }

    pub fn number_of_possibilities(&self) -> usize {
        self.possibilities.len()
    }

    pub fn add_revelations(&mut self, revelations: &WordRevelation) {
        for rev in revelations {
            self.add_revelation(rev);
        }
        self.possibilities = self.filtering_possibilities();
    }

    pub fn add_revelation(&mut self, rev: &LetterRevelation) {
        if self.revelations.contains(rev) {
            return;
        }
        let entry = self
            .constraints
            .entry(rev.letter)
            .or_insert(LetterConstraint { min: 0, max: None });

        match rev.state {
            State::Correct | State::Change => {
                entry.min += 1;
            }
            State::Wrong => {
                entry.max = Some(entry.min);
            }
        }

        self.revelations.push(rev.clone());
    }

    fn filter_word(&self, word: &str) -> bool {
        let bytes = word.as_bytes();

        let positional_ok = self.revelations.iter().all(|rev| match rev.state {
            State::Correct => bytes[rev.index] == rev.letter as u8,
            State::Change => bytes[rev.index] != rev.letter as u8,
            State::Wrong => {
                let constraint = self.constraints.get(&rev.letter);
                if let Some(c) = constraint {
                    if c.max == Some(c.min) && c.min > 0 {
                        true
                    } else {
                        bytes[rev.index] != rev.letter as u8
                    }
                } else {
                    bytes[rev.index] != rev.letter as u8
                }
            }
        });

        if !positional_ok {
            return false;
        }

        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in word.chars() {
            *counts.entry(c).or_insert(0) += 1
        }

        self.constraints.iter().all(|(letter, constraint)| {
            let actual = *counts.get(letter).unwrap_or(&0);
            actual >= constraint.min && constraint.max.map_or(true, |max| actual <= max)
        })
    }

    fn filtering_possibilities(&mut self) -> Vec<String> {
        self.possibilities
            .iter()
            .filter(|word| self.filter_word(word))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_letter() {
        let mut solver = Solver::new(vec![]);
        solver.add_revelation(&LetterRevelation {
            index: 0,
            letter: 't',
            state: State::Correct,
        });
        assert!(solver.filter_word("tests"));
    }

    #[test]
    fn wrong_letter() {
        let mut solver = Solver::new(vec![]);
        solver.add_revelation(&LetterRevelation {
            index: 0,
            letter: 't',
            state: State::Wrong,
        });
        assert!(!solver.filter_word("tests"));
    }

    #[test]
    fn change_letter() {
        let mut solver = Solver::new(vec![]);
        solver.add_revelation(&LetterRevelation {
            index: 0,
            letter: 't',
            state: State::Change,
        });
        assert!(!solver.filter_word("tests"));
    }

    #[test]
    fn everything_correct() {
        let mut solver = Solver::new(vec![]);
        for (idx, c) in String::from("tests").chars().enumerate() {
            solver.add_revelation(&LetterRevelation {
                index: idx,
                letter: c,
                state: State::Correct,
            });
        }
        assert!(solver.filter_word("tests"));
    }

    #[test]
    fn bug() {
        let mut solver = Solver::new(vec![]);
        let mut revelations: HashMap<String, WordRevelation> = Default::default();
        revelations.insert(
            String::from("tests"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 't',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'e',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 's',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 't',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 's',
                    state: State::Wrong,
                },
            ]),
        );
        revelations.insert(
            String::from("clear"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'c',
                    state: State::Change,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'l',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'e',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'a',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 'r',
                    state: State::Change,
                },
            ]),
        );

        revelations.insert(
            String::from("blink"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'b',
                    state: State::Change,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'l',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'i',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'n',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 'k',
                    state: State::Wrong,
                },
            ]),
        );
        revelations.insert(
            String::from("tames"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 't',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'a',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'm',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'e',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 's',
                    state: State::Wrong,
                },
            ]),
        );
        revelations.insert(
            String::from("humps"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'h',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'u',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'm',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'p',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 's',
                    state: State::Wrong,
                },
            ]),
        );
        for f in revelations.into_values() {
            solver.add_revelations(&f);
        }
        dbg!(&solver.constraints);
        assert!(solver.filter_word("scrob"));
    }
    #[test]
    fn another_bug() {
        let mut solver = Solver::new(vec![]);
        let mut revelations: HashMap<String, WordRevelation> = Default::default();
        revelations.insert(
            String::from("tests"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 't',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'e',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 's',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 't',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 's',
                    state: State::Wrong,
                },
            ]),
        );
        revelations.insert(
            String::from("clear"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'c',
                    state: State::Change,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'l',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'e',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'a',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 'r',
                    state: State::Change,
                },
            ]),
        );

        revelations.insert(
            String::from("blink"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'b',
                    state: State::Change,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'l',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'i',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'n',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 'k',
                    state: State::Wrong,
                },
            ]),
        );
        revelations.insert(
            String::from("tames"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 't',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'a',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'm',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'e',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 's',
                    state: State::Wrong,
                },
            ]),
        );
        revelations.insert(
            String::from("humps"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'h',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'u',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'm',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'p',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 's',
                    state: State::Wrong,
                },
            ]),
        );
        assert!(solver.filter_word("scrob"));
    }
}
