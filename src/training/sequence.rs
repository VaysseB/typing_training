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

impl TypingSequence {
    pub fn new(sentence: String) -> TypingSequence {
        let mut keys = Vec::new();
        for c in sentence.chars() {
            keys.push(key::Key { code: c, status: key::Status::Unvalidated });
        }
        TypingSequence { keys: keys }
    }

    pub fn key_ref(&self, i: usize) -> Option<&key::Key> {
        if i < self.keys.len() { Some(&self.keys[i]) } else { None }
    }

    pub fn key_ref_mut(&mut self, i: usize) -> Option<&mut key::Key> {
        if i < self.keys.len() { Some(&mut self.keys[i]) } else { None }
    }

    pub fn miss(&mut self, i: usize) {
        self.key_ref_mut(i).unwrap().status = key::Status::Missed;
    }

    pub fn pass(&mut self, i: usize) {
        self.key_ref_mut(i).unwrap().status = key::Status::Passed;
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}
