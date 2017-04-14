use std::io::{Read, Write};

use termion::event::Key;
use termion::input::TermRead;

use training::sign::{TypingSign, SignPrinter};

macro_rules! flush {
    ($output:expr) => { $output.flush().unwrap(); }
}


pub enum Status {
    Aborted,
    Completed
}

pub fn exercise<F, R: Read, W: Write>(
            sign: &mut TypingSign,
            input_provider: F,
            output: &mut W
        ) -> Status
        where F: Fn() -> R  {
    let mut output = output;

    'step: while !sign.seq.is_completed() {
        let current = sign.seq.curr_ref().unwrap().code;

        sign.show(&mut output);
        flush!(output);

        let input = input_provider();
        'input: for c in input.keys() {
            match c.unwrap() {
                Key::Esc => { break 'step }
                Key::Char(c) if c == current => {
                    sign.seq.pass();
                    sign.seq.forward();
                    break 'input;
                }
                Key::Char(_) => { sign.seq.miss(); }
                _ => {}
            };

            sign.show(&mut output);
            flush!(output);
        }
    }

    if sign.seq.is_completed() { Status::Completed } else { Status::Aborted }
}
