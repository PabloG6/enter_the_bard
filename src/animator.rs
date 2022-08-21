

use specs::Join;
use crate::Sprite;
use crate::Velocity;
use crate::components;
use specs::ReadStorage;
use crate::MovementAnimation;
use specs::System;
use specs::WriteStorage;


pub struct Animator;
impl<'a> System<'a> for Animator {
  type SystemData = (
  WriteStorage<'a, MovementAnimation>,
  WriteStorage<'a, Sprite>,
  ReadStorage<'a, Velocity>,);

    fn run(&mut self, mut data: Self::SystemData) {
      use components::Direction::*;
      for (anim, sprite, vel) in (&mut data.0, &mut data.1, &data.2).join() {
        if vel.speed == 0 {
          continue;
        }

        let frames = match vel.direction {
          Left => &anim.left_frames,
          Right => &anim.right_frames,
          Down => &anim.down_frames,
          Up => &anim.up_frames
           
        };

        anim.current_frame = (anim.current_frame + 1) % frames.len();
        *sprite = frames[anim.current_frame].clone();

      }


    }
  
}

