/// Represents an x and y cordinate pair. As y increases, it goes down the y axis.
struct Point(u32, u32);

/// Represents an entity that is a part of the grid.
#[derive(Debug, Clone, Copy, PartialEq)]
enum GridPoint {
    Pellet,
    PowerPellet,
    Wall,
    Empty,
}

/// Represents errors when accessing the grid.
#[derive(Debug, PartialEq)]
enum GridPointError {
    InconsumableError(GridPoint),
    BadPosError,
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

/// Stores the grid and its meta data.
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<GridPoint>>,
    width: usize,
    height: usize,
    pellets_left: u32,
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
            pellets_left: 5
        }
    }

    /// Check to see if provided point is a valid position for an entity to be on.
    fn is_valid_pos(&self, pos: &Point) -> bool {
        let col: usize = pos.0.try_into().unwrap();
        let row: usize = pos.1.try_into().unwrap();
        
        if col >= self.width || row >= self.height { return false; }
        match self.grid[row][col] {
            GridPoint::Wall => false,
            _ => true,
        }
    }

    /// Retrieves the GridPoint stored at the provided point and replaces it with empty.
    fn eat(&mut self, pos: &Point) -> Result<GridPoint, GridPointError> {
        let col: usize = pos.0.try_into().unwrap();
        let row: usize = pos.1.try_into().unwrap();

        if col >= self.width || row >= self.height { return Err(GridPointError::BadPosError); }
        let res = {
            match self.grid[row][col] {
                GridPoint::Pellet | GridPoint::PowerPellet  => {
                        self.pellets_left -= 1;
                        Ok(self.grid[row][col]) 
                    },
                GridPoint::Empty => Ok(GridPoint::Empty),
                GridPoint::Wall => {Err(GridPointError::InconsumableError(self.grid[row][col]))},
            }
        };

        self.grid[row][col] = GridPoint::Empty;
        
        res 
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

    /// Tests if the grid can accurately return true on valid positions.
    #[test]
    fn valid_pos() {
        let grid = Grid::new();
        assert!(grid.is_valid_pos(&Point((grid.width - 2) as u32, (grid.height - 2) as u32)));
        assert!(grid.is_valid_pos(&Point(1, 1)));
    }

    /// Tests if the grid can accurately return false on invalid positions.
    #[test]
    fn invalid_pos() {
        let grid = Grid::new();
        assert!(!grid.is_valid_pos(&Point(0, 0)));
        assert!(!grid.is_valid_pos(&Point((grid.width) as u32, (grid.height) as u32)));
        assert!(!grid.is_valid_pos(&Point((grid.width - 1) as u32, (grid.height - 1) as u32)));
    }

    /// Tests if the grid can accurately return a Grid point on valid positions.
    /// This test is reliant on the current 5x5 test grid and will need to be updated when the grid is updated.
    #[test]
    fn valid_eat() {
        let mut grid = Grid::new();
        assert_eq!(grid.eat(&Point(1, 1)), Ok(GridPoint::Pellet));
        assert_eq!(grid.pellets_left, 4);
        assert_eq!(grid.eat(&Point(2, 1)), Ok(GridPoint::PowerPellet));
        assert_eq!(grid.pellets_left, 3);
        assert_eq!(grid.eat(&Point(1, 3)), Ok(GridPoint::Empty));
    }

    /// Tests if the grid replaces previously eatten points with Empty.
    /// This tests are reliant on the current 5x5 test grid and will need to be updated when the grid is updated.
    #[test]
    fn check_eat_empty() {
        let mut grid = Grid::new();
        let _ = grid.eat(&Point(1, 1));
        assert_eq!(grid.eat(&Point(1, 1)), Ok(GridPoint::Empty));
        assert_eq!(grid.pellets_left, 4);
        let _ = grid.eat(&Point(2, 1));
        assert_eq!(grid.eat(&Point(2, 1)), Ok(GridPoint::Empty));
        assert_eq!(grid.pellets_left, 3);
        let _ = grid.eat(&Point(1, 3));
        assert_eq!(grid.eat(&Point(1, 3)), Ok(GridPoint::Empty));
    }

    /// Tests if the grid can accurately return an error on invalid eat positions.
    #[test]
    fn invalid_eat() {
        let mut grid = Grid::new();
        assert_eq!(grid.eat(&Point(0, 0)), Err(GridPointError::InconsumableError(GridPoint::Wall)));
        assert_eq!(grid.eat(&Point((grid.width) as u32, (grid.height) as u32)), Err(GridPointError::BadPosError));
        assert_eq!(grid.eat(&Point((grid.width - 1) as u32, (grid.height - 1) as u32)), Err(GridPointError::InconsumableError(GridPoint::Wall)));
    }
}
