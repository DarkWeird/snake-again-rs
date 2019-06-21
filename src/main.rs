extern crate termion;
extern crate rand;
extern crate kiss3d;
extern crate nalgebra as na;

mod game;
mod tui;
mod ai;

use kiss3d::light::Light;
use kiss3d::scene::PlanarSceneNode;
use kiss3d::window::{State, Window};
use na::{UnitComplex, Isometry2, Translation2, Vector2};
use kiss3d::event::{Action, WindowEvent, Key};
use crate::game::{Size, Game, Pos, Direction, MoveResult};
use crate::ai::build_path;
use std::collections::VecDeque;

const CELL_SIZE: f32 = 5.0;

struct AppState {
    autopilot: bool,
    game: Game,
    size: Size,
    snake: VecDeque<PlanarSceneNode>,
    food: PlanarSceneNode,
}


impl AppState {
    pub fn new(window: &mut Window, size: Size) -> AppState {
        let game = Game::new(size);
        let snake = game.get_snake();
        let food_pos = game.get_food_pos();
        AppState {
            autopilot: false,
            game,
            size,
            snake: snake.iter().cloned().map(|e| AppState::create_snake_rect(window, size, e)).collect(),
            food: AppState::create_food(window, size, food_pos),
        }
    }

    pub fn create_snake_rect(window: &mut Window, fieldSize: Size, pos: Pos) -> PlanarSceneNode {
        let mut rect = window.add_rectangle(CELL_SIZE, CELL_SIZE);
        rect.append_translation(&AppState::translate(fieldSize, pos));
        rect.set_color(1.0, 0.0, 0.0);
        rect
    }

    pub fn create_food(window: &mut Window, fieldSize: Size, pos: Pos) -> PlanarSceneNode {
        let mut circle = window.add_circle(CELL_SIZE / 2.0);
        circle.append_translation(&AppState::translate(fieldSize, pos));
        circle.set_color(0.0, 1.0, 0.0);
        circle
    }

    pub fn translate(fieldSize: Size, pos: Pos) -> Translation2<f32> {
        Translation2::from(Vector2::new(pos.0 as f32 * CELL_SIZE - CELL_SIZE / 2.0 - CELL_SIZE * fieldSize.0 as f32 / 2.0,
                                        pos.1 as f32 * CELL_SIZE - CELL_SIZE / 2.0 - CELL_SIZE * fieldSize.1 as f32 / 2.0))
    }
}


impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        for mut event in window.events().iter() {
            if let Some(direction) = match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    event.inhibited = true; // override the default keyboard handler
                    match button {
                        Key::Q => {
                            window.close();
                            None
                        }
                        Key::A => {
                            self.autopilot = !self.autopilot;
                            None
                        }
                        Key::Left => Some(Direction::Left),
                        Key::Up => Some(Direction::Up),
                        Key::Right => Some(Direction::Right),
                        Key::Down => Some(Direction::Down),
                        _ => None
                    }
                }
                _ => None
            } { self.game.set_direction(direction); }
        }
        if self.autopilot {
            self.game.set_direction(build_path(self.size, &self.game.get_snake(), self.game.get_food_pos()));
        }
        match self.game.move_snake() {
            MoveResult::Ok { new_pos } => {
                println!("Move");
                window.remove_planar_node(&mut self.snake.pop_back().unwrap());
                self.snake.push_front(AppState::create_snake_rect(window, self.size, new_pos));
            }
            MoveResult::Die => {
                println!("Dead");
                window.close()
            }
            MoveResult::Yummi { new_pos, food } => {
                println!("Yummi");
                self.food.set_local_translation(AppState::translate(self.size, food));
                self.snake.push_front(AppState::create_snake_rect(window, self.size, new_pos));
            }
        }
    }
}

fn main() {
    let mut window = Window::new("Snake-again");

    window.set_light(Light::StickToCamera);

    let state = AppState::new(&mut window, (100, 100));

    window.render_loop(state)
}