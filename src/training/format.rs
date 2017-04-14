use std::borrow::Borrow;

use termion::color::{self};

use training::sequence::TypingSequence;
use training::sequence::key::Status;


pub trait TermFormat {
    fn colored_repr(&self) -> String;
}

impl TermFormat for TypingSequence {
    fn colored_repr(&self) -> String {
        let mut repr = String::new();

        // naive solution
        for (i, key) in self.keys.iter().enumerate() {
            let is_current = i == self.progress;
            let color = if is_current {
                match key.status {
                    Status::Unvalidated => format!("{}{}", color::Bg(color::Magenta), key.code),
                    Status::Missed => format!("{}{}", color::Bg(color::LightRed), key.code),
                    Status::Passed => format!("{}{}", color::Bg(color::Green), key.code)
                }
            } else {
                match key.status {
                    Status::Unvalidated => format!("{}{}", color::Bg(color::Reset), key.code),
                    Status::Missed => format!("{}{}", color::Bg(color::LightRed), key.code),
                    Status::Passed => format!("{}{}", color::Bg(color::Green), key.code)
                }
            };
            repr.push_str(color.borrow());
        }

        // reset after for sanity
        repr.push_str(format!("{}", color::Bg(color::Reset)).borrow());
        repr
    }
}
