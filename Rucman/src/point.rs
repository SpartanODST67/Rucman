use std::ops::{Add, Sub};

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2(pub i32, pub i32);

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self (
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self (
            self.0 - rhs.0,
            self.1 - rhs.1,
        )
    }
}

impl Vector2 {
    pub fn forward(&self, direction: Direction) -> Vector2 {
        match direction {
            Direction::Up(pos) |
            Direction::Down(pos) |
            Direction::Left(pos) |
            Direction::Right(pos) => pos + *self,
        }
    }

    pub fn back(&self, direction: Direction) -> Vector2 {
        match direction {
            Direction::Up(pos) | 
            Direction::Down(pos) | 
            Direction::Left(pos) | 
            Direction::Right(pos) => *self - pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Vector2(0, 0) + Vector2(1, 1), Vector2(1, 1));
        assert_eq!(Vector2(1, 1) + Vector2(0, 0), Vector2(1, 1));
        assert_eq!(Vector2(0, 0) + Vector2(-1, -1), Vector2(-1, -1));
        assert_eq!(Vector2(-1, -1) + Vector2(0, 0), Vector2(-1, -1));
        assert_eq!(Vector2(1, 1) + Vector2(-1, -1), Vector2(0, 0));
        assert_eq!(Vector2(-1, -1) + Vector2(1, 1), Vector2(0, 0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vector2(0, 0) - Vector2(1, 1), Vector2(-1, -1));
        assert_eq!(Vector2(1, 1) - Vector2(0, 0), Vector2(1, 1));
        assert_eq!(Vector2(0, 0) - Vector2(-1, -1), Vector2(1, 1));
        assert_eq!(Vector2(-1, -1) - Vector2(0, 0), Vector2(-1, -1));
        assert_eq!(Vector2(1, 1) - Vector2(-1, -1), Vector2(2, 2));
        assert_eq!(Vector2(-1, -1) - Vector2(1, 1), Vector2(-2, -2));
    }

    #[test]
    fn test_forward() {
        let mut origin = Vector2(0, 0);
        origin = origin.forward(Direction::up());
        assert_eq!(origin, Vector2(0, -1));
        origin = origin.forward(Direction::left());
        assert_eq!(origin, Vector2(-1, -1));
        origin = origin.forward(Direction::down());
        assert_eq!(origin, Vector2(-1, 0));
        origin = origin.forward(Direction::right());
        assert_eq!(origin, Vector2(0, 0));
        origin = origin.forward(Direction::up()).forward(Direction::up());
        assert_eq!(origin, Vector2(0, -2));
    }

    #[test]
    fn test_back() {
        let mut origin = Vector2(0, 0);
        origin = origin.back(Direction::up());
        assert_eq!(origin, Vector2(0, 1));
        origin = origin.back(Direction::left());
        assert_eq!(origin, Vector2(1, 1));
        origin = origin.back(Direction::down());
        assert_eq!(origin, Vector2(1, 0));
        origin = origin.back(Direction::right());
        assert_eq!(origin, Vector2(0, 0));
        origin = origin.back(Direction::up()).back(Direction::up());
        assert_eq!(origin, Vector2(0, 2));
    }
}