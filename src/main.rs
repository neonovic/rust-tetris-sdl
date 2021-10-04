use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::time::Duration;

const CANVAS_SIZE: (i32, i32) = (20, 30);
const BOX_SIZE: i32 = 20;

struct Gamestate {
    area: [[i32; CANVAS_SIZE.0 as usize]; CANVAS_SIZE.1 as usize],
    store: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

struct Pieces {
    list: Vec<Vec<Vec<u8>>>,
}

#[derive(Debug)]
struct Piece {
    position: (i32, i32),
    shape: Vec<Vec<u8>>,
    shape_width: i32,
    direction: Direction,
    steps: u8,
}

fn convert_coords_to_point(coords: (i32, i32)) -> Point {
    Point::new(coords.0 * BOX_SIZE, coords.1 * BOX_SIZE)
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    piece: &Piece,
    gamestate: &Gamestate,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let mut piece_position = convert_coords_to_point(piece.position);

    canvas.set_draw_color(Color::BLUE);
    for row in &piece.shape {
        for point in row {
            if *point == 1 {
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

    for (y, row) in gamestate.area.iter().enumerate() {
        for (x, a) in row.iter().enumerate() {
            if gamestate.area[y][x] == 1 {
                let area_point = convert_coords_to_point((x as i32, y as i32));
                canvas
                    .fill_rect(Rect::new(
                        area_point.x,
                        area_point.y,
                        BOX_SIZE as u32,
                        BOX_SIZE as u32,
                    ))
                    .unwrap();
            }
        }
    }

    canvas.present();

    Ok(())
}

fn store_piece_to_game_state(piece: &Piece, gamestate: &mut Gamestate) {
    for (y, row) in piece.shape.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if *p == 1 {
                gamestate.area[piece.position.1 as usize + y][piece.position.0 as usize + x] = 1;
            }
        }
    }
}

fn check_pieces_collision_left(piece: &Piece, gamestate: &Gamestate) -> bool {
    for (y, row) in piece.shape.iter().enumerate() {
        if row[0] == 1
            && gamestate.area[piece.position.1 as usize + y][piece.position.0 as usize - 1] == 1
        {
            return true;
        }
    }
    false
}

fn check_pieces_collision_down(piece: &Piece, gamestate: &Gamestate) -> bool {
    for (y, row) in piece.shape.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if *p == 1
                && gamestate.area[piece.position.1 as usize + y + 1][piece.position.0 as usize + x]
                    == 1
            {
                return true;
            }
        }
    }
    false
}

fn check_pieces_collision_right(piece: &Piece, gamestate: &Gamestate) -> bool {
    for (y, row) in piece.shape.iter().enumerate() {
        if row[row.len() - 1] == 1
            && gamestate.area[piece.position.1 as usize + y]
                [piece.position.0 as usize + piece.shape_width as usize]
                == 1
        {
            return true;
        }
    }
    false
}

fn rotate(piece: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut piece_rotated: Vec<Vec<u8>> = vec![];
    for y in 0..piece.len() {
        piece_rotated.push(vec![]);
        for x in 0..piece.len() {
            piece_rotated[y].push(piece[x][piece.len() - 1 - y]);
        }
    }
    piece_rotated
}

fn update_piece(piece: &mut Piece, gamestate: &mut Gamestate) {
    use self::Direction::*;
    piece.position = match piece.direction {
        Left => {
            if piece.position.0 == 0 || check_pieces_collision_left(piece, gamestate) {
                piece.position
            } else {
                (piece.position.0 - 1, piece.position.1)
            }
        }
        Right => {
            if piece.position.0 + piece.shape_width == CANVAS_SIZE.0
                || check_pieces_collision_right(piece, gamestate)
            {
                piece.position
            } else {
                (piece.position.0 + 1, piece.position.1)
            }
        }
        Down => {
            if piece.position.1 + piece.shape.len() as i32 >= CANVAS_SIZE.1 - 1
                || check_pieces_collision_down(piece, gamestate)
            {
                piece.position
            } else {
                (piece.position.0, piece.position.1 + 1)
            }
        }
        Up => {
            piece.shape = rotate(&piece.shape);
            piece.position
        }
        None => piece.position,
    };
    piece.direction = None;

    if !gamestate.store {
        for (y, row) in piece.shape.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if *p == 1
                    && (piece.position.1 as usize + y >= CANVAS_SIZE.1 as usize - 1
                        || gamestate.area[piece.position.1 as usize + y + 1]
                            [piece.position.0 as usize + x]
                            == 1)
                {
                    gamestate.store = true;
                }
            }
        }
    }

    // Only once per 4 game loops we want to update falling of the piece one step down
    piece.steps += 1;
    if piece.steps == 4 {
        if gamestate.store {
            store_piece_to_game_state(piece, gamestate);
            piece.position = (CANVAS_SIZE.0 / 2 - 2, 0);
            gamestate.store = false;
        } else {
            piece.position = (piece.position.0, piece.position.1 + 1);
        }
        piece.steps = 1;
    }
}

fn main() -> Result<(), String> {
    let mut gamestate = Gamestate {
        area: [[0; CANVAS_SIZE.0 as usize]; CANVAS_SIZE.1 as usize],
        store: false,
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

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

    let pieces = Pieces {
        list: vec![vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 0, 0]]],
    };

    let mut piece = Piece {
        position: (CANVAS_SIZE.0 / 2 - 2, 0),
        shape: pieces.list[0].clone(),
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
                        piece.direction = direction;
                    }
                }
                _ => {}
            }
        }

        // Update
        update_piece(&mut piece, &mut gamestate);

        // Render
        render(&mut canvas, Color::BLACK, &piece, &gamestate)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 8));
    }

    Ok(())
}
