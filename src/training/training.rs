
use std::io;
use std::ops::IndexMut;
use std::cell::{Ref, RefMut, RefCell};
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
    pub pieces: Box<[(Rc<RefCell<TypingSequence>>, Rc<RefCell<Pos>>)]>,
    pub next_at: usize,
    pub exercise: Option<Exercise>
}


impl Training {

    pub fn new<T, R>(words: T, plan: R) -> Training
        where T: Iterator<Item=String>, R: Iterator<Item=Pos> {
        let pieces : Vec<(Rc<RefCell<TypingSequence>>, Rc<RefCell<Pos>>)> = words
            .map(|w| Rc::new(RefCell::new(TypingSequence::new(&w))) )
            .zip(plan.map(|p| Rc::new(RefCell::new(p))))
            .collect();

        Training { pieces: pieces.into_boxed_slice(), next_at: 0, exercise: None }
    }

    pub fn next(&mut self) {
        let pair = self.pieces.index_mut(self.next_at);
        self.exercise = Some(Exercise::new(Rc::clone(&pair.0), Rc::clone(&pair.1)));
        self.next_at = self.next_at + 1;
    }

    pub fn play<W: io::Write>(&mut self, key: &event::Key, mut output: &mut W) -> io::Result<(TypingInput, Ending)> {
        let mut exercise = self.exercise.as_mut().expect("cannot play without exercise");
        let status = try!(exercise.play(&key));
        try!(exercise.write(&mut output));
        Ok(status)
    }

    pub fn write_all<W: io::Write>(&self, mut output: &mut W) -> io::Result<()> {
        let not_the_current = usize::max_value();
        for &(ref typing, ref pos) in self.pieces.iter() {
            let typing : &RefCell<TypingSequence> = Rc::borrow(typing);
            let typing : Ref<TypingSequence> = typing.borrow();
            let pos : &RefCell<Pos> = Rc::borrow(pos);
            let pos : Ref<Pos> = pos.borrow();
            try!(typing.write_seq(&mut output, not_the_current, &pos));
        }
        flush!(output)
    }

}


pub struct Exercise {
    subject: Weak<RefCell<TypingSequence>>,
    pos: Weak<RefCell<Pos>>,
    curr: usize
}


impl Exercise {
    pub fn new(seq: Rc<RefCell<TypingSequence>>, pos: Rc<RefCell<Pos>>) -> Exercise {
        Exercise { subject: Rc::downgrade(&seq), pos: Rc::downgrade(&pos), curr: 0 }
    }

    fn update_cursor_pos<W: io::Write>(&self, mut output: &mut W) -> io::Result<()> {
        use training::format::PosToTerm;
        match self.pos.upgrade() {
            Some(pos) => {
                let pos: &RefCell<Pos> = Rc::borrow(&pos);
                let pos: Ref<Pos> = pos.borrow();
                let term_pos = Pos { x: pos.x + self.curr as u16, y: pos.y };
                write!(output, "{}", term_pos.term_pos())
            },
            None => Ok(())
        }
    }

    pub fn write<W: io::Write>(&mut self, mut output: &mut W) -> io::Result<()> {
        let subject = self.subject.upgrade().expect("no subject to write");
        let subject : &RefCell<TypingSequence> = Rc::borrow(&subject);
        let subject : Ref<TypingSequence> = subject.borrow();
        let pos = self.pos.upgrade().expect("no position to write subject");
        let pos : &RefCell<Pos> = Rc::borrow(&pos);
        let pos : Ref<Pos> = pos.borrow();

        try!(subject.write_seq(&mut output, self.curr, &pos));
        try!(self.update_cursor_pos(&mut output));
        flush!(output)
    }

    pub fn play(&mut self, key: &event::Key) -> io::Result<(TypingInput, Ending)> {
        let subject = self.subject.upgrade().expect("no subject to write");
        let subject : &RefCell<TypingSequence> = Rc::borrow(&subject);
        let mut subject : RefMut<TypingSequence> = subject.borrow_mut();
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

