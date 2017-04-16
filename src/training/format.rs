use std::borrow::Borrow;

use termion;
use termion::color::{self};

use training::positioning::Pos;
use training::sequence::TypingSequence;
use training::sequence::key::Status;


pub trait PosToTerm {
    fn term_pos(&self) -> termion::cursor::Goto;
}

impl PosToTerm for Pos {
    fn term_pos(&self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x, self.y)
    }
}


pub trait SequenceFormat {
    fn color_of(status: &Status, is_curr: bool) -> String;

    fn colorized(&self, current: usize) -> String;
}

impl SequenceFormat for TypingSequence {
    fn color_of(status: &Status, is_curr: bool) -> String {
        match status {
            &Status::Unvalidated => {
                if is_curr { format!("{}", color::Bg(color::Magenta)) }
                else { format!("{}", color::Bg(color::Reset)) }
            },
            &Status::Missed => format!("{}", color::Bg(color::LightRed)),
            &Status::Passed => format!("{}", color::Bg(color::Green))
        }
    }

    fn colorized(&self, current: usize) -> String {
        let mut repr = String::new();

        // naive solution
        for (i, key) in self.keys.iter().enumerate() {
            let color = format!("{}{}", Self::color_of(&key.status, i == current), key.code);
            repr.push_str(color.borrow());
        }

        // reset after for sanity
        repr.push_str(format!("{}", color::Bg(color::Reset)).borrow());
        repr
    }
}
