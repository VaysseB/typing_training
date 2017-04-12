
use std::borrow::Borrow;

use termion;
use termion::color::{self};

use typing;
use typing::key::Status;


pub trait TypingFormat {
    fn highlight_display(&self) -> String;
}

impl TypingFormat for typing::Typing {
    fn highlight_display(&self) -> String {
        let mut repr = String::new();

        // naive solution
        for (_, key) in self.sentence.iter().enumerate() {
            let color : String = match key.status {
                Status::Unvalidated => format!("{}{}", color::Bg(color::Reset), key.code),
                Status::Missed => format!("{}{}", color::Bg(color::LightRed), key.code),
                Status::Passed => format!("{}{}", color::Bg(color::Green), key.code)
            };
            repr.push_str(color.borrow());
        }

        // reset after for sanity
        repr.push_str(format!("{}", color::Bg(color::Reset)).borrow());
        repr
    }
}
