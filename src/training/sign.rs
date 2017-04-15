
use std::io::Write;

use termion;

use training::sequence::TypingSequence;
use training::format::TermFormat;


pub struct Pos {
    pub x: u16,
    pub y: u16
}


pub trait PosToTermConverter {
    fn term_pos(&self) -> termion::cursor::Goto;
}

impl PosToTermConverter for Pos {
    fn term_pos(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x, self.y)
    }
}


pub trait SignPrinter<T> where T: Write {
    fn show(&self, output: &mut T, coord: &Pos);
}

impl<T> SignPrinter<T> for TypingSequence where T: Write {
    fn show(&self, output: &mut T, coord: &Pos) {
        writeln!(output, "{}{}",
                 coord.term_pos(),
                 self.colorized()
        ).unwrap();
    }
}
