use termion::{color, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Read, Write, stdout, stdin, StdoutLock};
use termion::input::TermRead;


pub fn draw_border(stdout: &mut RawTerminal<StdoutLock>, size: (u16, u16)) {
    let up_down = vec![b'#'; size.0 as usize];
    let up_down: String = String::from_utf8(up_down).unwrap();
    write!(stdout,
           "{}{}{}{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           up_down,
           termion::cursor::Goto(1, size.1),
           up_down,
           termion::cursor::Hide
    ).unwrap();
    for i in 2..size.1 {
        write!(stdout,
               "{}{}{}{}",
               termion::cursor::Goto(1, i),
               "#",
               termion::cursor::Goto(size.0, i),
               "#"
        ).unwrap()
    }
}

pub fn clear_pos(stdout: &mut RawTerminal<StdoutLock>, pos: &(u16, u16)) {
    draw_point(stdout, pos, " ");
}


pub fn draw_point(stdout: &mut RawTerminal<StdoutLock>, pos: &(u16, u16), char: &str) {
    write!(stdout,
           "{}{}",
           termion::cursor::Goto(1 + pos.0, 1 + pos.1),
           char
    ).unwrap()
}
