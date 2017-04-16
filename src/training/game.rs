
use std::io;
use std::io::{Read, Write};

use termion::event::Key;
use termion::input::TermRead;

use training::sequence::{TypingSequence, key};
use training::positioning::Pos;
use training::print::SequencePrinter;

macro_rules! flush {
    ($output:expr) => { $output.flush() }
}


pub enum Ending {
    Aborted,
    Completed
}

pub struct Exercise<'a> {
    subject: &'a mut TypingSequence,
    pos: &'a Pos,
    curr: usize
}

impl<'a> Exercise<'a> {
    pub fn new(seq: &'a mut TypingSequence, pos: &'a Pos) -> Exercise<'a> {
        Exercise { subject: seq, pos: pos, curr: 0 }
    }

    fn is_done(&self) -> bool {
        self.curr >= self.subject.len()
    }

    fn update_cursor_pos<W: Write>(&self, output: &mut W) -> io::Result<()> {
        use training::format::PosToTerm;
        let mut output = output;
        let cpos = Pos { x: self.pos.x + self.curr as u16, y: self.pos.y };
        write!(output, "{}", cpos.term_pos())
    }

    pub fn play<F, R: Read, W: Write>(&mut self, input_provider: F, output: &mut W) -> Ending
        where F: Fn() -> R {
        let mut output = output;

        'step: while !self.is_done() {
            let current = self.subject[self.curr].code;

            self.subject.write_seq(&mut output, self.curr, &self.pos).unwrap();
            self.update_cursor_pos(&mut output).unwrap();
            flush!(output).unwrap();

            let input = input_provider();
            'input: for c in input.keys() {
                match c.unwrap() {
                    Key::Esc => { break 'step }
                    Key::Char(c) if c == current => {
                        self.subject[self.curr].status = key::Status::Passed;
                        self.curr += 1;
                        break 'input;
                    }
                    Key::Char(_) => {
                        self.subject[self.curr].status = key::Status::Missed;
                    }
                    _ => {}
                };

                self.subject.write_seq(&mut output, self.curr, &self.pos).unwrap();
                self.update_cursor_pos(&mut output).unwrap();
                flush!(output).unwrap();
            }
        }

        if self.is_done() { Ending::Completed } else { Ending::Aborted }
    }
}

