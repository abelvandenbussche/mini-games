///  A tetris piece
pub struct Piece {
    /// The location of the piece in the game
    center: Point,
    /// The location of the blocks relative to the center
    /// There are always 4 blocks in a piece
    blocks: [Point; 4],
}

/// The different kinds of pieces in the game
pub enum PieceVariant {
    /// The square piece
    O,
    /// The straight piece
    I,
    /// The L piece
    L,
    /// The J piece
    J,
    /// The S piece
    S,
    /// The Z piece
    Z,
    /// The T piece
    T,
}

impl From<PieceVariant> for Piece {
    fn from(value: PieceVariant) -> Self {
        let initial_position = Point(0.0, 0.0);
        match value {
            PieceVariant::O => Self {
                center: initial_position,
                blocks: [
                    Point(0.5, 0.5),
                    Point(0.5, -0.5),
                    Point(-0.5, 0.5),
                    Point(-0.5, -0.5),
                ],
            },
            PieceVariant::I => Self {
                center: initial_position,
                blocks: [
                    Point(1.5, 0.5),
                    Point(0.5, 0.5),
                    Point(-0.5, 0.5),
                    Point(-1.5, 0.5),
                ],
            },
            PieceVariant::L => Self {
                center: initial_position,
                blocks: [
                    Point(0.0, 0.0),
                    Point(0.0, -1.0),
                    Point(0.0, 1.0),
                    Point(-1.0, 1.0),
                ],
            },
            PieceVariant::J => Self {
                center: initial_position,
                blocks: [
                    Point(0.0, 0.0),
                    Point(0.0, -1.0),
                    Point(0.0, 1.0),
                    Point(-1.0, -1.0),
                ],
            },
            PieceVariant::S => Self {
                center: initial_position,
                blocks: [
                    Point(0.0, 0.0),
                    Point(-1.0, 0.0),
                    Point(0.0, 1.0),
                    Point(-1.0, -1.0),
                ],
            },
            PieceVariant::Z => Self {
                center: initial_position,
                blocks: [
                    Point(0.0, 0.0),
                    Point(-1.0, 0.0),
                    Point(0.0, -1.0),
                    Point(-1.0, 1.0),
                ],
            },
            PieceVariant::T => Self {
                center: initial_position,
                blocks: [
                    Point(0.0, 0.0),
                    Point(-1.0, 0.0),
                    Point(0.0, -1.0),
                    Point(0.0, 1.0),
                ],
            },
        }
    }
}

impl Piece {
    /// Create a new piece of a certain type at the given position
    pub fn new(pos: Point, piece_type: PieceVariant) -> Self {
        let mut new_piece: Piece = piece_type.into();
        new_piece.center = pos;
        new_piece
    }
    /// Rotates the piece in the given direction
    pub fn rotate(&mut self, direction: RotationDirection) {}
}

pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

/// A point in a 2D space
struct Point(f32, f32);
