use std::io;

use termion::event::{Event, Key};
use termion::input::TermRead;

use training::ui::Ui;
use training::training::{Training, ExerciseStatus};


pub struct Game {
    pub ui: Ui,
    pub training: Training
}


impl Game {
    pub fn new(ui: Ui, t: Training) -> Game {
        Game { ui: ui, training: t }
    }

    pub fn exec<F, R: io::Read, W: io::Write>(&mut self, input_provider: &F, mut output: &mut W) -> io::Result<()>
        where F: Fn() -> R {
        // setup the game
        try!(self.ui.clear_and_hide_cursor(output));
        try!(self.ui.refresh(output));
        try!(self.training.write_all(output));

        // move to the first word to type
        self.training.next();

        let input = input_provider();
        'events: for evt in input.events() {
            match evt.expect("no event") {
                Event::Key(key) if key == Key::Esc => {
                    try!(write!(output, "ABORTED"));
                    break 'events;
                }
                Event::Key(key) => {
                    match self.training.play(&key) {
                        ExerciseStatus::Validated => {
                            if self.training.has_next() {
                                try!(self.training.write_current(&mut output));
                                self.training.next()
                            } else { break 'events }
                        }
                        ExerciseStatus::NotYetDone => ()
                    }
                }
                Event::Mouse(me) => {
                    try!(write!(output, "Mouse event! (=> {:?})", me));
                }
                Event::Unsupported(x) => {
                    try!(write!(output, "Unsupported event occurred (=> {:?})", x));
                }
            }

            try!(self.training.write_current(&mut output));
        }

        // clean up (a little bit only) after the game
        try!(self.ui.reset_cursor(output));
        Ok(())
    }
}
