extern crate termion;

use std::io::{stdout, stdin};

use termion::raw::IntoRawMode;

mod training;

use training::positioning::{Constraint, HAlignment, VAlignment, PostLayoutAction};
use training::training::Training;
use training::ui::Ui;
use training::game::{self, Game};


fn main() {
    let stdin = stdin();

    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    // TODO fetch words to type
    let bucket : Vec<String> = vec!["if", "this", "is", "not", "in", "self", "then", "i", "begin", "to", "call", "rust"]
        .iter().map(|x: &&'static str| x.to_string()).collect();
    let sizes : Vec<usize> = bucket.iter().map(|ref w| w.len()).collect();

    // TODO extrapolate frame constraint based on terminal size and user settings
    let constraints = Constraint {
        win: game::play_area(),
        infinite_height: false,
        h_align: HAlignment::AlignMiddle,
        v_align: VAlignment::AlignCenter,
        action: Some(PostLayoutAction::ShrinkEmptySpaces(2, 1))
    };

    // plan words position on the screen based on constraints
    // if this is not possible, the app panic
    let mut ui = Ui::new(constraints, sizes);
    let positions = ui.do_layout().unwrap();
    let training = Training::new(bucket.into_iter(), positions.into_iter());
    Game::new(ui, training).exec(&|| stdin.lock(), &mut stdout).unwrap();
}
