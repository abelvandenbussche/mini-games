use std::{
    io::{self, Write, stdout},
    mem::replace,
};

use rand::RngExt;

use crate::tetris::types::{Piece, PieceVariant};

mod types;

struct GameState {
    board: [[bool; 10]; 20],
    current_piece: Piece,
    next_piece: Piece,
}

impl GameState {
    /// Create a new game state ready to be worked with
    fn new() -> Self {
        Self {
            board: [[false; 10]; 20],
            current_piece: get_random_piece(),
            next_piece: get_random_piece(),
        }
    }
    /// Makes the next piece the current piece and gets a new next piece
    fn next_piece(&mut self) {
        self.current_piece = replace(&mut self.next_piece, get_random_piece());
    }
}

pub fn tetris_main() {
    let mut state = GameState::new();

    // Main game loop
    loop {
        // Getting user input

        // Updating the display
        draw_game_state(&state);
    }
}

/// Returns a random tetris piece
fn get_random_piece() -> Piece {
    let num = rand::rng().random_range(0..5);
    let variant = match num {
        0 => PieceVariant::O,
        1 => PieceVariant::I,
        2 => PieceVariant::T,
        3 => PieceVariant::L,
        4 => PieceVariant::J,
        5 => PieceVariant::Z,
        6 => PieceVariant::S,
        _ => unreachable!(),
    };
    variant.into()
}

/// Draws a full frame to the terminal
fn draw_game_state(state: &GameState) -> io::Result<()> {
    let mut out = stdout();
    let mut output = "\x1b[2J\x1b[H".to_owned();
    for row in state.board {
        for cell in row {
            output.push(if cell { '*' } else { ' ' });
        }
    }
    out.write_all(&mut output.as_bytes())?;
    out.flush()
}
