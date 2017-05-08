use std::fmt;
use std::cmp::{max, min};

use termion;

use app::word::Bucket;

pub mod brush;


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
    #[allow(unused)]
    pub fn centered() -> Alignment {
        Alignment {
            vert: VAlignment::AlignCenter,
            hori: HAlignment::AlignMiddle
        }
    }

    #[allow(unused)]
    pub fn top_left() -> Alignment {
        Alignment {
            vert: VAlignment::AlignTop,
            hori: HAlignment::AlignLeft
        }
    }

    #[allow(unused)]
    pub fn bottom_right() -> Alignment {
        Alignment {
            vert: VAlignment::AlignBottom,
            hori: HAlignment::AlignRight
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
    pub origin: Pos,
    pub dim: AdaptativeDim,
    pub align: Alignment
}


//---
#[derive(Debug, PartialEq)]
pub struct Pos {
    // TODO keep it only visible inside the crate
    pub x: u16,
    // TODO keep it only visible inside the crate
    pub y: u16
}

impl Pos {
    pub fn shift(&self, incrx: i16, incry: i16) -> Pos {
        Pos {
            x: (self.x as i16 + incrx) as u16,
            y: (self.y as i16 + incry) as u16
        }
    }
}

impl<'a> Into<termion::cursor::Goto> for &'a Pos {
    fn into(self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x, self.y)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.into::<termion::cursor::Goto>()) // WHY doesn't this work ?
        write!(f, "{}", Into::<termion::cursor::Goto>::into(self))
    }
}


//---
#[derive(Debug, PartialEq)]
pub struct Dim {
    // TODO keep it only visible inside the crate
    pub w: u16,
    // TODO keep it only visible inside the crate
    pub h: u16
}

impl Dim {
    pub fn shrink(&self, incrw: u16, incrh: u16) -> Dim {
        Dim { w: self.w - incrw, h: self.h - incrh }
    }

    pub fn grow(&self, incrw: u16, incrh: u16) -> Dim {
        Dim { w: self.w + incrw, h: self.h + incrh }
    }
}

impl Into<AdaptativeDim> for Dim {
    fn into(self) -> AdaptativeDim {
        AdaptativeDim {
            width: Measurement::Value(self.w),
            height: Measurement::Value(self.h)
        }
    }
}

pub fn term_dim() -> Dim {
    let size = termion::terminal_size().expect("no size of terminal");
    Dim {
        h: size.1 - 2,
        w: size.0 - 1
    }
}


//---
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

impl BoundingBox {
    pub fn pos(&self) -> Pos {
        Pos { x: self.x, y: self.y }
    }

    pub fn dim(&self) -> Dim {
        Dim { w: self.w, h: self.h }
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
    pub fn organize(&self, bucket: &Bucket) -> Result<(Vec<Pos>, BoundingBox), LayoutError> {
        let (mut poss, surface, rows) = try!(self.split_roughly(&bucket));
        let bbox = self.align(&mut poss, &surface, &rows);
        Ok((poss, bbox))
    }

    fn align(&self, rough_pos: &mut Vec<Pos>, area_size: &Dim, rows_length: &Vec<u16>) -> BoundingBox {
        if rough_pos.len() == 0 {
            return BoundingBox { x: self.origin.x, y: self.origin.y, w: 0, h: 0 };
        }

        let mut bbox = BoundingBox { x: u16::max_value(), y: 0, w: 0, h: 0 };

        let offset_y = match self.dim.height {
            Measurement::Infinite => 0,
            Measurement::Value(height) => {
                debug_assert!(area_size.h <= height, format!("{} <= {}", area_size.h, height));
                match self.align.vert {
                    VAlignment::AlignTop => 0,
                    VAlignment::AlignCenter => (height - area_size.h) / 2,
                    VAlignment::AlignBottom => height - area_size.h
                }
            }
        };
        bbox.y = rough_pos.first().expect("not possible").y;
        bbox.h = rough_pos.last().expect("not possible").y - bbox.y + 1;
        bbox.y = bbox.y + offset_y;

        for ref mut pos in rough_pos.into_iter() {
            let row_length = rows_length[(pos.y - self.origin.y) as usize];

            bbox.w = max(bbox.w, row_length);

            let offset_x = match self.dim.width {
                Measurement::Infinite => 0,
                Measurement::Value(width) => {
                    debug_assert!(row_length <= width, format!("{} <= {}", row_length, width));
                    match self.align.hori {
                        HAlignment::AlignLeft => 0,
                        HAlignment::AlignMiddle => (width - row_length) / 2,
                        HAlignment::AlignRight => width - row_length
                    }
                }
            };

            bbox.x = min(bbox.x, pos.x + offset_x);

            **pos = Pos {
                x: pos.x + offset_x,
                y: pos.y + offset_y
            };
        }

        bbox
    }

    fn split_roughly(&self, bucket: &Bucket) -> Result<(Vec<Pos>, Dim, Vec<u16>), LayoutError> {
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
                let last_pos = planning.last().unwrap_or(&self.origin);
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
                Measurement::Value(frame_width) if start_x + len - self.origin.x <= frame_width => {
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
                        Measurement::Value(frame_height) if start_y + 1 - self.origin.y >= frame_height => {
                            return Err(LayoutError::TooManyWords(i))
                        }
                        // the word is now the starter of a new row
                        Measurement::Value(_) | Measurement::Infinite => {
                            Pos {
                                x: self.origin.x,
                                y: start_y + 1
                            }
                        }
                    }
                }
            };

            start_the_row = pos.y != start_y;

            if start_the_row {
                let previous_row_length = start_x - gap;
                rows_length.push(previous_row_length - self.origin.x);
                right_side = max(right_side, previous_row_length);
            }

            last_len = len;
            planning.push(pos);
        }

        let bottom_line;
        {
            let last_pos = planning.last().expect("not possible");
            rows_length.push(last_pos.x + last_len - self.origin.x);
            bottom_line = last_pos.y - self.origin.y;
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
            origin: Pos { x: 0, y: 0 },
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
            origin: Pos { x: 0, y: 0 },
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

        // fixed inputs
        let words = vec!["first", "second", "third"];
        let gap = 1;

        // deduced inputs
        let width = words[0].len() as u16 + gap + words[1].len() as u16;

        let c = Constraint {
            origin: Pos { x: 0, y: 0 },
            dim: AdaptativeDim {
                height: Measurement::Value(2 as u16),
                width: Measurement::Value(width as u16)
            },
            align: Alignment::top_left()
        };
        let expected_positions = vec![
            Pos { x: 0, y: 0 },
            Pos { x: words[0].len() as u16 + gap, y: 0 },
            Pos { x: 0, y: 1 }
        ];

        // test
        let final_positions = c.organize(&Bucket::new(words)).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }

    #[test]
    fn keep_on_one_line() {
        use super::*;
        let c = Constraint {
            origin: Pos { x: 0, y: 0 },
            dim: AdaptativeDim {
                height: Measurement::Value(1 as u16),
                // not relevant as long as not null
                width: Measurement::Infinite
            },
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["first", "second", "third"]);
        let expected_positions = vec![Pos { x: 0, y: 0 }, Pos { x: 6, y: 0 }, Pos { x: 13, y: 0 }];

        let final_positions = c.organize(&input_bucket).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }

    #[test]
    fn auto_add_rows() {
        use super::*;
        let c = Constraint {
            origin: Pos { x: 0, y: 0 },
            dim: AdaptativeDim {
                height: Measurement::Infinite,
                width: Measurement::Value(6 as u16) // not relevant as long as minimal word len
            },
            align: Alignment::top_left()
        };
        let input_bucket = Bucket::new(vec!["first", "second", "third"]);
        let expected_positions = vec![Pos { x: 0, y: 0 }, Pos { x: 0, y: 1 }, Pos { x: 0, y: 2 }];

        let final_positions = c.organize(&input_bucket).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }

    #[test]
    fn center_content() {
        use super::*;

        // fixed inputs
        let words = vec!["first", "second", "third"];
        let gap = 1;
        let offset_first_line: u16 = 2;

        // deduced inputs
        let width = offset_first_line + words[0].len() as u16 + gap + words[1].len() as u16 + offset_first_line;
        assert!(offset_first_line * 2 < words[2].len() as u16, "pre-condition failed");
        let offset_second_line: u16 = (width - words[2].len() as u16) / 2;

        let c = Constraint {
            origin: Pos { x: 0, y: 0 },
            dim: AdaptativeDim {
                height: Measurement::Infinite,
                width: Measurement::Value(width as u16)
            },
            align: Alignment::centered()
        };
        let expected_positions = vec![
            Pos { x: offset_first_line, y: 0 },
            Pos { x: offset_first_line + gap + words[0].len() as u16, y: 0 },
            Pos { x: offset_second_line, y: 1 }
        ];

        // test
        let final_positions = c.organize(&Bucket::new(words)).expect("positioning failed").0;
        assert_eq!(final_positions, expected_positions);
    }

    #[test]
    fn opposite_align() {
        use super::*;

        // fixed inputs
        let words = vec!["first", "second", "third"];
        let gap = 1;
        let offset_first_line: u16 = 2;

        // deduced inputs
        assert!(offset_first_line < words[2].len() as u16, "pre-condition failed");
        let width = offset_first_line + words[0].len() as u16 + gap + words[1].len() as u16;
        let offset_second_line: u16 = width - words[2].len() as u16;

        let c = Constraint {
            origin: Pos { x: 0, y: 0 },
            dim: AdaptativeDim {
                height: Measurement::Infinite,
                width: Measurement::Value(width as u16)
            },
            align: Alignment::bottom_right()
        };
        let expected_positions = vec![
            Pos { x: offset_first_line, y: 0 },
            Pos { x: offset_first_line + gap + words[0].len() as u16, y: 0 },
            Pos { x: offset_second_line, y: 1 }
        ];

        // test
        let final_positions = c.organize(&Bucket::new(words)).expect("positioning failed").0;
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

    debug_assert!(bbox.x >= constraint.origin.x,
    format!("post-condition failed on x ({} >= {})", bbox.x, constraint.origin.x));
    debug_assert!(bbox.y >= constraint.origin.y,
    format!("post-condition failed on y ({} >= {})", bbox.y, constraint.origin.y));
    match constraint.dim.width {
        Measurement::Value(w) => {
            debug_assert!(bbox.w <= w,
            format!("post-condition failed on w ({} <= {})", bbox.w, w));
        }
        _ => {}
    }
    match constraint.dim.height {
        Measurement::Value(h) => {
            debug_assert!(bbox.h <= h,
            format!("post-condition failed on h ({} <= {})", bbox.h, h));
        }
        _ => {}
    }

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
