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
        let mut current_guess_min: HashMap<char, usize> = HashMap::new();

        for rev in revelations {
            if rev.state == State::Correct || rev.state == State::Change {
                *current_guess_min.entry(rev.letter).or_insert(0) += 1;
            }

            if !self.revelations.contains(rev) {
                self.revelations.push(rev.clone());
            }
        }

        for (letter, count) in current_guess_min {
            let entry = self
                .constraints
                .entry(letter)
                .or_insert(LetterConstraint { min: 0, max: None });
            if count > entry.min {
                entry.min = count;
            }
        }

        for rev in revelations {
            if rev.state == State::Wrong {
                let entry = self
                    .constraints
                    .entry(rev.letter)
                    .or_insert(LetterConstraint { min: 0, max: None });
                let count_in_this_guess = revelations
                    .into_iter()
                    .filter(|r| {
                        r.letter == rev.letter
                            && (r.state == State::Correct || r.state == State::Change)
                    })
                    .count();
                entry.max = Some(count_in_this_guess);
            }
        }

        self.possibilities = self.filtering_possibilities();
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
                    state: State::Change,
                },
                LetterRevelation {
                    index: 2,
                    letter: 's',
                    state: State::Change,
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
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'l',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'e',
                    state: State::Change,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'a',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 'r',
                    state: State::Wrong,
                },
            ]),
        );

        revelations.insert(
            String::from("blink"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 'b',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'l',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'i',
                    state: State::Correct,
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
            String::from("shiny"),
            WordRevelation::new(vec![
                LetterRevelation {
                    index: 0,
                    letter: 's',
                    state: State::Correct,
                },
                LetterRevelation {
                    index: 1,
                    letter: 'h',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 2,
                    letter: 'i',
                    state: State::Correct,
                },
                LetterRevelation {
                    index: 3,
                    letter: 'n',
                    state: State::Wrong,
                },
                LetterRevelation {
                    index: 4,
                    letter: 'y',
                    state: State::Wrong,
                },
            ]),
        );
        for f in revelations.into_values() {
            solver.add_revelations(&f);
        }
        dbg!(&solver.constraints);
        assert!(solver.filter_word("spide"));
    }
}
