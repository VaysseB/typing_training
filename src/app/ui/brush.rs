use std::io;
use std::io::Write;

use app::ui::{Pos, Dim, BoundingBox};


// taken from python3.6/curses/textpad.py
pub static ACS_TLCORNER: char = '┌';
pub static ACS_BLCORNER: char = '└';
pub static ACS_TRCORNER: char = '┐';
pub static ACS_BRCORNER: char = '┘';
pub static ACS_HLINE: char = '─';
pub static ACS_VLINE: char = '│';


//---
pub fn write_at(keycode: char, pos: &Pos, output: &mut io::Write) -> io::Result<()> {
    try!(writeln!(output, "{}{}", pos, keycode));
    Ok(())
}


pub fn write_hline(keycode: char, pos: &Pos, w: u16, output: &mut io::Write) -> io::Result<()> {
    let line = keycode.to_string().repeat(w as usize);
    try!(writeln!(output, "{}{}", pos, line));
    Ok(())
}


pub fn write_vline(keycode: char, pos: &Pos, h: u16, output: &mut io::Write) -> io::Result<()> {
    for dy in 0..h {
        let rpos = Pos { x: pos.x, y: pos.y + dy };
        try!(writeln!(output, "{}{}", rpos, keycode));
    }
    Ok(())
}


pub fn write_frame(pos: Pos, dim: Dim, output: &mut io::Write) -> io::Result<()> {
    // top left
    try!(write_at(ACS_TLCORNER, &Pos {
        x: pos.x,
        y: pos.y
    }, output));

    // top right
    try!(write_at(ACS_TRCORNER, &Pos {
        x: pos.x + dim.w - 1,
        y: pos.y
    }, output));

    // bottom left
    try!(write_at(ACS_BLCORNER, &Pos {
        x: pos.x,
        y: pos.y + dim.h - 1
    }, output));

    // bottom right
    try!(write_at(ACS_BRCORNER, &Pos {
        x: pos.x + dim.w - 1,
        y: pos.y + dim.h - 1
    }, output));

    // top line
    try!(write_hline(ACS_HLINE, &Pos {
        x: pos.x,
        y: pos.y
    }, dim.w - 2 , output));

    // right line
    try!(write_vline(ACS_VLINE, &Pos {
        x: pos.x + dim.w - 1,
        y: pos.y
    }, dim.h - 2 , output));

    // bottom line
    try!(write_hline(ACS_HLINE, &Pos {
        x: pos.x + dim.w - 1,
        y: pos.y + dim.h - 1
    }, dim.w - 2 , output));

    // left line
    try!(write_vline(ACS_VLINE, &Pos {
        x: pos.x,
        y: pos.y
    }, dim.h - 2 , output));

    Ok(())
}
