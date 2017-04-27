
use std::io;
use std::ops::IndexMut;
use std::rc::{Rc, Weak};
use std::borrow::Borrow;

use termion::event;

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


pub enum TypingInput {
    Right,
    Wrong,
    Unhandled
}


pub struct Training {
    pub pieces: Box<[(Rc<TypingSequence>, Rc<Pos>)]>,
    pub next_at: usize,
    pub exercise: Option<Exercise>
}


impl Training {

    pub fn new<T, R>(words: T, plan: R) -> Training
        where T: Iterator<Item=String>, R: Iterator<Item=Pos> {
        let pieces : Vec<(Rc<TypingSequence>, Rc<Pos>)> = words
            .map(|w| Rc::new(TypingSequence::new(&w)) )
            .zip(plan.map(|p| Rc::new(p)))
            .collect();

        Training { pieces: pieces.into_boxed_slice(), next_at: 0, exercise: None }
    }

    pub fn next(&mut self) {
        let pair : &mut (Rc<TypingSequence>, Rc<Pos>) = self.pieces.index_mut(self.next_at);
        self.exercise = Some(Exercise::new(Rc::clone(&pair.0), Rc::clone(&pair.1)));
        self.next_at = self.next_at + 1;
    }

    pub fn play<W: io::Write>(&mut self, key: &event::Key, mut output: &mut W) -> io::Result<(TypingInput, Ending)> {
        let mut exercise = self.exercise.as_mut().unwrap();
        let status = try!(exercise.play(&key));
        try!(exercise.write(&mut output));
        Ok(status)
    }

    pub fn write_all<W: io::Write>(&self, mut output: &mut W) -> io::Result<()> {
        let not_the_current = usize::max_value();
        for &(ref typing, ref pos) in self.pieces.iter() {
            try!(typing.write_seq(&mut output, not_the_current, &pos));
        }
        flush!(output)
    }

}


pub struct Exercise {
    subject: Weak<TypingSequence>,
    pos: Weak<Pos>,
    curr: usize
}


impl Exercise {

    pub fn new(seq: Rc<TypingSequence>, pos: Rc<Pos>) -> Exercise {
        Exercise { subject: Rc::downgrade(&seq), pos: Rc::downgrade(&pos), curr: 0 }
    }


    fn update_cursor_pos<W: io::Write>(&self, mut output: &mut W) -> io::Result<()> {
        use training::format::PosToTerm;
        let pos = self.pos.upgrade().unwrap();
        let pos : &Pos = pos.borrow();
        let cpos = Pos { x: pos.x + self.curr as u16, y: pos.y };
        write!(output, "{}", cpos.term_pos())
    }


    pub fn write<W: io::Write>(&mut self, mut output: &mut W) -> io::Result<()> {
        let mut subject = self.subject.upgrade().unwrap();
        let subject : &mut TypingSequence = Rc::get_mut(&mut subject).unwrap();

        try!(subject.write_seq(&mut output, self.curr, &self.pos.upgrade().unwrap()));
        try!(self.update_cursor_pos(&mut output));
        flush!(output)
    }


    pub fn play(&mut self, key: &event::Key) -> io::Result<(TypingInput, Ending)> {
        let mut subject = self.subject.upgrade().unwrap();
        let mut subject : &mut TypingSequence = Rc::get_mut(&mut subject).unwrap();
        let current = subject[self.curr].code;

        let ti = match key {
            &event::Key::Char(c) if c == current => {
                subject[self.curr].status = key::Status::Passed;
                self.curr += 1;
                TypingInput::Right
            }
            &event::Key::Char(_) => {
                subject[self.curr].status = key::Status::Missed;
                TypingInput::Wrong
            }
            _ => TypingInput::Unhandled
        };

        let is_done = self.curr >= subject.len();
        if is_done { Ok((ti, Ending::Completed)) } else { Ok((ti, Ending::Aborted)) }
    }
}

