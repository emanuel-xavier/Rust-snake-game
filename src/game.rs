use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOT_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    foot_position: (i32, i32),
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            foot_position: (4, 6),
            food_exists: true,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        if self.food_exists {
            draw_block(
                FOOT_COLOR,
                self.foot_position.0,
                self.foot_position.1,
                ctx,
                g,
            );
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();

        if self.food_exists && self.foot_position.0 == head_x && self.foot_position.1 == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            // println!("CHECK_EATING: the snake ate a fruit");
        }
    }

    fn check_if_snake_is_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tails(next_x, next_y) {
            // println!("CHECK_IF_SNAKE_IS_ALIVE: {}-{} overlap", next_x, next_y);
            return false;
        }

        next_y > 0 && next_x > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x: i32;
        let mut new_y: i32;

        loop {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
            if !self.snake.overlap_tails(new_x, new_y) {
                // println!("ADD_FOOD: food overlap");
                break;
            }
        }

        self.foot_position = (new_x, new_y);
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        // println!("{:?}", self.snake);

        if self.check_if_snake_is_alive(dir) {
            // println!("UPDATE_SNAKE: the snake is alive");
            self.snake.move_foward(dir);
            self.check_eating();
        } else {
            // println!("UPDATE_SNAKE: the snake is dead");
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        // println!("RESSTART: restarting the game");
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.foot_position = (6, 4);
        self.game_over = false;
    }
}
