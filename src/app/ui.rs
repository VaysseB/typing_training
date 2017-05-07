use app::word::Bucket;


//---
#[allow(dead_code)]
#[derive(Debug)]
pub enum HAlignment {
    AlignLeft,
    AlignMiddle,
    AlignRight
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum VAlignment {
    AlignTop,
    AlignCenter,
    AlignBottom
}

#[derive(Debug)]
pub struct Alignment {
    vert: VAlignment,
    hori: HAlignment
}

impl Alignment {
    pub fn centered() -> Alignment {
        Alignment {
            vert: VAlignment::AlignCenter,
            hori: HAlignment::AlignMiddle
        }
    }
}


//---
#[allow(dead_code)]
#[derive(Debug)]
pub enum Measurement<T> {
    Value(T),
    Infinite
}


//---
#[derive(Debug)]
pub struct AdaptativeDim {
    pub width: Measurement<u16>,
    pub height: Measurement<u16>
}


//---
#[derive(Debug)]
pub struct Constraint {
    pub dim: AdaptativeDim,
    pub align: Alignment
}

impl Constraint {
    pub fn new(f: AdaptativeDim, alg: Alignment) -> Constraint {
        Constraint { dim: f, align: alg }
    }
}


//---
#[derive(Debug, PartialEq)]
pub struct Pos {
    pub x: u16, // TODO hide it
    pub y: u16  // TODO hide it
}

#[derive(Debug)]
pub struct Dim {
    pub width: u16, // TODO hide it
    pub height: u16 // TODO hide it
}

impl Into<AdaptativeDim> for Dim {
    fn into(self) -> AdaptativeDim {
        AdaptativeDim {
            width: Measurement::Value(self.width),
            height: Measurement::Value(self.height)
        }
    }
}


//---
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum LayoutError {
    // index of the word which overflows
    TooWide(usize),
    // index of the word which overflows
    TooManyWords(usize)
}

impl Constraint {
    fn organize(&self, bucket: &Bucket) -> Result<Vec<Pos>, LayoutError> {
        if bucket.words.is_empty() { return Ok(Vec::new()) }

        // the position system within the terminal starts at (1, 1)
        // This is really annoying, because I handle position starting at 0 instead of 1.
        // This algorithm use `first` as an offset, to still work within [0,n] range.
        let first = Pos { x: 1, y: 1 };
        let sep: u16 = 1;
        let mut planning = Vec::new();
        let mut last_len: u16 = 0;
        let mut start_the_row = true;

        for (i, word) in bucket.words.iter().enumerate() {
            let len = (*word).raw.len() as u16;
            let (gap, start_x, start_y): (u16, _, _);

            {
                let last_pos = planning.last().unwrap_or(&first);
                gap = if start_the_row { 0 } else { sep };
                start_x = last_pos.x + last_len + gap;
                start_y = last_pos.y;
            }

            // check if this fit horizontally
            let pos = match self.dim.width {
                // if the word itself is too wide for the constraint
                Measurement::Value(frame_width) if len > frame_width => {
                    return Err(LayoutError::TooWide(i))
                }
                // if the word fit following the last word in the same row
                Measurement::Infinite => {
                    Pos {
                        x: start_x,
                        y: start_y
                    }
                }
                // if the word fit following the last word in the same row
                Measurement::Value(frame_width) if start_x + len - first.x <= frame_width => {
                    Pos {
                        x: start_x,
                        y: start_y
                    }
                }
                // if the word make the current row overflows
                Measurement::Value(_) => {
                    // check if this fit vertically
                    match self.dim.height {
                        // if the new row overflows the constraint
                        Measurement::Value(frame_height) if start_y + 1 - first.y >= frame_height => {
                            return Err(LayoutError::TooManyWords(i))
                        }
                        // the word is now the starter of a new row
                        Measurement::Value(_) | Measurement::Infinite => {
                            Pos {
                                x: 1,
                                y: start_y + 1
                            }
                        }
                    }
                }
            };

            start_the_row = pos.y != start_y;
            last_len = len;
            planning.push(pos);
        }

        Ok(planning)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn word_overflow_frame_width() {
        use super::*;
        let enough_height_for_all = 1;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Value(enough_height_for_all as u16),
                width: Measurement::Value(5 as u16)
            },
            align: Alignment::centered() // not relevant
        };
        let bucket = Bucket::new(vec!["larger"]);
        let index_of_word_larger = 0;
        assert_eq!(c.organize(&bucket), Err(LayoutError::TooWide(index_of_word_larger)));
    }

    #[test]
    fn word_overflow_frame_height() {
        use super::*;
        let enough_width_for_all = 10;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Value(1 as u16),
                width: Measurement::Value(enough_width_for_all as u16)
            },
            align: Alignment::centered() // not relevant
        };
        let bucket = Bucket::new(vec!["fit", "stalker"]);
        let index_of_word_stalker = 1;
        assert_eq!(c.organize(&bucket), Err(LayoutError::TooManyWords(index_of_word_stalker)));
    }

    #[test]
    fn perfect_fit() {
        use super::*;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Value(2 as u16),
                width: Measurement::Value(12 as u16)
            },
            align: Alignment::centered() // not relevant
        };
        let bucket = Bucket::new(vec!["first", "second", "third"]);
        let positions = vec![Pos { x: 1, y: 1 }, Pos { x: 7, y: 1 }, Pos { x: 1, y: 2 }];
        assert_eq!(c.organize(&bucket), Ok(positions));
    }

    #[test]
    fn keep_on_one_line() {
        use super::*;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Value(1 as u16), // not relevant as long as not null
                width: Measurement::Infinite
            },
            align: Alignment::centered() // not relevant
        };
        let bucket = Bucket::new(vec!["first", "second", "third"]);
        let positions = vec![Pos { x: 1, y: 1 }, Pos { x: 7, y: 1 }, Pos { x: 14, y: 1 }];
        assert_eq!(c.organize(&bucket), Ok(positions));
    }

    #[test]
    fn auto_add_rows() {
        use super::*;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Infinite,
                width: Measurement::Value(6 as u16) // not relevant as long as minimal word len
            },
            align: Alignment::centered() // not relevant
        };
        let bucket = Bucket::new(vec!["first", "second", "third"]);
        let positions = vec![Pos { x: 1, y: 1 }, Pos { x: 1, y: 2 }, Pos { x: 1, y: 3 }];
        assert_eq!(c.organize(&bucket), Ok(positions));
    }
}


//---
#[derive(Debug)]
pub struct Layout {
    pub positions: Vec<Pos> // TODO hide it
}

pub fn layout(constraint: &Constraint, bucket: &Bucket) -> Result<Layout, LayoutError> {
    Ok(Layout {
        positions: try!(constraint.organize(&bucket))
    })
}


//---
#[macro_export]
macro_rules! write_iter {
    (&mut $dst:ident, $fmt:expr, $iter_a:expr) => {{
        let mut res = Ok(());
        for ref a in $iter_a.iter() {
            match write!(&mut $dst, $fmt, &a) {
                Ok(_) => (),
                Err(w) => { res = Err(w); break; }
            }
        }
        res
    }};

    (&mut $dst:ident, $fmt:expr, $iter_a:expr, $iter_b:expr) => {{
        let mut res = Ok(());
        for (ref a, ref b) in $iter_a.iter().zip($iter_b.iter()) {
            match write!(&mut $dst, $fmt, &a, &b) {
                Ok(_) => (),
                Err(w) => { res = Err(w); break; }
            }
        }
        res
    }};
}
