use std::collections::VecDeque;
use std::cell::RefCell;

pub type Pos = (u16, u16);
pub type Size = (u16, u16);

#[derive(Debug,Clone,PartialOrd, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq)]
pub enum MoveResult {
    Ok { new_pos: Pos },
    Die,
    Yummi { new_pos: Pos, food: Pos },
}

#[derive(Debug)]
pub struct Game {
    field: Size,
    snake: VecDeque<Pos>,
    food: Pos,
    score: u16,
    direction: Direction,
}

impl Game {
    pub fn new(size: Size) -> Game {
        Game {
            field: size,
            snake: Game::build_snake(&size),
            food: Game::rand_food(&size),
            score: 0,
            direction: Direction::Right,
        }
    }

    fn build_snake(field_size: &Size) -> VecDeque<Pos> {
        let mut snake: VecDeque<Pos> = VecDeque::with_capacity(5);
        let pos: Pos = (&field_size.0 / 2, &field_size.1 / 2);
        for i in 0..5 {
            snake.push_back((pos.0 - i, pos.1))
        }
        snake
    }

    pub fn move_snake(&mut self) -> MoveResult {
        let curr_head = self.snake.front().unwrap();
        let candidate = match self.direction {
            Direction::Up => { (curr_head.0, curr_head.1 - 1) }
            Direction::Down => { (curr_head.0, curr_head.1 + 1) }
            Direction::Left => { (curr_head.0 - 1, curr_head.1) }
            Direction::Right => { (curr_head.0 + 1, curr_head.1) }
        };

        if self.snake.contains(&candidate) ||
            !(0..self.field.0).contains(&candidate.0) ||
            !(0..self.field.1).contains(&candidate.1) {
            MoveResult::Die
        } else {
            self.snake.push_front(candidate);
            if self.food == candidate {
                self.food = Game::rand_food(&self.field);
                MoveResult::Yummi { new_pos: candidate, food: self.food }
            } else {
                self.snake.pop_back().unwrap();
                MoveResult::Ok { new_pos: candidate }
            }
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.clone() as u32 % 2 != direction.clone() as u32 % 2 {
            self.direction = direction;
        }
    }

    fn rand_food(field_size: &Size) -> Pos {
        (rand::random::<u16>() % field_size.0,
         rand::random::<u16>() % field_size.1)
    }

    pub fn get_food_pos(&self) -> Pos {
        self.food.clone()
    }

    pub fn get_snake(&self) -> VecDeque<Pos> {
        self.snake.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use super::Size;
    use super::Direction;
    use super::MoveResult;

    #[test]
    fn its_work() {
        let game = Game::new((10, 10));
        assert!(game.field == (10, 10))
    }

    #[test]
    fn build_snake_test() {
        let mut deque = Game::build_snake(&(10, 10));
        assert_eq!(deque.len(), 5);
        assert_eq!(deque.pop_front(), Some((5, 5)));
        assert_eq!(deque.pop_front(), Some((4, 5)));
        assert_eq!(deque.pop_front(), Some((3, 5)));
        assert_eq!(deque.pop_front(), Some((2, 5)));
        assert_eq!(deque.pop_front(), Some((1, 5)));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn rand_food_test() {
        let size: Size = (10, 10);
        let new_pos = Game::rand_food(&size);
        assert!((0..size.0).contains(&new_pos.0));
        assert!((0..size.1).contains(&new_pos.1));
    }

    #[test]
    fn yummi_test() {
        let mut game = Game::new((10, 10));
        game.food = (5, 6);
        game.set_direction(Direction::Down);
        if let MoveResult::Yummi { new_pos: pos, .. } = game.move_snake() {
            assert_eq!(pos, (5, 6));
            assert_eq!(game.snake.len(), 6);
            assert_eq!(game.snake.pop_front(), Some((5, 6)));
            assert_eq!(game.snake.pop_front(), Some((5, 5)));
            assert_eq!(game.snake.pop_front(), Some((4, 5)));
            assert_eq!(game.snake.pop_front(), Some((3, 5)));
            assert_eq!(game.snake.pop_front(), Some((2, 5)));
            assert_eq!(game.snake.pop_front(), Some((1, 5)));
            assert_eq!(game.snake.pop_front(), None);
        } else {
            panic!("It's not Yummi!");
        }
    }

    #[test]
    fn move_test() {
        let mut game = Game::new((10, 10));
        game.food = (6, 6);
        game.set_direction(Direction::Down);
        if let MoveResult::Ok { new_pos: pos } = game.move_snake() {
            assert_eq!(pos, (5, 6));
        } else {
            panic!("It's not Ok!");
        }
    }

    #[test]
    fn die_test() {
        let mut game = Game::new((10, 10));
        game.food = (6, 6);
        game.set_direction(Direction::Down);
        for _ in 0..5 {
            game.move_snake();
        }
        assert_eq!(game.move_snake(), MoveResult::Die);
        assert_eq!(game.food, (6, 6));
        assert_eq!(game.snake.front(), Some(&(5, 9)));
        assert_eq!(game.snake.get(1), Some(&(5, 8)));
        assert_eq!(game.snake.get(2), Some(&(5, 7)));
        assert_eq!(game.snake.get(3), Some(&(5, 6)));
        assert_eq!(game.snake.get(4), Some(&(5, 5)));
        assert_eq!(game.snake.get(5), None);
    }
}
