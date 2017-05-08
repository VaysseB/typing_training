
use std::fmt;
use std::ops::Index;


//---
#[derive(Debug)]
pub struct Word {
    pub raw: String // TODO keep it only visible inside the crate
}

impl Word {
    pub fn new(s: &'static str) -> Word {
        Word { raw: s.to_string() }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.raw)
    }
}


//---
#[derive(Debug)]
pub struct Bucket {
    pub words: Vec<Word> // TODO keep it only visible inside the crate
}

impl Bucket {
    pub fn new(w: Vec<&'static str>) -> Bucket {
        Bucket {
            words: w.iter()
                .map(|w: &&'static str| Word::new(w))
                .collect::<Vec<Word>>()
        }
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }
}

impl Index<usize> for Bucket {
    type Output = Word;

    fn index(&self, i: usize) -> &Word {
        &self.words[i]
    }
}
