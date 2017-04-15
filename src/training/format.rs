use std::borrow::Borrow;

use termion::color::{self};

use training::sequence::TypingSequence;
use training::sequence::key::Status;


pub trait TermFormat {
    fn color_of(status: &Status, is_curr: bool) -> String;

    fn colorized(&self, current: usize) -> String;
}

impl TermFormat for TypingSequence {
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
