mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::Builder;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;

use crate::components::*;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("celeste 2.0", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video game");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");


    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);
    let player_animations = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Right,
        ),
    };
    let texture_creator = canvas.texture_creator();
    let textures = [texture_creator.load_texture("assets/bardo.png")?];
   
    let mut dispatcher = DispatcherBuilder::new().with(keyboard::Keyboard, "Keyboard", &[]).
    with(physics::Physics, "Physics", &["Keyboard"]).
    with(animator::Animator,"Animator", &["Keyboard"] ).build();
    

    let mut world = World::new();
    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Sprite>();
    world.register::<MovementAnimation>();
    world.register::<KeyboardControlled>();
    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
        })
        .with(player_animations.right_frames[0].clone())
        .with(player_animations)
        .build();

    'running: loop {
        let mut movement_command: Option<MovementCommand> = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                   movement_command = Some(MovementCommand::Move(Direction::Down))
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up))
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                   movement_command = Some(MovementCommand::Move(Direction::Right))
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left))
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                   movement_command = Some(MovementCommand::Stop)
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        i = (i + 1) % 255;

        dispatcher.dispatch(&mut world);
        world.maintain();
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}

//functions
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }

}





fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet: spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}
