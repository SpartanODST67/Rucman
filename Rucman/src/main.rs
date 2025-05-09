use std::{thread::sleep, time::Duration, io};

mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;
use point::Vector2;

mod direction;
use direction::Direction;

mod character;
use character::{Character, CharacterData};

fn main() {
    let mut grid = Grid::new();

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

    let mut lives: u8 = 3;
    let mut score: u32 = 0;

    while lives > 0 {
        print_screen(&grid, &rucman, &ghosts, &score, &lives);
        
        let next_dir = take_input();
        match next_dir {
            Some(dir) => rucman.set_direction(dir),
            None => {},
        }

        let next_pos = rucman.calculate_facing_position();
        if grid.is_valid_pos(&next_pos) { rucman.set_position(next_pos) };

        let eatten = grid.eat(&rucman.get_position());
        match eatten {
            Ok(pellet) => {
                match pellet {
                    GridPoint::Pellet => score += 5,
                    GridPoint::PowerPellet => {
                        score += 10;
                    },
                    _ => {},
                }
            },
            Err(invalid) => {
                match invalid {
                    _ => {},
                }
            },
        }

        sleep(Duration::new(0, 500_000_000));
    }
}

fn print_screen(grid: &Grid, rucman: &CharacterData, ghosts: &Vec<CharacterData>, score: &u32, lives: &u8) {
    clearscreen::clear().unwrap();
    
    let mut pass_one = Vec::new();

    /* What we're doing here is like painting a landscape. We start with painting the background
    and then we put the details and subjects over it.*/
    // Collect the maze (the background)
    for row in grid.get_maze() {
        let mut row_collect = Vec::new();
        for col in row {
            row_collect.push(char::from(*col));
        }
        pass_one.push(row_collect);
    }

    //Place the ghosts and rucman over the maze (the subjects)
    for ghost in ghosts {
        let pos = ghost.get_position();
        pass_one[pos.1 as usize][pos.0 as usize] = char::from(ghost);
    }

    let pos = rucman.get_position();
    pass_one[pos.1 as usize][pos.0 as usize] = char::from(rucman);
    
    // Convert collected data into strings and print it.
    let mut i = 0;
    for row in pass_one {
        let mut row_string: String = row.iter().collect();
        if i == 1 { row_string.push_str(format!(" Score: {score}").as_str()); }
        if i == 2 { row_string.push_str(format!(" Lives: {lives}").as_str()); }

        println!("{row_string}");
        i += 1;
    }
}

fn take_input() -> Option<Direction> {
    let mut input = String::new();
    let read = io::stdin().read_line(&mut input);
    match read {
        Ok(_) => {
            match input.trim() {
                "w" => Some(Direction::up()),
                "a" => Some(Direction::left()),
                "s" => Some(Direction::down()),
                "d" => Some(Direction::right()),
                _ => None,
            }
        }
        Err(_) => None,
    }
}