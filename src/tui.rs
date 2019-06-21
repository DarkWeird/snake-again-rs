use termion::{color, terminal_size, async_stdin};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Read, Write, stdout, stdin, StdoutLock};
use termion::input::TermRead;
use std::thread::sleep;

use crate::game::Game;
use crate::game::Direction;
use crate::game::MoveResult;
use crate::game::Pos;
use crate::game::Size;
use crate::ai::build_path;

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

const FOOD_CHAR: &str = "@";
const SNAKE_CHAR: &str = "+";

pub fn run() {
    let size = termion::terminal_size().unwrap();
    let mut game = Game::new(size);
    let mut snake = game.get_snake();
    let mut food_pos = game.get_food_pos();

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();
    let stdin = stdin;

    let mut autopilot = false;

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide);
    draw_point(&mut stdout, &food_pos, FOOD_CHAR);
    for pos in snake.clone() {
        draw_point(&mut stdout, &pos, SNAKE_CHAR);
    }

    stdout.flush().unwrap();

    let mut keys = stdin.keys();
    loop {
        if let Some(Ok(c)) = keys.next() {
            if let Some(direction) = match c {
                termion::event::Key::Char('q') => {
                    write!(stdout, "{}", termion::clear::All);
                    return;
                }
                termion::event::Key::Char('a') => {
                    autopilot = !autopilot;
                    None
                }
                termion::event::Key::Up => Some(Direction::Up),
                termion::event::Key::Down => Some(Direction::Down),
                termion::event::Key::Left => Some(Direction::Left),
                termion::event::Key::Right => Some(Direction::Right),
                _ => None,
            } {
                if !autopilot {
                    game.set_direction(direction);
                }
            }
        }
        if autopilot {
            game.set_direction(build_path(size, &snake, food_pos));
        }
        match game.move_snake() {
            MoveResult::Ok { new_pos } => {
                clear_pos(&mut stdout, &snake.pop_back().unwrap());
                snake.push_front(new_pos);
                draw_point(&mut stdout, &new_pos, SNAKE_CHAR)
            }
            MoveResult::Die => return,
            MoveResult::Yummi { new_pos, food } => {
                clear_pos(&mut stdout, &food_pos);
                draw_point(&mut stdout, &food, FOOD_CHAR);
                food_pos = food;
                snake.push_front(new_pos);
                draw_point(&mut stdout, &new_pos, SNAKE_CHAR)
            }
        }
//        write!(stdout,"{}{:?}",termion::cursor::Goto(1,size.1),build_path(size, &snake, food_pos));
        stdout.flush().unwrap();

        sleep(std::time::Duration::from_millis(if autopilot { 10 } else { 100 }));
    }
}