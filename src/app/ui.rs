use std::cmp::max;


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

    pub fn top_left() -> Alignment {
        Alignment {
            vert: VAlignment::AlignTop,
            hori: HAlignment::AlignLeft
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
    // TODO keep it only visible inside the crate
    pub x: u16,
    // TODO keep it only visible inside the crate
    pub y: u16
}

#[derive(Debug, PartialEq)]
pub struct Dim {
    // TODO keep it only visible inside the crate
    pub w: u16,
    // TODO keep it only visible inside the crate
    pub h: u16
}

impl Into<AdaptativeDim> for Dim {
    fn into(self) -> AdaptativeDim {
        AdaptativeDim {
            width: Measurement::Value(self.w),
            height: Measurement::Value(self.h)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BoundingBox {
    // TODO keep it only visible inside the crate
    pub x: u16,
    // TODO keep it only visible inside the crate
    pub y: u16,
    // TODO keep it only visible inside the crate
    pub w: u16,
    // TODO keep it only visible inside the crate
    pub h: u16
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
    pub fn organize(&self, bucket: &Bucket) -> Result<(Vec<Pos>, BoundingBox), LayoutError> {
        // the position system within the terminal starts at (1, 1)
        // This is really annoying, because I handle position starting at 0 instead of 1.
        // This algorithm use `first` as an offset, to still work within [0,n] range.
        // TODO get it from somewhere else because it is bound to terminal's position coordinates
        let origin = Pos { x: 1, y: 1 };

        let (mut poss, surface, rows) = try!(self.split_roughly(&bucket, &origin));
        let bbox = self.align(&mut poss, &origin, &surface, &rows);

        Ok((poss, bbox))
    }

    fn align(&self, rough_pos: &mut Vec<Pos>, origin: &Pos, area_size: &Dim, rows_length: &Vec<u16>) -> BoundingBox {
        let mut bbox = if rough_pos.len() == 0 {
            BoundingBox { x: origin.x, y: origin.y, w: 0, h: 0 }
        } else {
            BoundingBox { x: u16::max_value(), y: u16::max_value(), w: 0, h: 0 }
        };

        let offset_y = match self.dim.height {
            Measurement::Infinite => 0,
            Measurement::Value(height) => {
                match self.align.vert {
                    VAlignment::AlignTop => 0,
                    VAlignment::AlignCenter => (height - area_size.h) / 2,
                    VAlignment::AlignBottom => height - area_size.h
                }
            }
        };
        bbox.y = offset_y;

        for ref mut pos in rough_pos.into_iter() {

            let row_length = rows_length[(pos.y - origin.y) as usize];

            let offset_x = match self.dim.width {
                Measurement::Infinite => 0,
                Measurement::Value(width) => {
                    match self.align.hori {
                        HAlignment::AlignLeft => 0,
                        HAlignment::AlignMiddle => (width - row_length) / 2,
                        HAlignment::AlignRight => width - row_length
                    }
                }
            };

            **pos = Pos {
                x: pos.x + offset_x,
                y: pos.y + offset_y
            };
        }

        bbox
    }

    fn split_roughly(&self, bucket: &Bucket, origin: &Pos) -> Result<(Vec<Pos>, Dim, Vec<u16>), LayoutError> {
        if bucket.words.is_empty() { return Ok((Vec::new(), Dim { w: 0, h: 0 }, Vec::new())); }

        let sep: u16 = 1;
        let mut planning: Vec<Pos> = Vec::new();
        let mut rows_length = Vec::new();
        let mut last_len: u16 = 0;
        let mut start_the_row = true;
        let mut right_side = 0;

        for (i, word) in bucket.words.iter().enumerate() {
            let len = (*word).raw.len() as u16;
            let (gap, start_x, start_y): (u16, _, _);

            {
                let last_pos = planning.last().unwrap_or(&origin);
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
                Measurement::Value(frame_width) if start_x + len - origin.x <= frame_width => {
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
                        Measurement::Value(frame_height) if start_y + 1 - origin.y >= frame_height => {
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

            if start_the_row {
                let previous_row_length = start_x + last_len;
                rows_length.push(previous_row_length);
                right_side = max(right_side, previous_row_length);
            }

            last_len = len;
            planning.push(pos);
        }

        let bottom_line;
        {
            let last_pos = planning.last().expect("not possible");
            rows_length.push(last_pos.x + last_len);
            bottom_line = last_pos.y;
        }

        Ok((planning, Dim { w: right_side, h: bottom_line }, rows_length))
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
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["larger"]);
        let index_of_word_larger = 0;
        assert_eq!(c.organize(&input_bucket), Err(LayoutError::TooWide(index_of_word_larger)));
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
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["fit", "stalker"]);
        let index_of_word_stalker = 1;
        assert_eq!(c.organize(&input_bucket), Err(LayoutError::TooManyWords(index_of_word_stalker)));
    }

    #[test]
    fn perfect_fit() {
        use super::*;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Value(2 as u16),
                width: Measurement::Value(12 as u16)
            },
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["first", "second", "third"]);
        let expected_positions = vec![Pos { x: 1, y: 1 }, Pos { x: 7, y: 1 }, Pos { x: 1, y: 2 }];
        let final_positions = c.organize(&input_bucket).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }

    #[test]
    fn keep_on_one_line() {
        use super::*;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Value(1 as u16),
                // not relevant as long as not null
                width: Measurement::Infinite
            },
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["first", "second", "third"]);
        let expected_positions = vec![Pos { x: 1, y: 1 }, Pos { x: 7, y: 1 }, Pos { x: 14, y: 1 }];
        let final_positions = c.organize(&input_bucket).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }

    #[test]
    fn auto_add_rows() {
        use super::*;
        let c = Constraint {
            dim: AdaptativeDim {
                height: Measurement::Infinite,
                width: Measurement::Value(6 as u16) // not relevant as long as minimal word len
            },
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["first", "second", "third"]);
        let expected_positions = vec![Pos { x: 1, y: 1 }, Pos { x: 1, y: 2 }, Pos { x: 1, y: 3 }];
        let final_positions = c.organize(&input_bucket).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }
}


//---
#[derive(Debug)]
pub struct Layout {
    pub frame: BoundingBox,
    // TODO keep it only visible inside the crate
    pub positions: Vec<Pos> // TODO keep it only visible inside the crate
}

pub fn layout(constraint: &Constraint, bucket: &Bucket) -> Result<Layout, LayoutError> {
    let (poses, bbox) = try!(constraint.organize(bucket));
    Ok(Layout {
        frame: bbox,
        positions: poses
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
