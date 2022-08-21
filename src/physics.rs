use crate::Velocity;
use specs::Join;
use specs::ReadStorage;
use crate::Position;
use specs::System;
use specs::WriteStorage;

pub(crate) struct Physics;
impl<'p> System<'p> for Physics {
  type SystemData = (WriteStorage<'p, Position>, ReadStorage<'p, Velocity>);
  fn run (&mut self, mut data: Self::SystemData) {
    use crate::Direction::*;

    for (pos, vel) in (&mut data.0, &data.1).join() {
      match vel.direction {
        Left => {
          pos.0 = pos.0.offset(-vel.speed, 0);
      }
      Right => pos.0 = pos.0.offset(vel.speed, 0),
      Up => pos.0 = pos.0.offset(0, -vel.speed),
      Down => pos.0 = pos.0.offset(0, vel.speed),
      }
    }
  }
}
