use std::{
    io::{self, Write, stdout},
    thread,
    time::Duration,
};

use crossterm::event::{self, KeyEvent};

/// Game state
struct State {
    head: Point,
    body: Vec<Point>,
    apple: Point,
    game_dimensions: (u32, u32),
    growing: bool,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl State {
    /// Create a new game state
    fn new(board_size: (u32, u32)) -> Self {
        let head_pos = Point(board_size.0.div_ceil(2), board_size.1.div_ceil(2));
        let mut body = vec![];
        for i in 1..5 {
            body.push(Point(head_pos.0, head_pos.1 + i));
        }
        Self {
            head: head_pos,
            body: body,
            apple: Point::random_point(board_size.0, board_size.1),
            game_dimensions: board_size,
            growing: false,
        }
    }
    fn move_snake(&mut self, direction: Direction) {
        let original_pos = self.head;
        match direction {
            Direction::North => self.head.1 -= 1,
            Direction::East => self.head.0 += 1,
            Direction::South => self.head.1 += 1,
            Direction::West => self.head.0 -= 1,
        };
        // Remove back and insert at front
        self.body.pop();
        self.body.insert(0, original_pos);
    }
}

/// Point on the board
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point(u32, u32);

impl Point {
    /// Generate a point at a random position
    fn random_point(width: u32, height: u32) -> Self {
        let x = rand::random_range(0..width);
        let y = rand::random_range(0..height);
        Self(x, y)
    }
}

pub fn snake_main() {
    let mut state = State::new((10, 10));
    let mut direction = Direction::North;

    // Main game loop
    loop {
        state.move_snake(direction);
        display_game(&state).expect("Error while updating display");
        thread::sleep(Duration::from_millis(100));
    }
}

/// Get the input from the user in the form of a direction
fn get_input() -> Option<Direction> {
    while event::poll(Duration::ZERO).unwrap() {
        if let Ok(read) = event::read() {
            match read {
                _ => {}
            }
        }
    }
    None
}

/// Fully display a game frame
fn display_game(state: &State) -> io::Result<()> {
    let mut out = stdout();
    let mut output = "\x1b[2J\x1b[H".to_owned();
    for y in 0..state.game_dimensions.1 {
        for x in 0..state.game_dimensions.0 {
            let cur_point = Point(x, y);
            if cur_point == state.head {
                output.push('#');
            } else if state.body.contains(&cur_point) {
                output.push('*');
            } else if cur_point == state.apple {
                output.push('O');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    out.write_all(&mut output.as_bytes())?;
    out.flush()?;
    Ok(())
}
