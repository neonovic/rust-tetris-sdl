use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::time::Duration;

const CANVAS_SIZE: (i32, i32) = (20, 30);
const BOX_SIZE: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug)]
enum Shapes {
    Elko([[u8; 3]; 2]),
}

#[derive(Debug)]
struct Piece {
    position: (i32, i32),
    shape: Shapes,
    shape_width: i32,
    direction: Direction,
    steps: u8,
}

fn convert_coords_to_point(coords: (i32, i32)) -> Point {
    Point::new(coords.0 * BOX_SIZE, coords.1 * BOX_SIZE)
}

fn render(canvas: &mut WindowCanvas, color: Color, piece: &Piece) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let mut piece_position = convert_coords_to_point(piece.position);

    canvas.set_draw_color(Color::BLUE);
    match piece.shape {
        Shapes::Elko(v) => {
            for row in v {
                for point in row {
                    if point == 1 {
                        canvas
                            .fill_rect(Rect::new(
                                piece_position.x,
                                piece_position.y,
                                BOX_SIZE as u32,
                                BOX_SIZE as u32,
                            ))
                            .unwrap();
                    }
                    piece_position += Point::new(BOX_SIZE, 0);
                }
                piece_position -= Point::new(piece.shape_width * BOX_SIZE, -BOX_SIZE);
            }
        }
    }

    canvas.present();

    Ok(())
}

fn update_piece(piece: &mut Piece) {
    use self::Direction::*;
    piece.position = match piece.direction {
        Left => (piece.position.0 - 1, piece.position.1),
        Right => (piece.position.0 + 1, piece.position.1),
        Up => piece.position,
        Down => (piece.position.0, piece.position.1 + 1),
        None => piece.position,
    };
    piece.direction = None;

    // Only once per 4 game loops we want to update falling of the piece one step down
    piece.steps += 1;
    if piece.steps == 4 {
        piece.position = (piece.position.0, piece.position.1 + 1);
        piece.steps = 1;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(
            "game tutorial",
            (CANVAS_SIZE.0 * BOX_SIZE) as u32,
            (CANVAS_SIZE.1 * BOX_SIZE) as u32,
        )
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut player = Piece {
        position: (CANVAS_SIZE.0 / 2 - 2, 0),
        shape: Shapes::Elko([[1, 1, 1], [1, 0, 1]]),
        shape_width: 3,
        direction: Direction::None,
        steps: 1,
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
                    keycode: Some(k), ..
                } => {
                    if let Some(direction) = match k {
                        Keycode::Left => Some(Direction::Left),
                        Keycode::Right => Some(Direction::Right),
                        Keycode::Up => Some(Direction::Up),
                        Keycode::Down => Some(Direction::Down),
                        _ => None,
                    } {
                        player.direction = direction;
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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 8));
    }

    Ok(())
}
