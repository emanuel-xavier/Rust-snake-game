use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];
const SNAKE_HEAD_COLOR: Color = [0.00, 0.90, 0.00, 1.00];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        for i in (0..3).rev() {
            body.push_back(Block { x: x + i, y })
        }
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        let (head_x, head_y) = self.head_position();
        draw_block(SNAKE_HEAD_COLOR, head_x, head_y, ctx, g);

        for block in self.body.iter().skip(1) {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_foward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (prev_x, prev_y): (i32, i32) = self.head_position();
        let new_block = match self.direction {
            Direction::Up => Block {
                x: prev_x,
                y: prev_y - 1,
            },
            Direction::Down => Block {
                x: prev_x,
                y: prev_y + 1,
            },
            Direction::Right => Block {
                x: prev_x + 1,
                y: prev_y,
            },
            Direction::Left => Block {
                x: prev_x - 1,
                y: prev_y,
            },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (mut head_x, mut head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        // println!("NEXT_HEAD: previos: {}-{}", head_x, head_y);

        (head_x, head_y) = match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
        };

        // println!("NEXT_HEAD: next: {}-{}", head_x, head_y);

        (head_x, head_y)
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tails(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        false
    }
}
