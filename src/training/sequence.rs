use std::ops::{Index, IndexMut};

pub mod key {
    pub enum Status {
        Unvalidated,
        Passed,
        Missed
    }

    pub struct Key {
        pub code: char,
        pub status: Status
    }
}

pub struct TypingSequence {
    pub keys: Vec<key::Key>
}

impl Index<usize> for TypingSequence {
    type Output = key::Key;

    fn index(&self, index: usize) -> &key::Key {
        &self.keys[index]
    }
}

impl IndexMut<usize> for TypingSequence {
    fn index_mut(&mut self, index: usize) -> &mut key::Key {
        &mut self.keys[index]
    }
}

impl TypingSequence {
    pub fn new(sentence: &String) -> TypingSequence {
        let mut keys = Vec::new();
        for c in sentence.chars() {
            keys.push(key::Key { code: c, status: key::Status::Unvalidated });
        }
        TypingSequence { keys: keys }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}
