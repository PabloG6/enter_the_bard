use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use std::time::Duration;
const PLAYER_SPEED: i32 = 20;
fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();


    let current_frame = Rect::new(
        player.position.x() + frame_width as i32 * player.current_frame,
        player.position.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height,
    );

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);


    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);

    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();
    Ok(())
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}
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

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;
    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 0,
        current_frame: 0,
        direction: Direction::Down,
    };
    'running: loop {
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
                    player.direction = Direction::Down;
                    player.speed = PLAYER_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Up;
                    player.speed = PLAYER_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Right;
                    player.speed = PLAYER_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Left;
                    player.speed = PLAYER_SPEED;
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
                    player.speed = 0;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        //render events here
        i = (i + 1) % 255;
        update_player(&mut player);
        render(&mut canvas, Color::RGB(0, 0, 0), &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
    current_frame: i32,
}

fn update_player(player: &mut Player) {
    use self::Direction::*;
     match player.direction {
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        }
        Right => player.position = player.position.offset(player.speed, 0),
        Up => player.position = player.position.offset(0, -player.speed),
        Down => player.position = player.position.offset(0, player.speed),
    }


    if player.speed != 0 {
        log::debug!("current frame: {}", player.current_frame);

        player.current_frame = (player.current_frame + 1) % 3;
        log::debug!("current frame: {}", player.current_frame)
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
