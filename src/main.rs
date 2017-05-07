extern crate termion;

use std::io::{stdout, stdin, Read, Write};

use termion::raw::IntoRawMode;

#[macro_use]
mod app;



fn main() {
    //    let stdin = stdin();

    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let bucket = app::Bucket::new(vec!["test", "this", "and", "the", "next"]);
    let ui_constraint = app::ui::Constraint {
        dim: app::term_impl::term_dim().into(),
        align: app::ui::Alignment::centered()
    };
    let layout = app::ui::layout(&ui_constraint, &bucket).expect("cannot layout word in those constraints");

    write_iter!(&mut stdout, "{}{}", layout.positions, bucket.words).unwrap();
}
