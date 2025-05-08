use std::ops::Add;

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
}