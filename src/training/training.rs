
use std::io;
use std::io::{Read, Write};

use termion::event::Key;
use termion::input::TermRead;

use training::sequence::{TypingSequence, key};
use training::positioning::Pos;
use training::print::SequencePrinter;


// This is a brilliant macro, such time savior
macro_rules! flush {
    ($output:expr) => { $output.flush() }
}


pub enum Ending {
    Aborted,
    Completed
}


pub struct Training {
    pub pieces: Vec<(TypingSequence, Pos)>
}


impl Training {

    pub fn new<T, R>(words: T, plan: R) -> Training
        where T: Iterator<Item=String>, R: Iterator<Item=Pos> {
        let pieces = words
            .map(|w| TypingSequence::new(&w) )
            .zip(plan)
            .collect();
        Training { pieces: pieces }
    }

    pub fn play<F, R: Read, W: Write>(&mut self, input_provider: &F, mut output: &mut W) -> io::Result<Ending>
        where F: Fn() -> R {
        for ref mut pair in self.pieces.iter_mut() {
            let mut typing : &mut TypingSequence = &mut pair.0;
            let pos : &Pos = &pair.1;

            let mut exercise = Exercise::new(&mut typing, &pos);
            let status = exercise.play(input_provider, &mut output);
            match try!(status) {
                // TODO this may be misleading, find something better when possible
                Ending::Aborted => { return Ok(Ending::Aborted); },
                // TODO do something with this
                Ending::Completed => ()
            }
        }

        Ok(Ending::Completed)
    }

    pub fn write_all<W: Write>(&self, mut output: &mut W) -> io::Result<()> {
        let not_the_current = usize::max_value();
        for &(ref typing, ref pos) in self.pieces.iter() {
            try!(typing.write_seq(&mut output, not_the_current, &pos));
        }
        flush!(output)
    }

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


    fn update_cursor_pos<W: Write>(&self, mut output: &mut W) -> io::Result<()> {
        use training::format::PosToTerm;
        let cpos = Pos { x: self.pos.x + self.curr as u16, y: self.pos.y };
        write!(output, "{}", cpos.term_pos())
    }


    pub fn play<F, R: Read, W: Write>(&mut self, input_provider: F, mut output: &mut W) -> io::Result<Ending>
        where F: Fn() -> R {

        'step: while !self.is_done() {
            let current = self.subject[self.curr].code;

            try!(self.subject.write_seq(&mut output, self.curr, &self.pos));
            try!(self.update_cursor_pos(&mut output));
            try!(flush!(output));

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
                    _ => ()
                };

                try!(self.subject.write_seq(&mut output, self.curr, &self.pos));
                try!(self.update_cursor_pos(&mut output));
                try!(flush!(output));
            }
        }

        // last update after completion
        try!(self.subject.write_seq(&mut output, self.curr, &self.pos));
        try!(flush!(output));

        if self.is_done() { Ok(Ending::Completed) } else { Ok(Ending::Aborted) }
    }
}

