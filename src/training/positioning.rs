use std::iter::Iterator;

enum PositioningError {
    // the size of the first string that doesn't fit, horizontal constraint
    WordIsTooWide(usize),

    // number of the first element that overflows, vertical constraint
    TooManyToFit(usize)
}

pub struct Pos {
    pub x: u16,
    pub y: u16
}

pub trait PosMovement {
    fn shifted_x(&self, dx: u16) -> Pos;
    fn shifted_y(&self, dy: u16) -> Pos;
}

impl PosMovement for Pos {
    fn shifted_x(&self, dx: u16) -> Pos { Pos { x: self.x + dx, y: self.y } }
    fn shifted_y(&self, dy: u16) -> Pos { Pos { x: self.x, y: self.y + dy } }
}

pub struct Window {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16
}

impl Window {
    pub fn grown_uniform(&self, incr: u16) -> Window {
        Window {
            x: self.x - incr,
            y: self.y - incr,
            w: self.w + incr * 2,
            h: self.h + incr * 2
        }
    }
}

pub trait WindowCorner {
    fn top_left(&self) -> Pos;
    fn top_right(&self) -> Pos;
    fn bottom_left(&self) -> Pos;
    fn bottom_right(&self) -> Pos;
}

impl WindowCorner for Window {
    fn top_left(&self) -> Pos { Pos { x: self.x, y: self.y } }
    fn top_right(&self) -> Pos { Pos { x: self.x + self.w - 1, y: self.y } }
    fn bottom_left(&self) -> Pos { Pos { x: self.x, y: self.y + self.h - 1 } }
    fn bottom_right(&self) -> Pos { Pos { x: self.x + self.w - 1, y: self.y + self.h - 1 } }
}

#[allow(dead_code)]
pub enum HAlignment {
    AlignLeft,
    AlignMiddle,
    AlignRight
}

#[allow(dead_code)]
pub enum VAlignment {
    AlignTop,
    AlignCenter,
    AlignBottom
}

pub struct Constraint {
    pub win: Window,
    pub infinite_height: bool,
    pub h_align: HAlignment,
    pub v_align: VAlignment
}

struct DetailMeasurement {
    len: usize,
    sizes: Vec<usize> // asc size of items
}

impl Constraint {
    fn split_rows(&self, words: &Vec<usize>, sep: usize)
                  -> Result<Vec<DetailMeasurement>, PositioningError> {
        let mut rows = Vec::new();
        let mut measure = DetailMeasurement { len: 0, sizes: Vec::new() };

        for (i, len) in words.iter().enumerate() {
            let len : usize = *len;

            // check if this fit horizontally
            if len >= (self.win.w as usize) { return Err(PositioningError::WordIsTooWide(len)) }

            let gap = if measure.len == 0 { 0 } else { sep };
            let new_len = measure.len + gap + len;

            // if the additional word fit the current row
            if new_len < (self.win.w as usize) {
                measure.sizes.push(measure.len + gap);
                measure.len = new_len;
            } else { // if it has to be shifted to the next row
                rows.push(measure);
                measure = DetailMeasurement { len: 0, sizes: Vec::new() };

                // check if this fit vertically
                if rows.len() >= (self.win.h as usize) { return Err(PositioningError::TooManyToFit(i)) }

                measure.sizes.push(measure.len);
                measure.len = len;
            }
        }

        // last part to add
        rows.push(measure);

        Ok(rows)
    }
}

pub trait Positioning {
    fn organise(&self, words: &Vec<usize>) -> Result<Vec<Pos>, String>;
}

impl Positioning for Constraint {
    // the planning fails if any word doesn't fit into the window
    fn organise(&self, words: &Vec<usize>) -> Result<Vec<Pos>, String> {
        let gap: usize = 1;
        match self.split_rows(words, gap) {
            Err(kind) => {
                match kind {
                    PositioningError::WordIsTooWide(len) =>
                        Err(format!("too narrow to fit a word of '{}' length", len)),
                    PositioningError::TooManyToFit(index) =>
                        Err(format!("too many word, overflow from the {}th", index))
                }
            },
            Ok(rows) => {
                let mut planning : Vec<Pos> = Vec::new();

                let start_y = match self.v_align {
                    VAlignment::AlignTop => 0,
                    VAlignment::AlignCenter => (self.win.h - (rows.len() as u16)) / 2,
                    VAlignment::AlignBottom => self.win.h - (rows.len() as u16)
                };

                for (dy, measure) in rows.iter().enumerate() {
                    let start_x = match self.h_align {
                        HAlignment::AlignLeft => 0,
                        HAlignment::AlignMiddle => (self.win.w - (measure.len as u16)) / 2,
                        HAlignment::AlignRight => self.win.w - (measure.len as u16)
                    };

                    for dx in measure.sizes.iter() {
                        planning.push(Pos {
                            x: self.win.x + start_x + (*dx as u16),
                            y: self.win.y + start_y + (dy as u16)
                        });
                    }
                }

                Ok(planning)
            }
        }
    }
}
