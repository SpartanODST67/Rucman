use std::ops::{Add, Sub};

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
    pub fn distance(start: Vector2, end: Vector2) -> f32 {
        let x1 = start.0;
        let x2 = end.0;

        let y1 = start.1;
        let y2 = end.1;

        let c = ((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1)) as f32;
        c.sqrt()
    } 

    pub fn side_distance(start: Vector2, end: Vector2) -> i32 {
        let x1 = start.0;
        let x2 = end.0;

        let y1 = start.1;
        let y2 = end.1;

        (x2 - x1).abs() + (y2 - y1).abs()
    }
    
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

    #[test]
    fn test_dist() {
        assert_eq!(Vector2::distance(Vector2(0, 0), Vector2(1, 1)), 1.41421356237);
        assert_eq!(Vector2::distance(Vector2(1, 1), Vector2(0, 0)), 1.41421356237);
        assert_eq!(Vector2::distance(Vector2(0, 0), Vector2(-1, -1)), 1.41421356237);
        assert_eq!(Vector2::distance(Vector2(-1, -1), Vector2(0, 0)), 1.41421356237);
        assert_eq!(Vector2::distance(Vector2(-2, 1), Vector2(2, 1)), 4.0);
        assert_eq!(Vector2::distance(Vector2(2, -1), Vector2(2, 1)), 2.0);
    }

    #[test]
    fn test_side_dist() {
        assert_eq!(Vector2::side_distance(Vector2(0, 0), Vector2(1, 1)), 2);
        assert_eq!(Vector2::side_distance(Vector2(1, 1), Vector2(0, 0)), 2);
        assert_eq!(Vector2::side_distance(Vector2(0, 0), Vector2(-1, -1)), 2);
        assert_eq!(Vector2::side_distance(Vector2(-1, -1), Vector2(0, 0)), 2);
        assert_eq!(Vector2::side_distance(Vector2(-2, 1), Vector2(2, 1)), 4);
        assert_eq!(Vector2::side_distance(Vector2(2, -1), Vector2(2, 1)), 2);
    }
}