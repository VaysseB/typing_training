use std::io;

use termion;

use training::positioning::{Pos, Constraint, Window, Positioning, PostLayoutAction};
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

    fn post_actions(&mut self, poses: Vec<Pos>, bounding_box: &Window) -> Vec<Pos> {
        match self.constraints.action {
            Some(ref action) => {
                match action {
                    &PostLayoutAction::ShrinkEmptySpaces(gapx, gapy) => {
                        self.constraints.win = bounding_box.grown_sym(gapx, gapy);
                    }
                }
            }
            None => ()
        }
        poses
    }

    pub fn do_layout(&mut self) -> Result<Vec<Pos>, String> {
        match self.constraints.organise(&self.items) {
            Ok((poses, win)) => Ok(self.post_actions(poses, &win)),
            Err(s) => Err(s)
        }
    }
}
