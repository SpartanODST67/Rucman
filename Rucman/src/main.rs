mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;
use point::Vector2;

mod direction;
use direction::Direction;

mod character;
use character::{Character, CharacterData};

fn main() {
    let grid = Grid::new();

    let mut rucman = CharacterData::new(Character::Rucman);
    let mut ghosts = vec![
        CharacterData::new(Character::Inky),
        CharacterData::new(Character::Blinky),
        CharacterData::new(Character::Pinky),
        CharacterData::new(Character::Clyde),
    ];

    ghosts[0].set_position(Vector2(0, 0));
    ghosts[1].set_position(Vector2(0, 0));
    ghosts[2].set_position(Vector2(0, 0));
    ghosts[3].set_position(Vector2(0, 0));
    rucman.set_position(Vector2(1, 1));

    for _ in 0..10 {
        print_screen(&grid, &rucman, &ghosts);
        rucman.set_position(rucman.calculate_facing_position());
        rucman.set_direction(match rucman.get_direction() {
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

    for row in grid.get_maze() {
        let mut row_collect = Vec::new();
        for col in row {
            row_collect.push(char::from(*col));
        }
        pass_one.push(row_collect);
    }

    for ghost in ghosts {
        let pos = ghost.get_position();
        pass_one[pos.1 as usize][pos.0 as usize] = char::from(ghost.get_character());
    }

    let pos = rucman.get_position();
    pass_one[pos.1 as usize][pos.0 as usize] = char::from(rucman.get_character());
    
    for row in pass_one {
        let row_string: String = row.iter().collect();
        println!("{row_string}");
    }
}