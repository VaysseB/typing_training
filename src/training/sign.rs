
use std::io::Write;

use termion;

use training::sequence::TypingSequence;
use training::format::TermFormat;


pub struct Pos {
    pub x: u16,
    pub y: u16
}

pub struct TypingSign {
    pub seq: TypingSequence,
    pub coord: Pos
}

impl TypingSign {
    pub fn new(sequence: TypingSequence) -> TypingSign {
        TypingSign {
            seq: sequence,
            coord: Pos { x: 0, y: 0 }
        }
    }

    pub fn move_(&mut self, x: u16, y: u16) {
        self.coord = Pos{x: x, y: y};
    }
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
    fn show(&self, output: &mut T);
}

impl<T> SignPrinter<T> for TypingSign where T: Write {
    fn show(&self, output: &mut T) {
        writeln!(output, "{}{}",
                 self.coord.term_pos(),
                 self.seq.colorized()
        ).unwrap();
    }
}
