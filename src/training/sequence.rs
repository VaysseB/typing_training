
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
    pub keys: Vec<key::Key>,
    pub progress: usize
}

impl TypingSequence {
    pub fn new(sentence: String) -> TypingSequence {
        let mut instance = TypingSequence { keys: Vec::new(), progress: 0 };
        instance.reset(sentence);
        instance
    }

    pub fn reset(&mut self, sentence: String) {
        self.progress = 0;
        self.keys.clear();
        for c in sentence.chars() {
            let key = key::Key { code: c, status: key::Status::Unvalidated };
            self.keys.push(key);
        }
    }

    pub fn curr_ref(&self) -> Option<&key::Key> {
        if self.progress < self.keys.len() { Some(&self.keys[self.progress]) } else { None }
    }

    pub fn forward(&mut self) {
        self.progress += 1;
    }

    pub fn is_completed(&self) -> bool {
        self.progress >= self.keys.len()
    }

    pub fn curr_ref_mut(&mut self) -> Option<&mut key::Key> {
        if self.progress < self.keys.len() { Some(&mut self.keys[self.progress]) } else { None }
    }

    pub fn miss(&mut self) {
        self.curr_ref_mut().unwrap().status = key::Status::Missed;
    }

    pub fn pass(&mut self) {
        self.curr_ref_mut().unwrap().status = key::Status::Passed;
    }
}
