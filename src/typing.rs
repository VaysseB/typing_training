
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

pub struct Typing {
    pub sentence: Vec<key::Key>,
    pub current: usize,
    pub raw: String
}

impl Typing {
    pub fn new(sentence: String) -> Typing {
        let mut instance = Typing { sentence: Vec::new(), current: 0, raw: String::new() };
        instance.reset(sentence);
        instance
    }

    pub fn reset(&mut self, sentence: String) {
        self.raw.clone_from(&sentence);
        self.current = 0;
        self.sentence.clear();
        for c in sentence.chars() {
            let key = key::Key { code: c, status: key::Status::Unvalidated };
            self.sentence.push(key);
        }
    }

    pub fn curr_ref(&self) -> Option<&key::Key> {
        if self.current < self.sentence.len() { Some(&self.sentence[self.current]) } else { None }
    }

    pub fn forward(&mut self) {
        self.current += self.curr_ref().unwrap().code.len_utf16();
    }

    fn curr_ref_mut(&mut self) -> Option<&mut key::Key> {
        if self.current < self.sentence.len() { Some(&mut self.sentence[self.current]) } else { None }
    }

    pub fn miss(&mut self) {
        self.curr_ref_mut().unwrap().status = key::Status::Missed;
    }

    pub fn pass(&mut self) {
        self.curr_ref_mut().unwrap().status = key::Status::Passed;
    }
}

pub fn new(raw: &'static str) -> Typing {
    Typing::new(raw.to_string())
}
