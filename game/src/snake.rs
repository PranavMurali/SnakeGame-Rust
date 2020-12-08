use std::collections::linked_list;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;

const SNAKE_COLOR: Color ={0.00,1.00,0.00,1.00};

pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl Direction{
    pub fn opposite(&self)->Direction{
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}