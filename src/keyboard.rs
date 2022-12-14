use specs::Join;
use specs::WriteStorage;
use crate::components::MovementCommand;
use crate::components::{Velocity, KeyboardControlled};
use specs::ReadStorage;
use specs::{ReadExpect, System};

pub struct Keyboard;

const PLAYER_MOVEMENT_SPEED: i32 = 20;
impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = PLAYER_MOVEMENT_SPEED;
                    vel.direction = direction;
                }

                MovementCommand::Stop => vel.speed = 0,
            }
        }
    }
}
