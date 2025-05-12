use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;

use std::time::Duration;
use std::io::{self, stdout, stderr, Error};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::thread::sleep;

mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;

mod direction;
use direction::Direction;

mod character;
use character::{Character, CharacterData, Vulnerability};

mod a_star;

enum InputError {
    QuitInput,
    StandardInputError(Error)
}

fn main() -> io::Result<()> {

    enable_raw_mode()?;

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

    let input_controller = create_input_controller();

    while lives > 0 {        
        //Move Rucman
        let player_input = input_controller.try_recv();
        
        while let Ok(_) = input_controller.try_recv() {}; //Empty the channel.

        match player_input {
            Ok(input_result) => {
                match input_result {
                    Ok(direction) => {
                        match direction {
                            Some(dir) => {
                                let old = rucman.get_direction();
                                rucman.set_direction(dir);
                                if !grid.is_valid_pos(&rucman.calculate_facing_position()) {
                                    rucman.set_direction(old);
                                }
                            }
                            None => {},
                        }
                    },
                    Err(err) => {
                        match err {
                            InputError::StandardInputError(err) => {
                                let _ = execute!(stderr(), Print(err.to_string()));
                                break;
                            },
                            InputError::QuitInput => break, //This is an expected break, so no need to log an error.
                        }
                    }
                }
            },
            Err(err) => {
                match err {
                    mpsc::TryRecvError::Disconnected => {
                        let _ = execute!(stderr(), Print(err.to_string()));
                        break;
                    }, //Stop the game if the input is ever lost.
                    mpsc::TryRecvError::Empty => {},
                }
            }
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

        if grid.pellets_left() == 0 { reset_game(&mut grid, &mut rucman, &mut ghosts); }

        print_screen(&grid, &rucman, &ghosts, &score, &lives)?;
        sleep(Duration::new(0, 16_666_667 * 16));
    }
    execute!(stdout(), Print(format!("Game over! Score: {}", score)))?;
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

fn take_input() -> Result<Option<Direction>, InputError> {
    match read() {
        Ok(event) => {
            match event {
                Event::Key(KeyEvent {               //Directional inputs
                    code: KeyCode::Char('w'),
                    kind: KeyEventKind::Press,
                    ..
                }) => Ok(Some(Direction::up())),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    kind: KeyEventKind::Press,
                    ..
                }) => Ok(Some(Direction::left())),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    kind: KeyEventKind::Press,
                    ..
                }) => Ok(Some(Direction::down())),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    ..
                }) => Ok(Some(Direction::right())),
                Event::Key(KeyEvent {                   //Quit inputs
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) | 
                Event::Key(KeyEvent { 
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                })=> Err(InputError::QuitInput),
                _ => Ok(None), //Ignore all other events.
            }
        }
        Err(err) => { Err(InputError::StandardInputError(err)) }, //Erros that are out of my control.
    }
}

fn create_input_controller() -> Receiver<Result<Option<Direction>, InputError>> {
    let (tx, rx) = mpsc::channel::<Result<Option<Direction>, InputError>>();

    thread::spawn(move || loop {
        match take_input() {
            Ok(direction) => {
                match direction {
                    Some(dir) => { 
                        let _ = tx.send(Ok(Some(dir))); 
                    },
                    None => {}
                }
            },
            Err(err) => {
                let _ = tx.send(Err(err));
                break;
            }
        }        
    });

    rx
}