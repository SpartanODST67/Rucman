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

struct EntityManager {
    grid: Arc<Mutex<Grid>>,
    rucman: Arc<Mutex<CharacterData>>,
    ghosts: Arc<Mutex<Vec<CharacterData>>>,
}

struct ScoreManager {
    score: u32,
    one_up_score: u32,
    lives: u8,
    scatter_interval: u128,
}

impl ScoreManager {
    fn add_score(&mut self, score: u32) {
        if score <= 0 { return; }

        self.score += score;
        if self.score >= self.one_up_score {
            self.one_up_score *= 2;
            self.lives += 1;
        }
    }

    fn remove_score(&mut self, score: u32) {
        if self.score < score { 
            self.score = 0;
            return;
        }

        self.score -= score;
    }

    fn lose_life(&mut self) {
        self.lives -= 1;
        self.remove_score(150);
    }
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

    let entity_manager = EntityManager {
        grid: Arc::new(Mutex::new(grid)),
        rucman: Arc::new(Mutex::new(rucman)),
        ghosts: Arc::new(Mutex::new(ghosts)), 
    };

    let mut score_manager = ScoreManager {
        score: 0,
        one_up_score: 1000,
        lives: 3,
        scatter_interval: 75,
    };

    let mut vulnerability_timer: u32 = 0;
    let mut vulnerability_length: u32 = 90;
    let mut frames: u128 = 0;

    let input_thread = create_input_controller(&entity_manager);

    while score_manager.lives > 0 {        
        if input_thread.is_finished() { break; }

        let mut rucman = entity_manager.rucman.lock().unwrap();
        let mut ghosts = entity_manager.ghosts.lock().unwrap();
        let mut grid = entity_manager.grid.lock().unwrap();

        rucman.rucman_move(&grid);

        //Eat pellets
        let eatten = grid.eat(&rucman.get_position());
        match eatten {
            Ok(pellet) => {
                match pellet {
                    GridPoint::Pellet => score_manager.add_score(5),
                    GridPoint::PowerPellet => {
                        for ghost in ghosts.iter_mut() {
                            ghost.set_vulnerable();
                        }
                        vulnerability_timer = vulnerability_length;
                        score_manager.add_score(10);
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

        check_collision(&mut rucman, &mut ghosts, &mut score_manager);

        //Move Ghosts
        for ghost in ghosts.iter_mut() {
            match ghost.get_vulnerability() {
                Vulnerability::Invulnerable => ghost.ghost_move(&mut grid, rucman.get_position(), rucman.get_direction()),
                Vulnerability::Vulnerable => {
                    if frames % 2 == 0 {
                        ghost.ghost_move(&mut grid, rucman.get_position(), rucman.get_direction());
                    }
                    if vulnerability_timer == 0 {
                        ghost.set_invulnerable();
                    }
                }
            }            
        }

        check_collision(&mut rucman, &mut ghosts, &mut score_manager);

        if vulnerability_timer > 0 { vulnerability_timer -= 1; }
        if frames == u128::MAX { frames = 0; } //Probably would never happen. Essentially overflow anyway, but this is to define what to happen on overflow.
        else { frames += 1; }

        if frames % score_manager.scatter_interval == 0 {
            for ghost in ghosts.iter_mut() {
                ghost.set_scatter_mode();
            }
        }

        if grid.pellets_left() == 0 { 
            reset_game(&mut grid, &mut rucman, &mut ghosts); 
            score_manager.add_score(1000);
            vulnerability_length -= 1;
            score_manager.scatter_interval *= 2;
        }

        print_screen(&grid, &rucman, &ghosts, &score_manager)?;
        
        //Frees up locks
        drop(rucman);
        drop(grid);
        drop(ghosts);

        sleep(Duration::new(0, 266666672));
    }

    execute!(stdout(), Print(format!("Game over! Score: {}\n", score_manager.score)))?;
    if !input_thread.is_finished() {
        execute!(stdout(), Print(format!("Press Ctrl+C to end game.\n")))?;
        let _ = input_thread.join();
    }
    disable_raw_mode()?;

    Ok(())
}

fn print_screen(grid: &Grid, rucman: &CharacterData, ghosts: &Vec<CharacterData>, score_manager: &ScoreManager) -> io::Result<()> {        
    let score = score_manager.score;
    let lives = score_manager.lives;
    let one_up_score = score_manager.one_up_score;

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
        if i == 3 { row_string.push_str(format!(" One up at: {one_up_score}").as_str());}
        row_string.push('\n');

        result_string.push_str(&row_string);
        i += 1;
    }

    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0), Print(result_string))?;
    Ok(())
}

fn check_collision(rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>, score_manager: &mut ScoreManager) {
    for ghost in ghosts.iter_mut() {
        if ghost.get_position() == rucman.get_position() {
            match ghost.get_vulnerability() {
                Vulnerability::Vulnerable => {
                    score_manager.add_score(200);
                    reset_character(ghost);
                }
                Vulnerability::Invulnerable => {
                    score_manager.lose_life();
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

fn create_input_controller(game_manager: &EntityManager) -> JoinHandle<()> {
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