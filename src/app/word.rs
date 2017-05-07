
//---
#[derive(Debug)]
pub struct Word {
    pub raw: String // TODO hide it
}

impl Word {
    pub fn new(s: &'static str) -> Word {
        Word { raw: s.to_string() }
    }
}


//---
#[derive(Debug)]
pub struct Bucket {
    pub words: Vec<Word> // TODO hide it
}

impl Bucket {
    pub fn new(w: Vec<&'static str>) -> Bucket {
        Bucket {
            words: w.iter()
                .map(|w: &&'static str| Word::new(w))
                .collect::<Vec<Word>>()
        }
    }
}
