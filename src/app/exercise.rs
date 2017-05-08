
use std::ops::Index;

use app::word::Word;



pub fn new(word: &Word) -> Exercise {
    Exercise(word.raw.chars().collect())
}


//---
pub struct Exercise(Vec<char>);

impl Exercise {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Exercise {
    type Output = char;

    fn index(&self, i: usize) -> &char {
        &self.0[i]
    }
}
