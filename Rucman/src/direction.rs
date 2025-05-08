use crate::point::Vector2;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up(Vector2),
    Down(Vector2),
    Left(Vector2),
    Right(Vector2),
}

impl Direction {
    pub fn up() -> Self {
        Self::Up(Vector2(0, -1))
    }

    pub fn down() -> Self {
        Self::Down(Vector2(0, 1))
    }

    pub fn left() -> Self {
        Self::Left(Vector2(-1, 0))
    }

    pub fn right() -> Self {
        Self::Right(Vector2(1, 0))
    }
}