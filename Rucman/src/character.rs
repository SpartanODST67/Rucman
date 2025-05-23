use std::vec;
use std::fmt::Display;

use crate::grid::grid::Grid;
use crate::a_star;
use crate::point::Vector2;
use crate::Direction;

/// Denotes which rucman character is currently represented.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Character {
    Rucman,
    Blinky, 
    Pinky, 
    Inky, 
    Clyde,
}

/// Denotes if a ghost is vulnerable or invulnerable. Both states have different behaviour when collided with.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Vulnerability {
    Invulnerable,
    Vulnerable,
}

/// Denotes if the ghost should chase rucman or scatter.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GhostMode {
    Scatter,
    Chase,
}

impl From<Character> for char {
    fn from(value: Character) -> Self {
        match value {
            Character::Rucman => 'R',
            Character::Inky => 'I',
            Character::Blinky => 'B',
            Character::Pinky => 'P',
            Character::Clyde => 'C',
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Character::Rucman => "Rucman",
            Character::Inky => "Inky",
            Character::Blinky => "Blinky",
            Character::Pinky => "Pinky",
            Character::Clyde => "Clyde",
        };
        
        write!(f, "{}", name)
    }
}

/// Stores character data. Most data is relevant for the ghosts.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterData {
    character: Character,
    vulnerability: Vulnerability,
    ghost_mode: GhostMode,
    position: Vector2,
    scatter_position: Vector2,
    nav_path: Vec<Vector2>,
    facing_direction: Direction,
}

impl From<&CharacterData> for char {
    fn from(value: &CharacterData) -> Self {
        match value.character {
            Character::Rucman => 'R',
            _ => {
                match value.vulnerability {
                    Vulnerability::Invulnerable => char::from(value.character),
                    Vulnerability::Vulnerable => char::from(value.character).to_ascii_lowercase(),
                }
            }
        }
    }
}
 
impl CharacterData {
    /// Creates and initializes new character data depending on the provided character.
    pub fn new(character: Character) -> Self {
        let position = match character { // Starting position of each character.
            Character::Inky => Vector2(12, 11),
            Character::Blinky => Vector2(13, 9),
            Character::Pinky => Vector2(13, 11),
            Character::Clyde => Vector2(14, 11),
            Character::Rucman => Vector2(13, 20),
        };

        let scatter_position = match character { // Position each character goes to during scatter mode.
            Character::Inky => Vector2(25, 25),
            Character::Blinky => Vector2(25, 1),
            Character::Pinky => Vector2(1, 1),
            Character::Clyde => Vector2(1, 25),
            Character::Rucman => Vector2(0, 0),
        };
        
        Self{ vulnerability: Vulnerability::Invulnerable, ghost_mode:GhostMode::Scatter, facing_direction: Direction::right(), nav_path: vec![], character, position, scatter_position }
    }

    /// Sets the position of the character.
    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    /// Gets the position of the character.
    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    /// Sets the direction of the character.
    pub fn set_direction(&mut self, direction: Direction) {
        self.facing_direction = direction;
    }

    /// Sets the direction of the character only if they are allowed to move in the new direction.
    pub fn set_direction_if_valid(&mut self, direction: Direction, grid: &Grid) {
        let old = self.get_direction();
        self.set_direction(direction);
        if !grid.is_valid_pos(&self.calculate_facing_position()) {
            self.set_direction(old);
        }
    }

    /// Gets the direction the character is currently facing.
    pub fn get_direction(&self) -> Direction{
        self.facing_direction
    }

    /// Gets which character is represented by this character data.
    pub fn get_character(&self) -> Character {
        self.character
    }

    /// Calculates a position 1 unit away in relation to the character's current direction.
    pub fn calculate_facing_position(&self) -> Vector2 {
        let offset = {
            match self.facing_direction {
                Direction::Up(dir) | Direction::Down(dir)
                | Direction::Left(dir) | Direction::Right(dir) => dir,
            }
        };

        self.position + offset
    }

    /// Moves in the direction the character is currently facing.
    pub fn rucman_move(&mut self, grid: &Grid) {
        let next_pos = self.calculate_facing_position();
        if grid.is_valid_pos(&next_pos) { self.set_position(next_pos) };
    }

    /// Move based on the current ghost mode.
    pub fn ghost_move(&mut self, grid: &mut Grid, position: Vector2, rucman_direction: Direction) {
        match self.ghost_mode {
            GhostMode::Chase => self.ghost_chase(grid, position, rucman_direction),
            GhostMode::Scatter => self.ghost_scatter(grid),
        }
    }

    /// Move towards the provided position.
    fn ghost_chase(&mut self, grid: &mut Grid, position: Vector2, rucman_direction: Direction) {
        // Only update path if remaining path is short or you're close to provided position.
        // So the ghosts aren't as relentless in their chases.
        if self.nav_path.len() > 5 && Vector2::distance(self.position, position) > 5.0 {
            let next = self.nav_path.pop().unwrap(); //Shouldn't be none since size is already checked.
            self.set_position(next);
            return;
        }
        
        // Determine target position
        let target: Vector2 = {
            match self.character {
                Character::Blinky | Character::Rucman => position, // Direct chase
                Character::Inky => { // 1 behind, slower chase
                    if Vector2::distance(self.position, position) < 2.0 { 
                        position
                    }
                    else {
                        let ambush = position.back(rucman_direction);
                        if !grid.is_valid_pos(&ambush) { 
                            position 
                        }
                        else {
                            ambush
                        }
                    }
                },
                Character::Pinky => { // 2 ahead, ambush chase
                    if Vector2::distance(self.position, position) < 2.0 { 
                        position 
                    }
                    else {
                        let mut cut_off = position.forward(rucman_direction).forward(rucman_direction);
                        if !grid.is_valid_pos(&cut_off) {
                            cut_off = position.forward(rucman_direction);
                            if !grid.is_valid_pos(&cut_off) { 
                                position 
                            }
                            else {
                                cut_off
                            }
                        }
                        else {
                            cut_off
                        }
                    }
                },
                Character ::Clyde => { // Random wander.
                    grid.get_random_position()
                }
            }
        };

        // Use A* to form a path
        match a_star::a_star(&grid, self.position, target, true) {
            Some(path) => {
                self.nav_path = path;
                let next = self.nav_path.pop();
                if next.is_some() {
                    self.set_position(next.unwrap());
                }
            },
            None => { self.nav_path = Vec::new() }
        }
    }

    /// Move towards the character's scatter point.
    fn ghost_scatter(&mut self, grid: &Grid) {
        if self.nav_path.is_empty() {
            self.nav_path = match a_star::a_star(grid, self.position, self.scatter_position, true) {
                Some(path) => path,
                None => Vec::new(),
            };
        }

        if !self.nav_path.is_empty() {
            let next = self.nav_path.pop().unwrap();
            self.set_position(next);
        }

        if self.nav_path.is_empty() {
            self.toggle_ghost_mode();
        }
    }

    /// Set ghost mode to Scatter if in Chase mode and vise versa.
    pub fn toggle_ghost_mode(&mut self) {
        match self.ghost_mode {
            GhostMode::Chase => self.set_scatter_mode(),
            GhostMode::Scatter => self.set_chase_mode(),
        }
    }

    /// Sets ghost mode to Scatter.
    pub fn set_scatter_mode(&mut self) {
        self.nav_path.clear();
        self.ghost_mode = GhostMode::Scatter;
    }

    /// Sets ghost mode to chase.
    pub fn set_chase_mode(&mut self) {
        self.nav_path.clear();
        self.ghost_mode = GhostMode::Chase;
    }

    /// Makes the ghost Vulnerable if they are Invulnerable and vise versa.
    pub fn toggle_vulnerability(&mut self) {
        match self.vulnerability {
            Vulnerability::Vulnerable => self.set_invulnerable(),
            Vulnerability::Invulnerable => self.set_vulnerable(),
        }
    }

    /// Makes the ghost vulnerable and enter scatter mode.
    pub fn set_vulnerable(&mut self) {
        self.set_scatter_mode();
        self.vulnerability = Vulnerability::Vulnerable;
    }

    /// Makes the ghost invulnerable and enter chase mode.
    pub fn set_invulnerable(&mut self) {
        self.set_chase_mode();
        self.vulnerability = Vulnerability::Invulnerable;
    }

    /// Gets the current vulnerability of the ghost.
    pub fn get_vulnerability(&self) -> Vulnerability {
        self.vulnerability
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests if character data creation works properly.
    #[test]
    fn test_character_data_creation() {
        assert_eq!(CharacterData::new(Character::Rucman), CharacterData{position: Vector2(13, 20), scatter_position: Vector2(0, 0), nav_path: vec![], facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, ghost_mode: GhostMode::Scatter, character: Character::Rucman});
        assert_eq!(CharacterData::new(Character::Inky), CharacterData{position: Vector2(12, 11), scatter_position: Vector2(25, 25), nav_path: vec![], facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, ghost_mode: GhostMode::Scatter, character: Character::Inky});
        assert_eq!(CharacterData::new(Character::Pinky), CharacterData{position: Vector2(13, 11), scatter_position: Vector2(1, 1), nav_path: vec![], facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, ghost_mode: GhostMode::Scatter, character: Character::Pinky});
        assert_eq!(CharacterData::new(Character::Blinky), CharacterData{position: Vector2(13, 9), scatter_position: Vector2(25, 1), nav_path: vec![], facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, ghost_mode: GhostMode::Scatter, character: Character::Blinky});
        assert_eq!(CharacterData::new(Character::Clyde), CharacterData{position: Vector2(14, 11), scatter_position: Vector2(1, 25), nav_path: vec![], facing_direction: Direction::right(), vulnerability: Vulnerability::Invulnerable, ghost_mode: GhostMode::Scatter, character: Character::Clyde});
    }

    /// Tests if character's position is set properly.
    #[test]
    fn test_set_position() {
        let mut test_char = CharacterData::new(Character::Rucman);
        test_char.set_position(Vector2(1, 1));
        assert_eq!(test_char.position, Vector2(1, 1));
        test_char.set_position(Vector2(-1, -1));
        assert_eq!(test_char.position, Vector2(-1, -1));
    }

    /// Tests if a character's direction is set properly.
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

    /// Tests if calculate_facing_direction properly calculates a position according to set direction.
    #[test]
    fn test_calculate_facing_direction() {
        let mut test_char = CharacterData::new(Character::Rucman);
        test_char.set_position(Vector2(0, 0));
        assert_eq!(test_char.calculate_facing_position(), Vector2(1, 0));
        test_char.set_direction(Direction::up());
        assert_eq!(test_char.calculate_facing_position(), Vector2(0, -1));
        test_char.set_direction(Direction::down());
        assert_eq!(test_char.calculate_facing_position(), Vector2(0, 1));
        test_char.set_direction(Direction::left());
        assert_eq!(test_char.calculate_facing_position(), Vector2(-1, 0));
    }
}