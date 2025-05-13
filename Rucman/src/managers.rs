use std::sync::{Arc, Mutex};
use crate::{Grid, CharacterData};

/// Stores mutable pointers to the characters and maze.
pub struct EntityManager {
    pub grid: Arc<Mutex<Grid>>,
    pub rucman: Arc<Mutex<CharacterData>>,
    pub ghosts: Arc<Mutex<Vec<CharacterData>>>,
}

/// Manages all numerical number.
pub struct NumberManager {
    level: u32,
    score: u32,
    one_up_score: u32,
    lives: u8,
    scatter_interval: u128,
    vulnerability_length: u32,
    vulernability_timer: u32,
}

impl NumberManager {
    /// Creates a new number manager
    pub fn new() -> Self {
        NumberManager {
            level: 1,
            score: 0,
            one_up_score: 1000,
            lives: 3,
            scatter_interval: 40,
            vulnerability_length: 28,
            vulernability_timer: 0,
        }
    }

    /// Retrieve number of lives.
    pub fn get_lives(&self) -> u8 {
        self.lives
    }

    /// Retrieve current level number.
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Retrive current score.
    pub fn get_score(&self) -> u32 {
        self.score
    }

    /// Retrive one up score.
    pub fn get_one_up_score(&self) -> u32 {
        self.one_up_score
    }

    pub fn get_scatter_interval(&self) -> u128 {
        self.scatter_interval
    }
    
    /// Adds provided points to score. Gives a life if one up score is achieved.
    pub fn add_score(&mut self, score: u32) {
        if score <= 0 { return; }

        self.score += score;
        if self.score >= self.one_up_score {
            self.one_up_score *= 2;
            self.lives += 1;
        }
    }

    /// Removes provided points from score. Prevents overflow.
    pub fn remove_score(&mut self, score: u32) {
        if self.score < score { 
            self.score = 0;
            return;
        }

        self.score -= score;
    }

    /// Removes a life.
    pub fn lose_life(&mut self) {
        if self.lives == 0 { return; }
        
        self.lives -= 1;
        self.remove_score(150);
    }

    /// Updates timers to new level.
    pub fn level_up(&mut self) {
        self.add_score(1000);
        self.shorten_vulnerability();
        self.lengthen_scatter_interval();
    }

    /// Shortens vulnerabilty window by 1 second and floors it at 2 seconds.
    pub fn shorten_vulnerability(&mut self) {
        self.vulnerability_length -= 4;
        if self.vulnerability_length < 8 { self.vulnerability_length = 8; } // Min at 2 seconds.
    }

    /// Sets vulnerability timer to vulnerability length.
    pub fn start_vulnerability_timer(&mut self) {
        self.vulernability_timer = self.vulnerability_length;
    }

    /// Lowers vulnerability timer by 1 frame.
    pub fn tick_vulernability_timer(&mut self) {
        if self.vulernability_timer == 0 {return;}

        self.vulernability_timer -= 1;
    }

    /// Returns true if vulnerability timer is 0.
    pub fn is_vulnerability_over(&self) -> bool {
        self.vulernability_timer == 0
    }

    /// Doubles the length of scatter interval. Maxes at 30 seconds.
    pub fn lengthen_scatter_interval(&mut self) {
        self.scatter_interval *= 2;
        if self.scatter_interval > 120 {self.scatter_interval = 120;} // Max out at 30 seconds.
    }
}