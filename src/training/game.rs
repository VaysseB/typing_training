use std::io;

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
        try!(self.ui.clear_and_hide_cursor(output));
        try!(self.ui.refresh(output));
        try!(self.training.write_all(output));
        let res = try!(self.training.play(input_provider, output));
        try!(self.ui.reset_cursor(output));
        Ok(res)
    }
}
