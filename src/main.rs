
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

        // TODO fetch words to type
        let bucket = vec!["if", "this", "is", "not", "in", "self", "then", "i", "begin", "to", "call", "rust"]
            .iter().map(|x| x.to_string()).collect();

        // TODO extrapolate frame constraint based on terminal size and user settings
        let constraints = Constraint {
            win: Window { x: 2, y: 2, h: 5, w: 24 },
            infinite_height: false,
            h_align: HAlignment::AlignMiddle,
            v_align: VAlignment::AlignCenter
        };

        // plan words position on the screen based on constraints
        // if this is not possible, the app panic
        let positions;
        match constraints.organise(&bucket) {
            Err(msg) => panic!(msg),
            Ok(r) => { positions = r; }
        }

        // print the big frame
        use training::print::WindowPrinter;
        constraints.win.grown_uniform(1).write_rect(&mut stdout).unwrap();

        // main loop of the game
        // typing word by word
        'game: for (word, pos) in bucket.iter().zip(positions) {
            let mut typing = TypingSequence::new(word.to_string());
            let mut exercise = Exercise::new(&mut typing, &pos);
            match exercise.play(|| stdin.lock(), &mut stdout) {
                Ending::Aborted => { break 'game },
                Ending::Completed => {}
            }

//            // temporary cleaning while words doesn't have any position
//            use training::format::PosToTerm;
//            write!(stdout, "{}{}",
//                    pos.term_pos(),
//                    termion::clear::CurrentLine).unwrap();
        }
    }

    // final cleanup
    write!(stdout, "\n\r{}", termion::cursor::Show).unwrap();
}
