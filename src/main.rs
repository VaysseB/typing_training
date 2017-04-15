
extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::raw::IntoRawMode;

mod training;


fn main() {
    let stdin = stdin();

    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let bucket = vec!["if", "this", "is", "not", "in", "self", "then", "i", "begin", "to", "call", "rust"];

    // init setup
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
        .unwrap();
    stdout.flush().unwrap();

    {
        use training::sequence::TypingSequence;
        use training::game::{Exercise, Ending};
        use training::sign::Pos;

        let pos = Pos{x: 1, y: 1};
        'game: for word in bucket.iter() {
            let mut typing = TypingSequence::new(word.to_string());
            let mut exercise = Exercise::new(&mut typing, &pos);
            match exercise.play(|| stdin.lock(), &mut stdout) {
                Ending::Aborted => { break 'game },
                Ending::Completed => {}
            }

            // temporary cleaning while words doesn't have any position
            use training::sign::PosToTermConverter;
            write!(stdout, "{}{}",
                    pos.term_pos(),
                    termion::clear::CurrentLine).unwrap();
        }
    }

    // cleanup
    write!(stdout, "\n{}", termion::cursor::Show).unwrap();
}
