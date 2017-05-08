extern crate termion;

#[allow(unused_imports)]
use std::io::{stdout, stdin, Read, Write};

use termion::raw::IntoRawMode;

#[macro_use]
mod app;


fn main() {
    //    let stdin = stdin();

    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();
    write!(&mut stdout, "{}{}", termion::clear::All, app::ui::Pos{ x: 1, y: 1 }).unwrap();

    let bucket = app::Bucket::new(vec!["test", "this", "and", "the", "next"]);
    let ui_constraint = app::ui::Constraint {
        origin: app::ui::Pos { x: 1, y: 1 }.shift(1, 1),
        dim: app::ui::term_dim().shrink(2, 2).into(),
        align: app::ui::Alignment::centered()
    };
    let layout = app::ui::layout(&ui_constraint, &bucket)
        .expect("cannot layout word in those constraints");

    write!(&mut stdout, "{}{:?}", app::ui::Pos { x: 1, y: 1 }, ui_constraint).unwrap();
    write!(&mut stdout, "{}{:?}", app::ui::Pos { x: 1, y: 2 }, layout.frame).unwrap();
    app::ui::brush::write_frame2(layout.frame, 2, &mut stdout).unwrap();
    write_iter!(&mut stdout, "{}{}", layout.positions, bucket.words).unwrap();
}
