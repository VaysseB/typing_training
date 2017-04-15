
extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::raw::IntoRawMode;

mod training;


fn main() {
    let stdin = stdin();

    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

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
        use training::positioning::{Constraint, Window, HAlignment, VAlignment, Positioning};

        let bucket = vec!["if", "this", "is", "not", "in", "self", "then", "i", "begin", "to", "call", "rust"]
            .iter().map(|x| x.to_string()).collect();
        let constraints = Constraint {
            win: Window { x: 1, y: 1, h: 3, w: 24 },
            infinite_height: false,
            h_align: HAlignment::AlignLeft,
            v_align: VAlignment::AlignTop
        };
        let positions;
        match constraints.organise(&bucket) {
            Err(msg) => panic!(msg),
            Ok(r) => { positions = r; }
        }

        'game: for (word, pos) in bucket.iter().zip(positions) {
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
