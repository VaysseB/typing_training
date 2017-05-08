extern crate termion;

#[allow(unused_imports)]
use std::io::{stdout, stdin, Read, Write};

use termion::raw::IntoRawMode;

#[macro_use]
mod app;


fn main() {
    // init
    let stdin = stdin();
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();
    write!(&mut stdout, "{}{}{}{}{}",
           termion::clear::All,
           app::ui::Pos { x: 1, y: 1 },
           termion::color::Bg(termion::color::Reset),
           termion::color::Fg(termion::color::Reset),
           termion::cursor::Hide
    ).unwrap();

    // input
    let f_term_size = || app::ui::term_dim().shrink(2, 2).into();
    let bucket = app::word::Bucket::new(vec!["test", "this", "and", "the", "next"]);

    // setup
    let count = bucket.len();
    let ui_constraint = app::ui::Constraint {
        origin: app::ui::Pos { x: 1, y: 1 }.shift(1, 1),
        dim: f_term_size(),
        align: app::ui::Alignment::centered()
    };
    let layout = app::ui::layout(&ui_constraint, &bucket)
        .expect("cannot layout word in those constraints");
    let status_bar_starter = app::ui::Pos { x: 1, y: app::ui::term_dim().h - 1 };

    // init print
    app::ui::brush::write_frame2(layout.frame, 2, &mut stdout).unwrap();
    write_iter!(&mut stdout, "{}{}", layout.positions, bucket.words).unwrap();

    // main loop
    'mainloop: for i_curr in 0..count {
        let exercise = app::exercise::new(&bucket[i_curr]);
        let mut progress: usize = 0;

        // initial key colorisation
        {
            write!(stdout, "{}{}{}",
                   termion::color::Bg(termion::color::Magenta),
                   layout.positions[i_curr],
                   exercise[progress]
            ).unwrap();
        }

        stdout.flush().unwrap();

        use termion::input::TermRead;
        let input = stdin.lock();
        'word: for event in input.events() {
            match event.expect("no event") {
                termion::event::Event::Key(key) => {
                    match key {
                        termion::event::Key::Esc => {
                            write!(stdout, "{}Aborted game", status_bar_starter).unwrap();
                            break 'mainloop;
                        }
                        termion::event::Key::Char(char_) => {
                            let ref pos = layout.positions[i_curr];
                            let curr_progress = progress;

                            match char_ == exercise[progress] {
                                true => {
                                    write!(stdout, "{}", termion::color::Bg(
                                        termion::color::Green
                                    )).unwrap();
                                    progress += 1;
                                }
                                false => {
                                    write!(stdout, "{}", termion::color::Bg(
                                        termion::color::Red
                                    )).unwrap();
                                }
                            }

                            write!(stdout, "{}{}", pos.shift(curr_progress as i16, 0), char_).unwrap();

                            if progress < exercise.len() {
                                write!(stdout, "{}{}{}",
                                       pos.shift(progress as i16, 0),
                                       termion::color::Bg(termion::color::Magenta),
                                       exercise[progress]
                                ).unwrap();
                            }
                            else {
                                break 'word;
                            }
                        }
                        // any other thing that isn't a simple char
                        _ => ()
                    }
                }
                termion::event::Event::Mouse(me) => {
                    write!(stdout, "{}Mouse event! (=> {:?})", status_bar_starter, me).unwrap();
                }
                termion::event::Event::Unsupported(x) => {
                    write!(stdout, "{}Unsupported event occurred (=> {:?})", status_bar_starter, x).unwrap();
                }
            }

            stdout.flush().unwrap();
        }
    }

    // finisher
    write!(&mut stdout, "{}{}{}{}\n",
           app::ui::Pos { x: 1, y: app::ui::term_dim().h - 1 },
           termion::cursor::Show,
           termion::color::Bg(termion::color::Reset),
           termion::color::Fg(termion::color::Reset)
    ).unwrap();
    stdout.flush().unwrap();
}
