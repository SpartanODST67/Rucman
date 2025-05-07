struct Point(u32, u32);

#[derive(Debug, Clone, Copy)]
enum GridPoint {
    Pellet,
    PowerPellet,
    Wall,
    Empty
}

impl From<GridPoint> for String {
    fn from(input: GridPoint) -> String {
        match input {
            GridPoint::Pellet => String::from("."),
            GridPoint::PowerPellet => String::from("*"),
            GridPoint::Wall => String::from("|"),
            GridPoint::Empty => String::from(" "),
        }
    }
}

impl From<GridPoint> for char {
    fn from(input: GridPoint) -> char {
        match input {
            GridPoint::Pellet => '.',
            GridPoint::PowerPellet => '*',
            GridPoint::Wall => '|',
            GridPoint::Empty => ' ',
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<GridPoint>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new() -> Self {
        let grid = vec![
            vec![GridPoint::Wall, GridPoint::Wall, GridPoint::Wall, GridPoint::Wall, GridPoint::Wall],
            vec![GridPoint::Wall, GridPoint::Pellet, GridPoint::PowerPellet, GridPoint::Pellet, GridPoint::Wall],
            vec![GridPoint::Wall, GridPoint::Pellet, GridPoint::Empty, GridPoint::Pellet, GridPoint::Wall],
            vec![GridPoint::Wall, GridPoint::Empty, GridPoint::Empty, GridPoint::Empty, GridPoint::Wall],
            vec![GridPoint::Wall, GridPoint::Wall, GridPoint::Wall, GridPoint::Wall, GridPoint::Wall],
        ];
        
        Grid {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    fn is_valid_pos(&self, pos: &Point) -> bool {
        let col: usize = pos.0.try_into().unwrap();
        let row: usize = pos.1.try_into().unwrap();
        
        if col >= self.width || row >= self.height { return false; }
        match self.grid[row][col] {
            GridPoint::Wall => false,
            _ => true,
        }
    }
}

fn main() {
    let grid = Grid::new();
    println!("{grid:?}");
    print_screen(&grid);
}

fn print_screen(grid: &Grid) {
    for row in &grid.grid {
        let mut row_string = String::from("");
        for col in row {
            row_string.push(char::from(*col));
        }
        print!("{row_string}\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_pos() {
        let grid = Grid::new();
        assert!(grid.is_valid_pos(&Point((grid.width - 2) as u32, (grid.height - 2) as u32)));
        assert!(grid.is_valid_pos(&Point(1, 1)));
    }

    #[test]
    fn invalid_pos() {
        let grid = Grid::new();
        assert!(!grid.is_valid_pos(&Point(0, 0)));
        assert!(!grid.is_valid_pos(&Point((grid.width) as u32, (grid.height) as u32)));
        assert!(!grid.is_valid_pos(&Point((grid.width - 1) as u32, (grid.height - 1) as u32)));
    }
}
