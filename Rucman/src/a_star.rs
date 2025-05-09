use crate::direction::Direction;
use crate::grid::grid::Grid;
use crate::point::Vector2;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    position: Vector2,
    cost: i32,
}

impl Ord for State{
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star(grid: &Grid, start: Vector2, end: Vector2) -> Option<Vec<Vector2>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(State{position: start, cost: Vector2::side_distance(start, end)});

    let mut came_from: HashMap<Vector2, Vector2> = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while !open_set.is_empty() {
        let current = open_set.pop();
        if current.is_none() { break; } //Shouldn't happen.
        let current = current.unwrap();

        if current.position == end { return Some(reconstruct_path(came_from, current.position)); }

        for direction in Direction::directions() {
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
            if tentative_g_score < *g_score.get(&next).unwrap_or(&i32::MAX) {
                came_from.insert(next, current.position);
                g_score.insert(next, tentative_g_score);
                let state = State {position: next, cost: tentative_g_score + Vector2::side_distance(next, end)};
                open_set.push(state);
            } 
        }
    }

    None
}

fn reconstruct_path(came_from: HashMap<Vector2, Vector2>, current: Vector2) -> Vec<Vector2> {
    let mut current = current;
    let mut path = vec![current];
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap(); //It shoun't be none since contains_key is checked beforehand.
        path.push(current);
    }
    path
}