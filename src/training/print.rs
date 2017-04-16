use std::io;
use std::io::Write;

use training::sequence::TypingSequence;
use training::format::{PosToTerm, SequenceFormat};
use training::positioning::{Pos, PosMovement, Window, WindowCorner};


// taken from python3.6/curses/textpad.py
pub static ACS_TLCORNER: char = '┌';
pub static ACS_BLCORNER: char = '└';
pub static ACS_TRCORNER: char = '┐';
pub static ACS_BRCORNER: char = '┘';
pub static ACS_HLINE: char = '─';
pub static ACS_VLINE: char = '│';


pub trait PosPrinter<T>
    where T: Write {

    fn write_hline(&self, keycode: char, w: u16, output: &mut T) -> io::Result<()>;

    fn write_vline(&self, keycode: char, h: u16, output: &mut T) -> io::Result<()>;

    fn write_at(&self, keycode: char, output: &mut T) -> io::Result<()>;
}


impl<T> PosPrinter<T> for Pos
    where T: Write {

    fn write_hline(&self, keycode: char, w: u16, output: &mut T) -> io::Result<()> {
        let line = keycode.to_string().repeat(w as usize);
        try!(writeln!(output, "{}{}", self.term_pos(), line));
        Ok(())
    }

    fn write_vline(&self, keycode: char, h: u16, output: &mut T) -> io::Result<()> {
        for dy in 0..h {
            let pos = Pos { x: self.x, y: self.y + dy };
            try!(writeln!(output, "{}{}", pos.term_pos(), keycode));
        }
        Ok(())
    }

    fn write_at(&self, keycode: char, output: &mut T) -> io::Result<()> {
        try!(writeln!(output, "{}{}", self.term_pos(), keycode));
        Ok(())
    }
}


pub trait WindowPrinter<T>
    where T: Write {

    fn write_rect(&self, output: &mut T) -> io::Result<()>;
}


impl<T> WindowPrinter<T> for Window
    where T: Write {

    fn write_rect(&self, output: &mut T) -> io::Result<()> {
        try!(self.top_left().write_at(ACS_TLCORNER, output));
        try!(self.top_right().write_at(ACS_TRCORNER, output));
        try!(self.bottom_left().write_at(ACS_BLCORNER, output));
        try!(self.bottom_right().write_at(ACS_BRCORNER, output));
        try!(self.top_left().shifted_x(1).write_hline(ACS_HLINE, self.w - 2, output));
        try!(self.bottom_left().shifted_x(1).write_hline(ACS_HLINE, self.w - 2, output));
        try!(self.top_left().shifted_y(1).write_vline(ACS_VLINE, self.h - 2, output));
        try!(self.top_right().shifted_y(1).write_vline(ACS_VLINE, self.h - 2, output));
        Ok(())
    }
}


pub trait SequencePrinter<T>
    where T: Write {

    fn write_seq(&self, output: &mut T, current: usize, coord: &Pos);
}

impl<T> SequencePrinter<T> for TypingSequence
    where T: Write {

    fn write_seq(&self, output: &mut T, current: usize, coord: &Pos) {
        write!(output, "{}{}",
               coord.term_pos(),
               self.colorized(current)
        ).unwrap();
    }
}
