use crate::point::Vector2;
use crate::Direction;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Character {
    Rucman,
    Blinky, 
    Pinky, 
    Inky, 
    Clyde,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Vulnerability {
    Invulnerable,
    Vulnerable,
}

impl From<Character> for char {
    fn from(value: Character) -> Self {
        match value {
            Character::Rucman => 'R',
            _ => 'C',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterData {
    character: Character,
    vulnerability: Vulnerability,
    position: Vector2,
    facing_direction: Direction,
}

impl From<CharacterData> for char {
    fn from(value: CharacterData) -> Self {
        match value.character {
            Character::Rucman => 'R',
            _ => {
                match value.vulnerability {
                    Vulnerability::Invulnerable => 'M',
                    Vulnerability::Vulnerable => 'W',
                }
            }
        }
    }
}

impl From<&CharacterData> for char {
    fn from(value: &CharacterData) -> Self {
        match value.character {
            Character::Rucman => 'R',
            _ => {
                match value.vulnerability {
                    Vulnerability::Invulnerable => 'M',
                    Vulnerability::Vulnerable => 'W',
                }
            }
        }
    }
}
 
impl CharacterData {
    pub fn new(character: Character) -> Self {
        Self{ position: Vector2(0, 0), facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, character }
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.facing_direction = direction;
    }

    pub fn get_direction(&self) -> Direction{
        self.facing_direction
    }

    pub fn get_character(&self) -> Character {
        self.character
    }

    pub fn calculate_facing_position(&self) -> Vector2 {
        let offset = {
            match self.facing_direction {
                Direction::Up(dir) | Direction::Down(dir)
                | Direction::Left(dir) | Direction::Right(dir) => dir,
            }
        };

        self.position + offset
    }

    pub fn rucman_move(&mut self, grid: &Grid) {
        let next_pos = self.calculate_facing_position();
        if grid.is_valid_pos(&next_pos) { self.set_position(next_pos) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_character_data_creation() {
        assert_eq!(CharacterData::new(Character::Rucman), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, character: Character::Rucman});
        assert_eq!(CharacterData::new(Character::Inky), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, character: Character::Inky});
        assert_eq!(CharacterData::new(Character::Pinky), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, character: Character::Pinky});
        assert_eq!(CharacterData::new(Character::Blinky), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, character: Character::Blinky});
        assert_eq!(CharacterData::new(Character::Clyde), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, character: Character::Clyde});
    }

    #[test]
    fn test_set_position() {
        let mut test_char = CharacterData::new(Character::Rucman);
        test_char.set_position(Vector2(1, 1));
        assert_eq!(test_char.position, Vector2(1, 1));
        test_char.set_position(Vector2(-1, -1));
        assert_eq!(test_char.position, Vector2(-1, -1));
    }

    #[test]
    fn test_set_direction() {
        let mut test_char = CharacterData::new(Character::Rucman);
        assert_eq!(test_char.facing_direction, Direction::right());
        test_char.set_direction(Direction::up());
        assert_eq!(test_char.facing_direction, Direction::up());
        test_char.set_direction(Direction::down());
        assert_eq!(test_char.facing_direction, Direction::down());
        test_char.set_direction(Direction::left());
        assert_eq!(test_char.facing_direction, Direction::left());
        test_char.set_direction(Direction::right());
        assert_eq!(test_char.facing_direction, Direction::right());
    }

    #[test]
    fn test_calculate_facing_direction() {
        let mut test_char = CharacterData::new(Character::Rucman);
        assert_eq!(test_char.calculate_facing_position(), Vector2(1, 0));
        test_char.set_direction(Direction::up());
        assert_eq!(test_char.calculate_facing_position(), Vector2(0, -1));
        test_char.set_direction(Direction::down());
        assert_eq!(test_char.calculate_facing_position(), Vector2(0, 1));
        test_char.set_direction(Direction::left());
        assert_eq!(test_char.calculate_facing_position(), Vector2(-1, 0));
    }
}