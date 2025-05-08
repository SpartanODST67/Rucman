mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;
use point::Vector2;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up(Vector2),
    Down(Vector2),
    Left(Vector2),
    Right(Vector2),
}

impl Direction {
    fn up() -> Self {
        Self::Up(Vector2(0, -1))
    }

    fn down() -> Self {
        Self::Down(Vector2(0, 1))
    }

    fn left() -> Self {
        Self::Left(Vector2(-1, 0))
    }

    fn right() -> Self {
        Self::Right(Vector2(1, 0))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Character {
    Rucman,
    Blinky, 
    Pinky, 
    Inky, 
    Clyde,
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
struct CharacterData {
    character: Character,
    position: Vector2,
    facing_direction: Direction,
}

impl From<CharacterData> for char {
    fn from(value: CharacterData) -> Self {
        char::from(value.character)
    }
}
 
impl CharacterData {
    fn new(character: Character) -> Self {
        Self{ position: Vector2(0, 0), facing_direction: Direction::right(), character }
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn set_direction(&mut self, direction: Direction) {
        self.facing_direction = direction;
    }

    fn calculate_facing_position(&self) -> Vector2 {
        let offset = {
            match self.facing_direction {
                Direction::Up(dir) | Direction::Down(dir)
                | Direction::Left(dir) | Direction::Right(dir) => dir,
            }
        };

        self.position + offset
    }
}


fn main() {
    let grid = Grid::new();

    let mut rucman = CharacterData::new(Character::Rucman);
    let mut ghosts = vec![
        CharacterData::new(Character::Inky),
        CharacterData::new(Character::Blinky),
        CharacterData::new(Character::Pinky),
        CharacterData::new(Character::Clyde),
    ];

    ghosts[0].set_position(Vector2(1, 1));
    ghosts[1].set_position(Vector2(1, 2));
    ghosts[2].set_position(Vector2(1, 3));
    ghosts[3].set_position(Vector2(2, 1));
    rucman.set_position(Vector2(2, 2));

    for _ in 0..10 {
        print_screen(&grid, &rucman, &ghosts);
        rucman.set_position(rucman.calculate_facing_position());
        rucman.set_direction(match rucman.facing_direction {
            Direction::Up(_) => Direction::right(), 
            Direction::Right(_) => Direction::down(), 
            Direction::Down(_) => Direction::left(), 
            Direction::Left(_) => Direction::up(), 
        });
    }    
    print_screen(&grid, &rucman, &ghosts);
}

fn print_screen(grid: &Grid, rucman: &CharacterData, ghosts: &Vec<CharacterData>) {
    let mut pass_one = Vec::new();

    for row in grid.get_grid() {
        let mut row_collect = Vec::new();
        for col in row {
            row_collect.push(char::from(*col));
        }
        pass_one.push(row_collect);
    }

    for ghost in ghosts {
        let pos = ghost.position;
        pass_one[pos.1 as usize][pos.0 as usize] = char::from(ghost.character);
    }

    let pos = rucman.position;
    pass_one[pos.1 as usize][pos.0 as usize] = char::from(rucman.character);
    
    for row in pass_one {
        let row_string: String = row.iter().collect();
        println!("{row_string}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_character_data_creation() {
        assert_eq!(CharacterData::new(Character::Rucman), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), character: Character::Rucman});
        assert_eq!(CharacterData::new(Character::Inky), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), character: Character::Inky});
        assert_eq!(CharacterData::new(Character::Pinky), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), character: Character::Pinky});
        assert_eq!(CharacterData::new(Character::Blinky), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), character: Character::Blinky});
        assert_eq!(CharacterData::new(Character::Clyde), CharacterData{position: Vector2(0, 0), facing_direction: Direction::right(), character: Character::Clyde});
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