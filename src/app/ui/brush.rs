use std::io;

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
    if dim.h == 0 || dim.w == 0 {
        return Ok(());
    }

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
    if dim.w > 1 {
        try!(write_hline(ACS_HLINE, &Pos {
            x: pos.x + 1,
            y: pos.y
        }, dim.w - 2, output));

        // bottom line
        try!(write_hline(ACS_HLINE, &Pos {
            x: pos.x + 1,
            y: pos.y + dim.h - 1
        }, dim.w - 2 , output));
    }

    if dim.h > 1 {
        // right line
        try!(write_vline(ACS_VLINE, &Pos {
            x: pos.x + dim.w - 1,
            y: pos.y + 1
        }, dim.h - 2, output));

        // left line
        try!(write_vline(ACS_VLINE, &Pos {
            x: pos.x,
            y: pos.y + 1
        }, dim.h - 2, output));
    }

    Ok(())
}


pub fn write_frame2(bbox: BoundingBox, gap: u16, output: &mut io::Write) -> io::Result<()> {
    write_frame(bbox.pos().shift(-1 * gap as i16, -1 * gap as i16), bbox.dim().grow(gap * 2, gap * 2), output)
}
