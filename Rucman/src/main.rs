use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::{self, Stdout, stdout, stderr};
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

/// Stores mutable pointers to the characters and maze.
struct EntityManager {
    grid: Arc<Mutex<Grid>>,
    rucman: Arc<Mutex<CharacterData>>,
    ghosts: Arc<Mutex<Vec<CharacterData>>>,
}

/// Manages all numerical number.
struct NumberManager {
    level: u32,
    score: u32,
    one_up_score: u32,
    lives: u8,
    scatter_interval: u128,
    vulnerability_length: u32,
    vulernability_timer: u32,
}

impl NumberManager {
    /// Adds provided points to score. Gives a life if one up score is achieved.
    fn add_score(&mut self, score: u32) {
        if score <= 0 { return; }

        self.score += score;
        if self.score >= self.one_up_score {
            self.one_up_score *= 2;
            self.lives += 1;
        }
    }

    /// Removes provided points from score. Prevents overflow.
    fn remove_score(&mut self, score: u32) {
        if self.score < score { 
            self.score = 0;
            return;
        }

        self.score -= score;
    }

    /// Removes a life.
    fn lose_life(&mut self) {
        if self.lives == 0 { return; }
        
        self.lives -= 1;
        self.remove_score(150);
    }

    /// Updates timers to new level.
    fn level_up(&mut self) {
        self.add_score(1000);
        self.shorten_vulnerability();
        self.lengthen_scatter_interval();
    }

    /// Shortens vulnerabilty window by 1 second and floors it at 2 seconds.
    fn shorten_vulnerability(&mut self) {
        self.vulnerability_length -= 4;
        if self.vulnerability_length < 8 { self.vulnerability_length = 8; } // Min at 2 seconds.
    }

    /// Sets vulnerability timer to vulnerability length.
    fn start_vulnerability_timer(&mut self) {
        self.vulernability_timer = self.vulnerability_length;
    }

    /// Lowers vulnerability timer by 1 frame.
    fn tick_vulernability_timer(&mut self) {
        if self.vulernability_timer == 0 {return;}

        self.vulernability_timer -= 1;
    }

    /// Returns true if vulnerability timer is 0.
    fn is_vulnerability_over(&self) -> bool {
        self.vulernability_timer == 0
    }

    /// Doubles the length of scatter interval. Maxes at 30 seconds.
    fn lengthen_scatter_interval(&mut self) {
        self.scatter_interval *= 2;
        if self.scatter_interval > 120 {self.scatter_interval = 120;} // Max out at 30 seconds.
    }
}

fn main() -> io::Result<()> {

    // Initialize data and game environment.
    enable_raw_mode()?;

    let mut stdout = stdout();
    let frame_sleep = Duration::new(0, 250_000_000);
    let three_seconds = Duration::new(3, 0);

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

    let mut number_manager = NumberManager {
        level: 1,
        score: 0,
        one_up_score: 1000,
        lives: 3,
        scatter_interval: 40,
        vulnerability_length: 28,
        vulernability_timer: 0,
    };

    let mut frames: u128 = 0;

    let input_thread = create_input_controller(&entity_manager);

    // Main game loop.
    while number_manager.lives > 0 {        
        if input_thread.is_finished() { break; } // Stop the game if input thread is ever finished.

        // Get the ability to modify rucman and the ghosts.
        let mut rucman = entity_manager.rucman.lock().unwrap();
        let mut ghosts = entity_manager.ghosts.lock().unwrap();
        let mut grid = entity_manager.grid.lock().unwrap();

        // Move rucman (rucman's direction is controlled by the thread.)
        rucman.rucman_move(&grid);

        //Eat pellets
        let eatten = grid.eat(&rucman.get_position());
        match eatten {
            Ok(pellet) => {
                match pellet {
                    GridPoint::Pellet => number_manager.add_score(5),
                    GridPoint::PowerPellet => {
                        for ghost in ghosts.iter_mut() {
                            ghost.set_vulnerable();
                        }
                        number_manager.start_vulnerability_timer();
                        number_manager.add_score(10);
                    },
                    _ => {}, // GridPoint empty and anything that eat doesn't denote as inedible.
                }
            },
            Err(invalid) => {
                match invalid {
                    GridPointError::InconsumableError(attempt) => {
                        match attempt {
                            GridPoint::Teleporter(other) => rucman.set_position(other), // Should be the only inediable object to worry about.
                            _ => {},
                        }
                    }
                    _ => {},
                }
            },
        }

        // Check if rucman ran into a ghost.
        if let Some(ghost) = check_collision(&mut rucman, &mut ghosts, &mut number_manager) {
            print_screen(&mut stdout, &grid, &rucman, &ghosts, &number_manager)?;
            execute!(stdout, Print(format!("Caught by: {:?}", ghost)))?;
            sleep(three_seconds);
            reset_characters(&mut rucman, &mut ghosts);
        }

        //Move Ghosts
        for ghost in ghosts.iter_mut() {
            match ghost.get_vulnerability() {
                Vulnerability::Invulnerable => ghost.ghost_move(&mut grid, rucman.get_position(), rucman.get_direction()),
                Vulnerability::Vulnerable => { // To make vulnerable ghosts slower, they only move on even frames.
                    if frames % 2 == 0 {
                        ghost.ghost_move(&mut grid, rucman.get_position(), rucman.get_direction());
                    }
                    if number_manager.is_vulnerability_over() {
                        ghost.set_invulnerable();
                    }
                }
            }            
        }

        // Copy and pasted.
        // Check if a ghost ran into rucman.
        if let Some(ghost) = check_collision(&mut rucman, &mut ghosts, &mut number_manager) {
            print_screen(&mut stdout, &grid, &rucman, &ghosts, &number_manager)?;
            execute!(stdout, Print(format!("Caught by: {:?}", ghost)))?;
            sleep(three_seconds);
            reset_characters(&mut rucman, &mut ghosts);
        }

        // Update time data.
        number_manager.tick_vulernability_timer();
        if frames == u128::MAX { frames = 0; } //Probably would never happen. Essentially overflow anyway, but this is to define what to happen on overflow.
        else { frames += 1; }

        // Scatter ghosts on time.
        if frames % number_manager.scatter_interval == 0 {
            for ghost in ghosts.iter_mut() {
                ghost.set_scatter_mode();
            }
        }

        // Level completion.
        if grid.pellets_left() == 0 { 
            print_screen(&mut stdout, &grid, &rucman, &ghosts, &number_manager)?;
            execute!(stdout, Print("Level complete!"))?;
            sleep(three_seconds);
            reset_game(&mut grid, &mut rucman, &mut ghosts); 

            // Level up
            number_manager.level_up();
        }

        print_screen(&mut stdout, &grid, &rucman, &ghosts, &number_manager)?;
        
        // Frees up locks
        drop(rucman);
        drop(grid);
        drop(ghosts);

        // So the game doesn't do every frame in a single frame.
        sleep(frame_sleep);
    }

    execute!(stdout, Print(format!("Game over! Score: {}\n", number_manager.score)))?;

    // Make sure we don't get an orphan thread.
    if !input_thread.is_finished() {
        execute!(stdout, Print(format!("Press Ctrl+C to end game.\n")))?;
        let _ = input_thread.join();
    }

    disable_raw_mode()?;

    Ok(())
}

/// Prints the screen
fn print_screen(stdout: &mut Stdout, grid: &Grid, rucman: &CharacterData, ghosts: &Vec<CharacterData>, score_manager: &NumberManager) -> io::Result<()> {        
    let level = score_manager.level;
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

    // Place the ghosts and rucman over the maze (the subjects)
    let pos = rucman.get_position();
    pass_one[pos.1 as usize][pos.0 as usize] = char::from(rucman);
    
    // Ghosts second so they overlap Rucman so its obvious when rucman is dead.
    for ghost in ghosts {
        let pos = ghost.get_position();
        pass_one[pos.1 as usize][pos.0 as usize] = char::from(ghost);
    }
    
    // Convert collected data into strings and print it.
    let mut result_string = String::new();
    let mut i = 0;
    for row in pass_one {
        let mut row_string: String = row.iter().collect();
        match i {
            1 => row_string.push_str(format!(" Level: {level}").as_str()),
            2 => row_string.push_str(format!(" Score: {score}").as_str()),
            3 => row_string.push_str(format!(" Lives: {lives}").as_str()),
            4 => row_string.push_str(format!(" One up at: {one_up_score}").as_str()),
            _ => {}
        }
        
        row_string.push('\n');

        result_string.push_str(&row_string);
        i += 1;
    }

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(result_string))?;
    Ok(())
}

/// Checks for collisions between rucman and the ghosts and handles the cases for vulnerable and invulnerable ghosts.
/// Returns a character if rucman collided with an invulnerable ghost.
fn check_collision(rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>, score_manager: &mut NumberManager) -> Option<Character> {
    for ghost in ghosts.iter_mut() {
        if ghost.get_position() == rucman.get_position() {
            match ghost.get_vulnerability() {
                Vulnerability::Vulnerable => {
                    score_manager.add_score(200);
                    reset_character(ghost);
                }
                Vulnerability::Invulnerable => {
                    score_manager.lose_life();
                    return Some(ghost.get_character());
                }
            }
        }
    }

    None
}

/// Resets the maze and characters to their initial state
fn reset_game(grid: &mut Grid, rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>) {
    *grid = Grid::new();
    reset_characters(rucman, ghosts);
}

/// Resets all characters to their initial state
fn reset_characters(rucman: &mut CharacterData, ghosts: &mut Vec<CharacterData>) {
    reset_character(rucman);
    for ghost in ghosts {
        reset_character(ghost);
    }
}

/// Resets a single character to their inital state
fn reset_character(character: &mut CharacterData) {
    *character = CharacterData::new(character.get_character());
}

/// Creates a thread that handles user input.
/// Directional key presses directly change the direction of Rucman.
/// Pressing Ctrl+C or Ctrl+Q closes the thread. The game should end if this thread ever closes.
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
                                // Directional inputs.
                                KeyCode::Char('w') => rucman.lock().unwrap().set_direction_if_valid(Direction::up(), &grid.lock().unwrap()),
                                KeyCode::Char('a') => rucman.lock().unwrap().set_direction_if_valid(Direction::left(), &grid.lock().unwrap()),
                                KeyCode::Char('s') => rucman.lock().unwrap().set_direction_if_valid(Direction::down(), &grid.lock().unwrap()),
                                KeyCode::Char('d') => rucman.lock().unwrap().set_direction_if_valid(Direction::right(), &grid.lock().unwrap()),
                                
                                // Control inputs.
                                KeyCode::Char('c') | KeyCode::Char('q') => { // Quit
                                    match key.modifiers {
                                        KeyModifiers::CONTROL => break,
                                        _ => {}
                                    }
                                },

                                _ => {}, // Ignore all other keys.
                            }
                        }
                    },

                    _ => {} // Ignore all other events
                }
            }
            Err(err) => {
                let _ = execute!(stderr(), Print(format!("{err}")));
                break;
            }
        }
    })
}