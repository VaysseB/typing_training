
extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::raw::IntoRawMode;

mod training;
mod game;


fn main() {
    let stdin = stdin();

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
    stdout.flush().unwrap();

    'game: loop {
        use training::sequence::TypingSequence;
        use training::sign::Pos;

        let mut sequence = TypingSequence::new(raw_sentence.to_string());
        let pos = Pos{x: 1, y: 1};
        {
            match game::exercise(&mut sequence, || stdin.lock(), &mut stdout, &pos) {
                game::Status::Aborted => { break 'game },
                game::Status::Completed => { break 'game }
            }
        }
    }

    // cleanup
    write!(stdout, "\n{}", termion::cursor::Show).unwrap();
}
