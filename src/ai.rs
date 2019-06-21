use std::collections::{HashSet, VecDeque};
use std::collections::hash_map::{HashMap, Entry};
use std::cmp::Ordering;
use crate::game::{Pos, Direction, Size};

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

pub fn build_path(size: Size, snake: &VecDeque<Pos>, food: Pos) -> Direction {
    let mut vec: Vec<(u16, u16)> = get_neighbours(snake.front().unwrap());
    let mut found = false;
    let mut inspected = HashSet::new();
    while !(found || vec.is_empty()) {
        vec.sort_by_key(|e|
            -get_distance(&e, &food));
        let candidate = vec.pop().unwrap();
        if candidate == food {
            found = true;
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
            .cloned()
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
    path.sort_by_key(|e| match &map.entry(*e) {
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


#[cfg(test)]
mod tests {
    use super::{Size, Pos};
    use super::Direction;
    use super::build_path;
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