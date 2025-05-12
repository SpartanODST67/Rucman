use crate::point::Vector2;

/// Denotes a direction an entity can go on the grid.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up(Vector2),
    Down(Vector2),
    Left(Vector2),
    Right(Vector2),
}

impl Direction {
    /// Returns a Direction with the Vector2 values of 0, -1
    pub fn up() -> Self {
        Self::Up(Vector2(0, -1))
    }

    /// Returns a Direction with the Vector2 values of 0, 1
    pub fn down() -> Self {
        Self::Down(Vector2(0, 1))
    }

    /// Returns a Direction with the Vector2 values of -1, 0
    pub fn left() -> Self {
        Self::Left(Vector2(-1, 0))
    }

    /// Returns a Direction with the Vector2 values of 1, 0
    pub fn right() -> Self {
        Self::Right(Vector2(1, 0))
    }

    /// Returns a Vec of Direction instances to be iterated through.
    pub fn directions() -> Vec<Self>{
        vec![
            Direction::up(),
            Direction::down(),
            Direction::left(),
            Direction::right(),
        ]
    }
}