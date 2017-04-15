use std::iter::Iterator;

enum PositioningError {
    WordIsTooWide(String),
    // the first string that doesn't fit, horizontal constraint
    TooManyToFit(usize) // number of the first element that overflows, vertical constraint
}

#[derive(Copy, Clone)]
pub struct Window {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16
}

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: u16,
    pub y: u16
}

pub enum HAlignment {
    AlignLeft
}

pub enum VAlignment {
    AlignTop
}

struct DetailMeasurement {
    len: usize,
    sizes: Vec<usize>
}

pub struct Constraint {
    pub win: Window,
    pub infinite_height: bool,
    pub h_align: HAlignment,
    pub v_align: VAlignment
}

impl Constraint {
    fn split_rows(&self, words: &Vec<String>, sep: usize)
                  -> Result<Vec<DetailMeasurement>, PositioningError> {
        let mut rows = Vec::new();
        let mut measure = DetailMeasurement { len: 0, sizes: Vec::new() };
        for (i, word) in words.iter().enumerate() {
            let len = word.len();

            // check if this fit horizontally
            if len >= (self.win.w as usize) { return Err(PositioningError::WordIsTooWide(word.clone())) }

            let new_len = measure.len + len + (if measure.len == 0 { 0 } else { sep });

            // if the word fit the current row
            if new_len < (self.win.w as usize) {
                measure.sizes.push(measure.len);
                measure.len = new_len;
            } else { // if it has to be shifted to the next row
                rows.push(measure);
                measure = DetailMeasurement { len: 0, sizes: Vec::new() };

                // check if this fit vertically
                if rows.len() >= (self.win.h as usize) { return Err(PositioningError::TooManyToFit(i)) }

                measure.sizes.push(measure.len);
                measure.len = word.len();
            }
        }

        // last part to add
        if measure.len > 0 {
            rows.push(measure);
        }

        Ok(rows)
    }
}

pub trait Positioning {
    fn organise(&self, words: &Vec<String>) -> Result<Vec<Pos>, String>;
}

impl Positioning for Constraint {
    // the planning fails if any word doesn't fit into the window
    fn organise(&self, words: &Vec<String>) -> Result<Vec<Pos>, String> {
        let gap: usize = 1;
        match self.split_rows(words, gap) {
            Err(kind) => {
                match kind {
                    PositioningError::WordIsTooWide(word) =>
                        Err(format!("too wide to fit '{}'", word)),
                    PositioningError::TooManyToFit(index) =>
                        Err(format!("too many word, overflow from the {}th", index))
                }
            },
            Ok(rows) => {
                let mut planning : Vec<Pos> = Vec::new();

                for (dy, measure) in rows.iter().enumerate() {
                    for dx in measure.sizes.iter() {
                        planning.push(Pos {
                            x: self.win.x + (*dx as u16),
                            y: self.win.y + (dy as u16)
                        });
                    }
                }

                Ok(planning)
            }
        }
    }
}
