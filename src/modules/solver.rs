use crate::modules::{revelation::Revelation, state::State};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct LetterConstraint {
    pub min: usize,
    pub max: Option<usize>,
}

#[derive(Default)]
pub struct Solver {
    pub possibilities: Vec<String>,
    pub revelations: Vec<Revelation>,
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

    pub fn number_of_possibilities(&self) -> usize {
        self.possibilities.len()
    }

    pub fn add_revelation(&mut self, rev: &Revelation) {
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

    pub fn filtering_possibilities(&mut self) {
        self.possibilities.retain(|word| {
            let bytes = word.as_bytes();

            let positional_ok = self.revelations.iter().all(|rev| {
                match rev.state {
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
                }
            });

            if !positional_ok {
                return false;
            }

            let mut counts: HashMap<char, usize> = HashMap::new();
            for c in word.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }

            self.constraints.iter().all(|(letter, constraint)| {
                let actual = *counts.get(letter).unwrap_or(&0);

                actual >= constraint.min && constraint.max.map_or(true, |max| actual <= max)
            })
        });
    }
}
