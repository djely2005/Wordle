use crate::modules::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct Revelation {
    pub letter: char,
    pub index: usize,
    pub state: State,
}

impl Revelation {
    pub fn get_correct(
        true_word: &mut [u8],
        guessed_letter: &u8,
        guessed_index: usize,
    ) -> Option<Revelation> {
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
                return Some(Revelation {
                    letter: *guessed_letter as char,
                    index: guessed_index,
                    state,
                });
            }
            _ => return None,
        }
    }
    pub fn get_incorrect(
        true_word: &mut [u8],
        guessed_letter: &u8,
        guessed_index: usize,
    ) -> Revelation {
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
        return Revelation {
            letter: *guessed_letter as char,
            index: guessed_index,
            state,
        };
    }
}
