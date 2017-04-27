use std::io;

use termion;

use training::positioning::{Pos, Constraint, Positioning};
use training::format::PosToTerm;
use training::print::WindowPrinter;


pub struct Ui {
    pub constraints: Constraint,
    pub items: Vec<usize>
}


impl Ui {
    pub fn new(c: Constraint, w: Vec<usize>) -> Ui {
        Ui { constraints: c, items: w }
    }

    pub fn clear_and_hide_cursor<W: io::Write>(&mut self, mut output: &mut W) -> io::Result<()> {
        // init setup
        try!(write!(output,
                    "{}{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    termion::cursor::Hide));
        output.flush()
    }

    pub fn reset_cursor<W: io::Write>(&mut self, mut output: &mut W) -> io::Result<()> {
        try!(write!(output, "{}{}",
               Pos { x: 1, y: self.height() }.term_pos(),
               termion::cursor::Show));
        output.flush()
    }

    pub fn refresh<W: io::Write>(&mut self, mut output: &mut W) -> io::Result<()> {
        try!(self.constraints.win.grown_uniform(1).write_rect(&mut output));
        output.flush()
    }

    pub fn height(&self) -> u16 {
        self.constraints.win.y + self.constraints.win.h + 1
    }

    pub fn do_layout(&self) -> Result<Vec<Pos>, String> {
        self.constraints.organise(&self.items)
    }
}
