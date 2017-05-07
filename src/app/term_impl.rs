use std::fmt;
use std::io;

use termion;

use app::word::{Word};
use app::ui::{AdaptativeDim, Measurement, Pos, Dim};


//---
pub fn term_dim() -> Dim {
    let size = termion::terminal_size().expect("no size of terminal");
    Dim {
        height: size.1 - 2,
        width: size.0 - 1
    }
}


//---
impl<'a> Into<termion::cursor::Goto> for &'a Pos {
    fn into(self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x, self.y)
    }
}


//---
impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.raw)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.into::<termion::cursor::Goto>()) // WHY doesn't this work ?
        write!(f, "{}", Into::<termion::cursor::Goto>::into(self))
    }
}
