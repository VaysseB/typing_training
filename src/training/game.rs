use std::io;

use termion::event::{Event, Key};
use termion::input::TermRead;

use training::ui::Ui;
use training::training::{Training, Ending};


pub struct Game {
    pub ui: Ui,
    pub training: Training
}


impl Game {
    pub fn new(ui: Ui, t: Training) -> Game {
        Game { ui: ui, training: t }
    }

    pub fn exec<F, R: io::Read, W: io::Write>(&mut self, input_provider: &F, mut output: &mut W) -> io::Result<Ending>
        where F: Fn() -> R {
        // setup the game
        try!(self.ui.clear_and_hide_cursor(output));
        try!(self.ui.refresh(output));
        try!(self.training.write_all(output));

        // move to the first word to type
        self.training.next();

        let input = input_provider();
        let mut res = Ending::Completed;
        'events: for evt in input.events() {
            match evt.expect("no event") {
                Event::Key(key) if key == Key::Esc => {
                    res = Ending::Aborted;
                    break 'events;
                },
                Event::Key(key) => {
                    res = try!(self.training.play(&key, output)).1;
                    match res {
                        Ending::Aborted => { break 'events; },
                        _ => ()
                    }
                },
                Event::Mouse(me) => {
                    try!(write!(output, "Mouse event! (=> {:?})", me));
                },
                Event::Unsupported(x) => {
                    try!(write!(output, "Unsupported event occurred (=> {:?})", x));
                }
            }
        }

        match res {
            Ending::Aborted => { try!(write!(output, "ABORTED")); },
            _ => ()
        }

        // clean up (a little bit only) after the game
        try!(self.ui.reset_cursor(output));
        Ok(Ending::Completed/*res*/)
    }
}
