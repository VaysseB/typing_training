extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::event::{Key};
use termion::color::{self};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod typing;
mod format;

use format::TypingFormat;


fn main() {
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let raw_sentence = "if this is not in self, then i begin to call rust.";

    // init setup
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
        .unwrap();
    write!(stdout, "{}*** Rules ***",
           termion::cursor::Goto(1, 1)).unwrap();
    write!(stdout, "{}1) Type what is displayed",
           termion::cursor::Goto(1, 2)).unwrap();
    stdout.flush().unwrap();


    'game: loop {
        let mut typing = typing::new(raw_sentence);


        write!(stdout,
               "{}{}>>> {}",
               termion::cursor::Goto(1, 3),
               termion::clear::CurrentLine,
               typing.highlight_display())
            .unwrap();

        write!(stdout,
               "{}{}Status: beginning",
               termion::cursor::Goto(1, 4),
               termion::clear::CurrentLine)
            .unwrap();
        stdout.flush().unwrap();


        let stdin = stdin();
        'step: loop {
            let stdin = stdin.lock();
            let current = match typing.curr_ref() {
                None => break 'game,
                Some(key) => key.code
            };

            write!(stdout,
                   "{}{}>>> {}",
                   termion::cursor::Goto(1, 3),
                   termion::clear::CurrentLine,
                   typing.highlight_display())
                .unwrap();
            stdout.flush().unwrap();

            'input: for c in stdin.keys() {
                let status = match c.unwrap() {
                    Key::Esc => break 'game,
                    Key::Char(c) if c == current => Ok(c),
                    Key::Char(c) => Err(c),
                    _ => Err('\0')
                };

                match status {
                    Err('\0') =>
                        write!(stdout,
                               "{}{}Status: /!\\ illegal input /!\\",
                               termion::cursor::Goto(1, 4),
                               termion::clear::CurrentLine)
                            .unwrap(),
                    Err(_) => {
                        write!(stdout,
                               "{}{}Status: missed",
                               termion::cursor::Goto(1, 4),
                               termion::clear::CurrentLine)
                            .unwrap();
                        typing.miss();
                    }
                    Ok(c) => {
                        write!(stdout,
                               "{}{}Status: good '{}'",
                               termion::cursor::Goto(1, 4),
                               termion::clear::CurrentLine,
                               c)
                            .unwrap();
                        typing.pass();
                        typing.forward();
                        break 'input;
                    }
                }

                write!(stdout,
                       "{}{}>>> {}",
                       termion::cursor::Goto(1, 3),
                       termion::clear::CurrentLine,
                       typing.highlight_display())
                    .unwrap();
                stdout.flush().unwrap();
            }
        }
    }

    // cleanup
    write!(stdout, "{}{}",
             termion::cursor::Goto(1, 5),
             termion::cursor::Show).unwrap();
}
