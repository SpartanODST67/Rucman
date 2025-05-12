use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::{self, stdout, stderr};
use std::thread;
use std::thread::{sleep, JoinHandle};

mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;

mod direction;
use direction::Direction;

mod character;
use character::{Character, CharacterData, Vulnerability};

mod a_star;

struct GameManager {
    grid: Arc<Mutex<Grid>>,
    rucman: Arc<Mutex<CharacterData>>,
    ghosts: Arc<Mutex<Vec<CharacterData>>>,
}

fn main() -> io::Result<()> {

    enable_raw_mode()?;

    let grid = Grid::new();

    let rucman = CharacterData::new(Character::Rucman);
    let ghosts = vec![
        CharacterData::new(Character::Inky),
        CharacterData::new(Character::Blinky),
        CharacterData::new(Character::Pinky),
        CharacterData::new(Character::Clyde),
    ];

    let game_manager = GameManager {
        grid: Arc::new(Mutex::new(grid)),
        rucman: Arc::new(Mutex::new(rucman)),
        ghosts: Arc::new(Mutex::new(ghosts)),
    };

    let mut lives: u8 = 3;
    let mut score: u32 = 0;
    let mut vulnerability_timer: u32 = 0;
    let vulnerability_length: u32 = 90;
    let mut frames: u128 = 0;

    let input_thread = create_input_controller(&game_manager);

    while lives > 0 {        
        if input_thread.is_finished() { break; }

        let mut rucman = game_manager.rucman.lock().unwrap();
        let mut ghosts = game_manager.ghosts.lock().unwrap();
        let mut grid = game_manager.grid.lock().unwrap();

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

        if grid.pellets_left() == 0 { reset_game(&mut grid, &mut rucman, &mut ghosts); }

        print_screen(&grid, &rucman, &ghosts, &score, &lives)?;
        
        //Frees up locks
        drop(rucman);
        drop(grid);
        drop(ghosts);
        sleep(Duration::new(0, 266666672));
        //sleep(Duration::new(0, 1000000000));
    }

    execute!(stdout(), Print(format!("Game over! Score: {}\n", score)))?;
    if !input_thread.is_finished() {
        execute!(stdout(), Print(format!("Press Ctrl+C to end game.\n")))?;
        let _ = input_thread.join();
    }
    disable_raw_mode()?;

    Ok(())
}

fn print_screen(grid: &Grid, rucman: &CharacterData, ghosts: &Vec<CharacterData>, score: &u32, lives: &u8) -> io::Result<()> {        
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
    let mut result_string = String::new();
    let mut i = 0;
    for row in pass_one {
        let mut row_string: String = row.iter().collect();
        if i == 1 { row_string.push_str(format!(" Score: {score}").as_str()); }
        if i == 2 { row_string.push_str(format!(" Lives: {lives}").as_str()); }
        row_string.push('\n');

        result_string.push_str(&row_string);
        i += 1;
    }

    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0), Print(result_string))?;
    Ok(())
}

fn check_collision(rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>, score: &mut u32, lives: &mut u8) {
    for ghost in ghosts.iter_mut() {
        if ghost.get_position() == rucman.get_position() {
            match ghost.get_vulnerability() {
                Vulnerability::Vulnerable => {
                    *score += 200;
                    reset_character(ghost);
                }
                Vulnerability::Invulnerable => {
                    *score -= if *score < 100 { *score } else { 100 };
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

fn create_input_controller(game_manager: &GameManager) -> JoinHandle<()> {
    let rucman = game_manager.rucman.clone();
    let grid = game_manager.grid.clone();

    thread::spawn(move || loop {
        match read() {
            Ok(event) => {
                match event {
                    Event::Key(key) => {
                        if !key.is_release() {
                            match key.code {
                                KeyCode::Char('w') => rucman.lock().unwrap().set_direction_if_valid(Direction::up(), &grid.lock().unwrap()),
                                KeyCode::Char('a') => rucman.lock().unwrap().set_direction_if_valid(Direction::left(), &grid.lock().unwrap()),
                                KeyCode::Char('s') => rucman.lock().unwrap().set_direction_if_valid(Direction::down(), &grid.lock().unwrap()),
                                KeyCode::Char('d') => rucman.lock().unwrap().set_direction_if_valid(Direction::right(), &grid.lock().unwrap()),
                                
                                KeyCode::Char('c') | KeyCode::Char('q') => {
                                    match key.modifiers {
                                        KeyModifiers::CONTROL => break,
                                        _ => {}
                                    }
                                },

                                _ => {},
                            }
                        }
                    },

                    _ => {}
                }
            }
            Err(err) => {
                let _ = execute!(stderr(), Print(format!("{err}")));
                break;
            }
        }
    })
}