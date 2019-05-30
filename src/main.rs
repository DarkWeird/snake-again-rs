extern crate termion;
extern crate rand;

mod game;
mod tui;

use game::Game;
use game::Direction;
use game::MoveResult;
use game::Pos;
use game::Size;

use termion::{color, terminal_size, async_stdin};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Read, Write, stdout, stdin, StdoutLock};
use termion::input::TermRead;

use tui::*;
use std::thread::sleep;
use std::collections::{VecDeque, HashSet, HashMap};
use std::cmp::Ordering;
use std::collections::hash_map::{Entry, OccupiedEntry};

const FOOD_CHAR: &str = "@";
const SNAKE_CHAR: &str = "+";

fn get_neighbours(pos: &Pos) -> Vec<Pos> {
    let mut vec = Vec::with_capacity(4);
    if pos.0 != 0 {
        vec.push((pos.0 - 1, pos.1));
    }
    if pos.1 != 0 {
        vec.push((pos.0, pos.1 - 1));
    }
    vec.push((pos.0 + 1, pos.1));
    vec.push((pos.0, pos.1 + 1));
    vec
}

fn get_distance(from: &Pos, to: &Pos) -> i32 {
    (((to.0 as f32 - from.0 as f32) as f32).powi(2)
        + ((to.1 as f32 - from.1 as f32) as f32).powi(2)) as i32
}

fn build_path(size: Size, snake: &VecDeque<Pos>, food: Pos) -> Direction {
    let mut vec: Vec<(u16, u16)> = get_neighbours(snake.front().unwrap());
    let mut finded = false;
    let mut inspected = HashSet::new();
    while !(finded || vec.is_empty()) {
        vec.sort_by_key(|e|
            -get_distance(&e, &food));
        let candidate = vec.pop().unwrap();
        if candidate == food {
            finded = true;
        }
        if snake.contains(&candidate) ||
            !(0..size.0).contains(&candidate.0) ||
            !(0..size.1).contains(&candidate.1) ||
            inspected.contains(&candidate) {
            continue;
        } else {
            &inspected.insert(candidate);
            for e in get_neighbours(&candidate).iter() {
                vec.push(*e);
            }
        }
    }
    let mut map = HashMap::new();
    map.entry(food).or_insert(0u32);

    let mut near = get_neighbours(&food);
    let mut i = 1;
    loop {
        if near.contains(snake.front().unwrap()) {
            break;
        }
        near = near.iter()
            .map(|e| *e)
            .filter(|e| inspected.contains(e))
            .filter(|e| {
                match map.entry(*e) {
                    Entry::Occupied(_) => { false }
                    Entry::Vacant(e) => {
                        e.insert(i);
                        true
                    }
                }
            })
            .flat_map(|e| get_neighbours(&e))
            .collect();
        i += 1;
    }

    let mut path = get_neighbours(snake.front().unwrap());
    path.sort_by_key(|e|  match &map.entry(*e) {
        Entry::Occupied(x) => { *x.get() }
        Entry::Vacant(_) => { 10000 }
    });

    if let Some(candidate) = path.first() {
        let head = snake.front().unwrap();
        match head.0.cmp(&candidate.0) {
            Ordering::Less => Direction::Right,
            Ordering::Equal => {
                match head.1.cmp(&candidate.1) {
                    Ordering::Less => Direction::Down,
                    Ordering::Equal => { panic!("o_O") }
                    Ordering::Greater => Direction::Up,
                }
            }
            Ordering::Greater => Direction::Left,
        }
    } else {
        panic!("it's impossible");
    }
}


fn main() {
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
// Quit
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


#[cfg(test)]
mod tests {
    use super::{Size, Pos};
    use super::build_path;
    use super::Direction;
    use std::collections::VecDeque;

    #[test]
    fn up() {
        let size: Size = (10u16, 10u16);
        let mut snake: VecDeque<Pos> = VecDeque::with_capacity(5);
        let pos: Pos = (&size.0 - 1, &size.1 - 1);
        for i in 0..5 {
            snake.push_back((pos.0 - i, pos.1))
        }
//        dbg!(&snake);
        draw_field(size, &snake, (5, 5));
        let dir = build_path(size, &snake, (4, 4));
        assert_eq!(dir, Direction::Up);
    }

    #[test]
    fn down() {
        let size: Size = (10u16, 10u16);
        let mut snake: VecDeque<Pos> = VecDeque::with_capacity(5);
        let pos: Pos = (&size.0 - 1, 1);
        for i in 0..5 {
            snake.push_back((pos.0 - i, pos.1))
        }
        draw_field(size, &snake, (5, 5));
        let dir = build_path(size, &snake, (5, 5));
        assert_eq!(dir, Direction::Down);
    }

    #[test]
    fn left() {
        let size: Size = (10u16, 10u16);
        let mut snake: VecDeque<Pos> = VecDeque::with_capacity(5);
        let pos: Pos = (&size.0 - 1, 1);
        for i in 0..5 {
            snake.push_back((pos.0, pos.1 + i))
        }
//        dbg!(&snake);
        draw_field(size, &snake, (5, 5));
        let dir = build_path(size, &snake, (5, 5));
        assert_eq!(dir, Direction::Left);
    }

    #[test]
    fn right() {
        let size: Size = (10u16, 10u16);
        let mut snake: VecDeque<Pos> = VecDeque::with_capacity(5);
        let pos: Pos = (1, 1);
        for i in 0..5 {
            snake.push_back((pos.0, pos.1 + i))
        }
//        dbg!(&snake);
        draw_field(size, &snake, (5, 5));
        let dir = build_path(size, &snake, (5, 5));
        assert_eq!(dir, Direction::Right);
    }

    fn draw_field(size: Size, snake: &VecDeque<Pos>, food: Pos) {
        for i in 0..size.0 {
            for j in 0..size.1 {
                match (j, i) {
                    x if x == food => print!("@"),
                    x if x == *snake.front().unwrap() => print!("="),
                    x if snake.contains(&x) => print!("+"),
                    _ => print!("."),
                }
            }
            println!();
        }
    }
}