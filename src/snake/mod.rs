use std::{
    error::Error,
    io::{self, Write, stdout},
    thread,
    time::Duration,
};

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

/// Game state
struct State {
    head: Point,
    body: Vec<Point>,
    apple: Point,
    game_dimensions: (u32, u32),
    growing: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Checks if the directions are opposites of each other
    fn is_opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::North => *other == Direction::South,
            Direction::East => *other == Direction::West,
            Direction::South => *other == Direction::North,
            Direction::West => *other == Direction::East,
        }
    }
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
    fn move_snake(&mut self, direction: Direction) -> Result<(), ()> {
        let original_pos = self.head;
        match direction {
            Direction::North => {
                if self.head.1 > 0 {
                    self.head.1 -= 1;
                } else {
                    return Err(());
                }
            }
            Direction::East => {
                if self.head.0 < self.game_dimensions.0 - 1 {
                    self.head.0 += 1;
                } else {
                    return Err(());
                }
            }
            Direction::South => {
                if self.head.1 < self.game_dimensions.1 - 1 {
                    self.head.1 += 1;
                } else {
                    return Err(());
                }
            }
            Direction::West => {
                if self.head.0 > 0 {
                    self.head.0 -= 1;
                } else {
                    return Err(());
                }
            }
        }
        // Remove back and insert at front
        if !self.growing {
            self.body.pop();
        } else {
            self.growing = false;
        }
        self.body.insert(0, original_pos);
        Ok(())
    }
    /// Grow and relocate the apple
    fn collect_apple(&mut self) {
        self.apple = self.random_exclusive_point();
        self.growing = true;
    }
    /// Generate a random point that is not on the snake or apple
    fn random_exclusive_point(&self) -> Point {
        let mut point = Point::random_point(self.game_dimensions.0, self.game_dimensions.1);
        while point == self.head || point == self.apple || self.body.contains(&point) {
            point = Point::random_point(self.game_dimensions.0, self.game_dimensions.1);
        }
        point
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
        // Getting user input
        if let Some(dir) = get_input()
            && !direction.is_opposite(&dir)
        {
            direction = dir;
        }

        // Moving the snake
        let res = state.move_snake(direction);
        if let Err(_) = res {
            break;
        }

        if state.head == state.apple {
            state.collect_apple();
        }
        display_game(&state).expect("Error while updating display");
        thread::sleep(Duration::from_millis(100));
    }
    println!("Game over");
}

/// Get the input from the user in the form of a direction
fn get_input() -> Option<Direction> {
    while event::poll(Duration::ZERO).unwrap() {
        if let Ok(read) = event::read() {
            match read {
                event::Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: KeyEventKind::Press,
                    ..
                }) => match c {
                    'w' | 'k' => return Some(Direction::North),
                    's' | 'j' => return Some(Direction::South),
                    'd' | 'l' => return Some(Direction::East),
                    'a' | 'h' => return Some(Direction::West),
                    _ => {}
                },
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
