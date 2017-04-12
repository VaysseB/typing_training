extern crate termion;

use termion::event::{Key};
use termion::color::{self};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};


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
        let material = raw_sentence.to_string();
        let mut training = material.chars();
        let mut i_split: usize = 0;

        write!(stdout,
               "{}{}>>> {}",
               termion::cursor::Goto(1, 3),
               termion::clear::CurrentLine,
               raw_sentence)
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
            let current = match training.next() {
                None => break 'game,
                Some(c) => c
            };

            let (head, mid_tail) = material.split_at(i_split);
            let mid_tail = mid_tail.to_string();
            let (curr, tail) = mid_tail.split_at(current.len_utf16());
            write!(stdout,
                   "{}{}>>> {}{}[{}]{}{}",
                   termion::cursor::Goto(1, 3),
                   termion::clear::CurrentLine,
                   head,
                   color::Bg(color::Green),
                   curr,
                   color::Bg(color::Reset),
                   tail)
                .unwrap();
            stdout.flush().unwrap();

            'input: for c in stdin.keys() {
                let status = match c.unwrap() {
                    Key::Esc => break 'game,
                    Key::Char(c) if c == current => Ok(c),
                    Key::Char(c) => Err(c),
                    _ => Err('\0')
                };

                // copy-paste for discovery ideas
                let (head, mid_tail) = material.split_at(i_split);
                let mid_tail = mid_tail.to_string();
                let (curr, tail) = mid_tail.split_at(current.len_utf16());
                write!(stdout,
                       "{}{}>>> {}{}[{}]{}{}",
                       termion::cursor::Goto(1, 3),
                       termion::clear::CurrentLine,
                       head,
                       color::Bg(color::LightRed),
                       curr,
                       color::Bg(color::Reset),
                       tail)
                    .unwrap();
                stdout.flush().unwrap();

                match status {
                    Err('\0') =>
                        write!(stdout,
                               "{}{}Status: /!\\ illegal input /!\\",
                               termion::cursor::Goto(1, 4),
                               termion::clear::CurrentLine)
                            .unwrap(),
                    Err(_) =>
                        write!(stdout,
                               "{}{}Status: missed",
                               termion::cursor::Goto(1, 4),
                               termion::clear::CurrentLine)
                            .unwrap(),
                    Ok(c) => {
                        write!(stdout,
                               "{}{}Status: good '{}'",
                               termion::cursor::Goto(1, 4),
                               termion::clear::CurrentLine,
                               c)
                            .unwrap();
                        i_split = i_split + current.len_utf16();
                        break 'input;
                    }
                }

                stdout.flush().unwrap();
            }
        }
    }

    // cleanup
    write!(stdout, "{}{}",
           termion::cursor::Goto(1, 5),
           termion::cursor::Show).unwrap();
}
