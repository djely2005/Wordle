use crate::modules::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct LetterRevelation {
    pub index: usize,
    pub letter: char,
    pub state: State,
}

impl LetterRevelation {
    pub fn get_correct(
        true_word: &mut [u8],
        guessed_letter: &u8,
        guessed_index: usize,
    ) -> Option<LetterRevelation> {
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
                return Some(LetterRevelation {
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
    ) -> LetterRevelation {
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
        return LetterRevelation {
            letter: *guessed_letter as char,
            index: guessed_index,
            state,
        };
    }
}

pub struct WordRevelation(
    LetterRevelation,
    LetterRevelation,
    LetterRevelation,
    LetterRevelation,
    LetterRevelation,
);

impl WordRevelation {
    pub fn new(v: Vec<LetterRevelation>) -> WordRevelation{
        let mut iter = v.into_iter();

        WordRevelation(
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
        )
    }
}

impl FromIterator<LetterRevelation> for WordRevelation {
    fn from_iter<T: IntoIterator<Item = LetterRevelation>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        Self(
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
            iter.next().expect("Iterator provided fewer than 5 items"),
        )
    }
}

impl IntoIterator for WordRevelation {
    type Item = LetterRevelation;
    type IntoIter = std::array::IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1, self.2, self.3, self.4].into_iter()
    }
}

impl<'a> IntoIterator for &'a WordRevelation {
    type Item = &'a LetterRevelation;
    type IntoIter = std::array::IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.0, &self.1, &self.2, &self.3, &self.4].into_iter()
    }
}
