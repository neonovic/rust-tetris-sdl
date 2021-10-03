use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, InitFlag, LoadTexture};
use std::collections::VecDeque;
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Shapes {
    Elko([[u8; 3]; 2]),
}

#[derive(Debug)]
struct Piece {
    position: Point,
    shape: Shapes,
    shape_width: i32,
    speed: i32,
    direction: VecDeque<Direction>,
}

fn render(canvas: &mut WindowCanvas, color: Color, piece: &Piece) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let start_position = piece.position + Point::new(width as i32 / 2, 0)
        - Point::new(piece.shape_width * 20 / 2, 0);

    canvas.set_draw_color(Color::BLUE);
    let mut piece_position = start_position;
    match piece.shape {
        Shapes::Elko(v) => {
            for row in v {
                for point in row {
                    dbg!(piece_position);
                    if point == 1 {
                        canvas.fill_rect(Rect::new(piece_position.x, piece_position.y, 20, 20));
                    }
                    piece_position += Point::new(20, 0);
                }
                piece_position -= Point::new(piece.shape_width * 20, -20);
            }
        }
    }

    canvas.present();

    Ok(())
}

fn update_piece(piece: &mut Piece) {
    use self::Direction::*;
    piece.position = match piece.direction.back() {
        Some(Left) => piece.position.offset(-20, 0),
        Some(Right) => piece.position.offset(20, 0),
        Some(Up) => piece.position,
        Some(Down) => piece.position.offset(0, 20),
        None => piece.position,
    };
    piece.position += Point::new(0, 20);
}

// Update player a fixed amount based on their speed.
// WARNING: Calling this function too often or at a variable speed will cause the player's speed
// to be unpredictable!
// fn update_player(player: &mut Piece) {
//     use self::Direction::*;
//     player.position = match player.direction.back() {
//         Some(Left) => player.position.offset(-player.speed, 0),
//         Some(Right) => player.position.offset(player.speed, 0),
//         Some(Up) => player.position.offset(0, -player.speed),
//         Some(Down) => player.position.offset(0, player.speed),
//         None => player.position,
//     }
// }

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 400, 800)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut player = Piece {
        position: Point::new(0, 0),
        shape: Shapes::Elko([[1, 1, 1], [1, 0, 1]]),
        shape_width: 3,
        speed: 0,
        direction: VecDeque::new(),
    };

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(k),
                    repeat: false,
                    ..
                } => {
                    if let Some(direction) = match k {
                        Keycode::Left => Some(Direction::Left),
                        Keycode::Right => Some(Direction::Right),
                        Keycode::Up => Some(Direction::Up),
                        Keycode::Down => Some(Direction::Down),
                        _ => None,
                    } {
                        player.speed = PLAYER_MOVEMENT_SPEED;
                        player.direction.push_back(direction);
                    }
                }
                Event::KeyUp {
                    keycode: Some(k),
                    repeat: false,
                    ..
                } => {
                    if let Some(index) = match k {
                        Keycode::Left => find_index(&player.direction, Direction::Left),
                        Keycode::Right => find_index(&player.direction, Direction::Right),
                        Keycode::Up => find_index(&player.direction, Direction::Up),
                        Keycode::Down => find_index(&player.direction, Direction::Down),
                        _ => None,
                    } {
                        player.direction.remove(index);
                    }
                }
                _ => {}
            }
        }

        // Update
        update_piece(&mut player);

        // Render
        render(&mut canvas, Color::BLACK, &player)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 2));
    }

    Ok(())
}

fn find_index(v: &VecDeque<Direction>, d: Direction) -> Option<usize> {
    for (index, c) in v.iter().enumerate() {
        if c == &d {
            return Some(index);
        }
    }
    None
}
