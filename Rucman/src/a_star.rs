use crate::direction::Direction;
use crate::grid::grid::Grid;
use crate::point::Vector2;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Used to order binary heap.
#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    position: Vector2,
    f_score: i32,
}

impl Ord for State{
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score) //Min heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds the shortest path from the start point to the end point.
pub fn a_star(grid: &Grid, start: Vector2, end: Vector2, already_on_start: bool) -> Option<Vec<Vector2>> {
    let mut open_set = BinaryHeap::new(); // Sorts frontier by the minimum f-scores.
    open_set.push(State{position: start, f_score: Vector2::side_distance(start, end)});

    let mut came_from: HashMap<Vector2, Vector2> = HashMap::new(); // For node n, came_from[n] is the currently known node preceding n in the shortest path.
    let mut g_score = HashMap::new(); // Shortest known distances for all nodes.
    g_score.insert(start, 0);

    while !open_set.is_empty() {
        let current = open_set.pop(); // Get element with lowest f-score.
        if current.is_none() { break; } //Shouldn't happen.
        let current = current.unwrap();

        if current.position == end { return Some(reconstruct_path(came_from, current.position, already_on_start)); } // If current is the end, return the path. 

        for direction in Direction::directions() { // Calculate f-scores for all valid neighbors.
            let direction = {
                match direction {
                    Direction::Up(dir) |
                    Direction::Down(dir) |
                    Direction::Left(dir) |
                    Direction::Right(dir) => dir
                }
            };

            let next = current.position + direction;
            if !grid.is_valid_pos(&next) { continue; }

            let tentative_g_score = g_score.get(&current.position).unwrap_or(&i32::MAX) + 1;
            //If this path to this neighbor is shorter than previously recorded, put it back into the open set.
            if tentative_g_score < *g_score.get(&next).unwrap_or(&i32::MAX) {
                came_from.insert(next, current.position);
                g_score.insert(next, tentative_g_score);
                let state = State {position: next, f_score: tentative_g_score + Vector2::side_distance(next, end)};
                //No need to check to see if the neighbor is already in the open set. Since it's a binary heap, it is guaranteed that
                //instances of the neighbor with lower f_scores will be accessed before instances with heigher f_scores.
                //If an instance with a higher f_score is accessed after an instance with a lower f_score, it will fail the 
                //g_score if statement.
                open_set.push(state); 
            } 
        }
    }

    None //No path was found.
}

/// Reconstructs the path found from the a-star algorithm. Path is in reverse order, so use pop to navigate it.
fn reconstruct_path(came_from: HashMap<Vector2, Vector2>, current: Vector2, already_on_start: bool) -> Vec<Vector2> {
    let mut current = current;
    let mut path = vec![current];
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap(); //It shouldn't be none since contains_key is checked beforehand.
        path.push(current);
    }

    if already_on_start { path.pop(); } // Remove the start from the path if we're already there.

    path
}