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
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
        .unwrap();
    stdout.flush().unwrap();

    let used_height;
    {
        use training::game::{Game};
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

        // now we have everything to build the game
        let mut game = Game::new(bucket.into_iter(), positions.into_iter());


        // print the big frame
        use training::print::WindowPrinter;
        constraints.win.grown_uniform(1).write_rect(&mut stdout).unwrap();
        // print every words the first time
        game.write_all(&mut stdout).unwrap();
        // end of init of the game
        stdout.flush().unwrap();

        // start the game
        game.play(&|| stdin.lock(), &mut stdout).unwrap();

        used_height = constraints.win.y + constraints.win.h + 1;
    }

    // final cleanup
    use training::positioning::Pos;
    use training::format::PosToTerm;
    write!(stdout, "{}{}",
           Pos { x: 1, y: used_height }.term_pos(),
           termion::cursor::Show)
        .unwrap();
}
