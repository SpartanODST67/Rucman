use std::{io, thread::sleep, time::Duration};

mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;

mod direction;
use direction::Direction;

mod character;
use character::{Character, CharacterData, Vulnerability};

mod a_star;

fn main() {
    let mut grid = Grid::new();

    let mut rucman = CharacterData::new(Character::Rucman);
    let mut ghosts = vec![
        CharacterData::new(Character::Inky),
        CharacterData::new(Character::Blinky),
        CharacterData::new(Character::Pinky),
        CharacterData::new(Character::Clyde),
    ];

    let mut lives: u8 = 3;
    let mut score: u32 = 0;
    let mut vulnerability_timer: u32 = 0;
    let vulnerability_length: u32 = 90;
    let mut frames: u128 = 0;

    while lives > 0 {
        print_screen(&grid, &rucman, &ghosts, &score, &lives);
        
        //Move Rucman
        let next_dir = take_input();
        match next_dir {
            Some(dir) => {
                let old = rucman.get_direction();
                rucman.set_direction(dir);
                if !grid.is_valid_pos(&rucman.calculate_facing_position()) {
                    rucman.set_direction(old);
                }
            },
            None => {},
        }
        rucman.rucman_move(&grid);

        //Eat pellets
        let eatten = grid.eat(&rucman.get_position());
        match eatten {
            Ok(pellet) => {
                match pellet {
                    GridPoint::Pellet => score += 5,
                    GridPoint::PowerPellet => {
                        for ghost in ghosts.iter_mut() {
                            ghost.set_vulnerable();
                        }
                        vulnerability_timer = vulnerability_length;
                        score += 10;
                    },
                    _ => {},
                }
            },
            Err(invalid) => {
                match invalid {
                    GridPointError::InconsumableError(attempt) => {
                        match attempt {
                            GridPoint::Teleporter(other) => rucman.set_position(other),
                            _ => {},
                        }
                    }
                    _ => {},
                }
            },
        }

        check_collision(&mut rucman, &mut ghosts, &mut score, &mut lives);

        //Move Ghosts
        for ghost in ghosts.iter_mut() {
            match ghost.get_vulnerability() {
                Vulnerability::Invulnerable => ghost.ghost_move(&grid, rucman.get_position(), rucman.get_direction()),
                Vulnerability::Vulnerable => {
                    if frames % 2 == 0 {
                        ghost.ghost_move(&grid, rucman.get_position(), rucman.get_direction());
                    }
                    if vulnerability_timer == 0 {
                        ghost.set_invulnerable();
                    }
                }
            }            
        }

        check_collision(&mut rucman, &mut ghosts, &mut score, &mut lives);

        if vulnerability_timer > 0 { vulnerability_timer -= 1; }
        if frames == u128::MAX { frames = 0; } //Probably would never happen. Essentially overflow anyway, but this is to define what to happen on overflow.
        else { frames += 1; }

        sleep(Duration::new(0, 500_000_000));
    }
    println!("Game over! Score: {}", score);
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

fn check_collision(rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>, score: &mut u32, lives: &mut u8) {
    for ghost in ghosts.iter_mut() {
        if ghost.get_position() == rucman.get_position() {
            match ghost.get_vulnerability() {
                Vulnerability::Vulnerable => {
                    *score += 100;
                    reset_character(ghost);
                }
                Vulnerability::Invulnerable => {
                    *score -= if *score < 1000 { *score } else { 1000 };
                    *lives -= 1;
                    reset_characters(rucman, ghosts);
                    break;
                }
            }
        }
    }
}

fn reset_game(grid: &mut Grid, rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>) {
    *grid = Grid::new();
    reset_characters(rucman, ghosts);
}

fn reset_characters(rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>) {
    reset_character(rucman);
    for ghost in ghosts {
        reset_character(ghost);
    }
}

fn reset_character(character: &mut CharacterData) {
    *character = CharacterData::new(character.get_character());
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