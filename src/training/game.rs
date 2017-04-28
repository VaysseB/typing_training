use std::io;

use termion;
use termion::event::{Event, Key};
use termion::input::TermRead;

use training::ui::Ui;
use training::training::{Training, ExerciseStatus};
use training::positioning::Window;


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


        'main: while self.training.has_next() {
            // move to the next word to type
            self.training.next();

            // first update of the new exercise
            try!(self.training.write_current(&mut output));

            let input = input_provider();
            'exercise: for evt in input.events() {
                match evt.expect("no event") {
                    Event::Key(key) if key == Key::Esc => {
                        try!(write!(output, "ABORTED"));
                        break 'exercise;
                    }
                    Event::Key(key) => {
                        match self.training.play(&key) {
                            ExerciseStatus::Validated => { break 'exercise }
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

            // last update of the exercise (the last character isn't updated yet
            try!(self.training.write_current(&mut output));

            try!(self.wait_separator(input_provider, &mut output));
        }

        // clean up (a little bit only) after the game
        try!(self.ui.reset_cursor(output));
        Ok(())
    }


    fn wait_separator<F, R: io::Read, W: io::Write>(&mut self, input_provider: &F, mut output: &mut W) -> io::Result<()>
        where F: Fn() -> R {
        let input = input_provider();
        for evt in input.events() {
            match evt.expect("no event") {
                Event::Key(key) if key == Key::Esc => {
                    return write!(output, "ABORTED");
                }
                Event::Key(key) => {
                    match key {
                        Key::Char(c) if c == ' ' => break,
                        _ => ()
                    }
                }
                Event::Mouse(me) => {
                    try!(write!(output, "Mouse event! (=> {:?})", me));
                }
                Event::Unsupported(x) => {
                    try!(write!(output, "Unsupported event occurred (=> {:?})", x));
                }
            }
        }
        Ok(())
    }
}

pub fn play_area() -> Window {
    let size = termion::terminal_size().expect("no size of terminal");
    Window { x: 1, y: 1, h: size.1 - 1, w: size.0 }.shrinked_uniform(1)
}
