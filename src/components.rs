
use std::default;

use crate::Rect;
use crate::Point;
use specs::VecStorage;
use specs::prelude::*;
use specs_derive::Component;
#[derive(Debug)]
//traits

pub struct Player {
   pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
   pub current_frame: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
     Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
   pub current_frame: usize,
   pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}


#[derive(Component, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub enum MovementCommand {
  Stop,
  Move(Direction),
}

impl Default for MovementCommand {
  fn default() -> Self {
        MovementCommand::Stop
  }
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
   pub speed: i32,
   pub direction: Direction
}


#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;
