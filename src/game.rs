use std::io::{Read, Write};

use termion::event::Key;
use termion::input::TermRead;

use training::sequence::TypingSequence;
use training::sign::{Pos, SignPrinter};

macro_rules! flush {
    ($output:expr) => { $output.flush().unwrap(); }
}


pub enum Status {
    Aborted,
    Completed
}

pub fn exercise<F, R: Read, W: Write>(
            seq: &mut TypingSequence,
            input_provider: F,
            output: &mut W,
            coord: &Pos
        ) -> Status
        where F: Fn() -> R  {
    let mut output = output;
    let pos = coord;

    'step: while !seq.is_completed() {
        let current = seq.curr_ref().unwrap().code;

        seq.show(&mut output, &pos);
        flush!(output);

        let input = input_provider();
        'input: for c in input.keys() {
            match c.unwrap() {
                Key::Esc => { break 'step }
                Key::Char(c) if c == current => {
                    seq.pass();
                    seq.forward();
                    break 'input;
                }
                Key::Char(_) => { seq.miss(); }
                _ => {}
            };

            seq.show(&mut output, &pos);
            flush!(output);
        }
    }

    if seq.is_completed() { Status::Completed } else { Status::Aborted }
}
