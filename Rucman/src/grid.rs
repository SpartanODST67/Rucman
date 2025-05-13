pub mod grid {
    use rand::prelude::*;
    use crate::point::Vector2;

    /// Represents an entity that is a part of the grid.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum GridPoint {
        Pellet,
        PowerPellet,
        Wall,
        Empty,
        Teleporter(Vector2),
    }

    /// Represents errors when accessing the grid.
    #[derive(Debug, PartialEq)]
    pub enum GridPointError {
        InconsumableError(GridPoint),
        BadPosError,
    }

    impl From<GridPoint> for char {
        fn from(input: GridPoint) -> char {
            match input {
                GridPoint::Pellet => '.',
                GridPoint::PowerPellet => '*',
                GridPoint::Wall => '█',
                _ => ' ',
            }
        }
    }

    impl From<char> for GridPoint {
        fn from(input: char) -> GridPoint {
            match input {
                '.' => GridPoint::Pellet,
                '*' => GridPoint::PowerPellet,
                '█' => GridPoint::Wall,
                ' ' => GridPoint::Empty,
                _ => panic!("Cannot convert {} to a grid point", input),
            }
        }
    }

    /// Stores the grid and its meta data.
    #[derive(Debug)]
    pub struct Grid {
        maze: Vec<Vec<GridPoint>>,
        open_spaces: Vec<Vector2>,
        width: usize,
        height: usize,
        pellets_left: u32,
    }

    impl Grid {
        /// Creates a new rucman grid.
        pub fn new() -> Self {
            // Doing it this way made it easier to visualize the maze.
            let maze = vec![
                vec!['█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█'],
                vec!['█','.','.','.','.','.','.','.','.','.','.','.','.','█','.','.','.','.','.','.','.','.','.','.','.','.','█'],
                vec!['█','.','█','█','█','█','.','█','█','█','█','█','.','█','.','█','█','█','█','█','.','█','█','█','█','.','█'],
                vec!['█','*','█','█','█','█','.','█','█','█','█','█','.','█','.','█','█','█','█','█','.','█','█','█','█','*','█'],
                vec!['█','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','█'],
                vec!['█','.','█','█','█','█','.','█','█','.','█','█','█','█','█','█','█','.','█','█','.','█','█','█','█','.','█'],
                vec!['█','.','█','█','█','█','.','█','█','.','█','█','█','█','█','█','█','.','█','█','.','█','█','█','█','.','█'],
                vec!['█','.','.','.','.','.','.','█','█','.','.','.','.','█','.','.','.','.','█','█','.','.','.','.','.','.','█'],
                vec!['█','█','█','█','█','█','.','█','█','█','█','█','.','█','.','█','█','█','█','█','.','█','█','█','█','█','█'],
                vec!['█','█','█','█','█','█','.','█','█',' ',' ',' ',' ',' ',' ',' ',' ',' ','█','█','.','█','█','█','█','█','█'],
                vec!['█','█','█','█','█','█','.','█','█',' ','█','█','█',' ','█','█','█',' ','█','█','.','█','█','█','█','█','█'],
                vec!['█','█','█','█','█','█','.','█','█',' ','█',' ',' ',' ',' ',' ','█',' ','█','█','.','█','█','█','█','█','█'],
                vec![' ',' ',' ',' ',' ',' ','.',' ',' ',' ','█',' ',' ',' ',' ',' ','█',' ',' ',' ','.',' ',' ',' ',' ',' ',' '],
                vec!['█','█','█','█','█','█','.','█','█',' ','█','█','█','█','█','█','█',' ','█','█','.','█','█','█','█','█','█'],
                vec!['█','█','█','█','█','█','.','█','█',' ',' ',' ',' ',' ',' ',' ',' ',' ','█','█','.','█','█','█','█','█','█'],
                vec!['█','█','█','█','█','█','.','█','█',' ','█','█','█','█','█','█','█',' ','█','█','.','█','█','█','█','█','█'],
                vec!['█','█','█','█','█','█','.','█','█',' ','█','█','█','█','█','█','█',' ','█','█','.','█','█','█','█','█','█'],
                vec!['█','.','.','.','.','.','.','.','.','.','.','.','.','█','.','.','.','.','.','.','.','.','.','.','.','.','█'],
                vec!['█','.','█','█','█','█','.','█','█','█','█','█','.','█','.','█','█','█','█','█','.','█','█','█','█','.','█'],
                vec!['█','.','█','█','█','█','.','█','█','█','█','█','.','█','.','█','█','█','█','█','.','█','█','█','█','.','█'],
                vec!['█','*','.','.','█','█','.','.','.','.','.','.','.',' ','.','.','.','.','.','.','.','█','█','.','.','*','█'],
                vec!['█','█','█','.','█','█','.','█','█','.','█','█','█','█','█','█','█','.','█','█','.','█','█','.','█','█','█'],
                vec!['█','█','█','.','█','█','.','█','█','.','█','█','█','█','█','█','█','.','█','█','.','█','█','.','█','█','█'],
                vec!['█','.','.','.','.','.','.','█','█','.','.','.','.','█','.','.','.','.','█','█','.','.','.','.','.','.','█'],
                vec!['█','.','█','█','█','█','█','█','█','█','█','█','.','█','.','█','█','█','█','█','█','█','█','█','█','.','█'],
                vec!['█','.','█','█','█','█','█','█','█','█','█','█','.','█','.','█','█','█','█','█','█','█','█','█','█','.','█'],
                vec!['█','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','█'],
                vec!['█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█','█'],
            ];

            let mut res = Self::from(maze);
            res.maze[12][0] = GridPoint::Teleporter(Vector2(26, 12));
            res.maze[12][26] = GridPoint::Teleporter(Vector2(0, 12));

            res
        }

        /// Borrow the maze from grid.
        pub fn get_maze(&self) -> &Vec<Vec<GridPoint>> {
            &self.maze
        }

        /// Retrieves the full width of the maze.
        pub fn get_width(&self) -> usize {
            self.width
        }

        /// Retrieves the full height of the maze.
        pub fn get_height(&self) -> usize {
            self.height
        }

        /// Retrieves a random valid position of the maze.
        pub fn get_random_position(&mut self) -> Vector2 {
            let dest = self.open_spaces.pop().unwrap();
            self.open_spaces.push(dest);
            dest
        }

        /// Check to see if provided point is a valid position for an entity to be on.
        pub fn is_valid_pos(&self, pos: &Vector2) -> bool {
            let col: i32 = pos.0;
            let row: i32 = pos.1;
            
            if col < 0 || row < 0 { return false; }

            let col: usize = col.try_into().unwrap();
            let row: usize = row.try_into().unwrap();

            if col >= self.width || row >= self.height { return false; }

            match self.maze[row][col] {
                GridPoint::Wall => false,
                _ => true,
            }
        }

        /// Retrieves the GridPoint stored at the provided point and replaces it with empty.
        pub fn eat(&mut self, pos: &Vector2) -> Result<GridPoint, GridPointError> {            
            let col: i32 = pos.0;
            let row: i32 = pos.1;
            
            if col < 0 || row < 0 { return Err(GridPointError::BadPosError); }

            let col: usize = col.try_into().unwrap();
            let row: usize = row.try_into().unwrap();

            if col >= self.width || row >= self.height { return Err(GridPointError::BadPosError); }

            let res = {
                match self.maze[row][col] {
                    GridPoint::Pellet | GridPoint::PowerPellet  => {
                            self.pellets_left -= 1;
                            Ok(self.maze[row][col]) 
                        },
                    GridPoint::Empty => Ok(GridPoint::Empty),
                    _ => {return Err(GridPointError::InconsumableError(self.maze[row][col]))},
                }
            };

            self.maze[row][col] = GridPoint::Empty;
            
            res 
        }

        /// Retrieves the number of pellets left in the maze.
        pub fn pellets_left(&self) -> u32 {
            self.pellets_left
        }
    }

    impl From<Vec<Vec<char>>> for Grid {
        fn from(value: Vec<Vec<char>>) -> Self {
            let mut grid = Vec::new();
            let mut open_spaces = Vec::new();
            let mut pellets_left = 0;

            let mut row_num = 0;
            for row in value {
                let mut row_collection = Vec::new();
                let mut col_num = 0;
                for col in row {
                    let grid_point: GridPoint = col.into();
                    match grid_point {
                        GridPoint::Pellet | GridPoint::PowerPellet => {
                            pellets_left += 1;
                            open_spaces.push(Vector2(col_num, row_num));
                        },
                        GridPoint::Empty => open_spaces.push(Vector2(col_num, row_num)),
                        _ => {},
                    }
                    col_num += 1;
                    row_collection.push(grid_point);
                }
                row_num += 1;
                grid.push(row_collection);
            }

            open_spaces.shuffle(&mut rand::rng());

            Grid {
                width: grid[0].len(),
                height: grid.len(),
                maze: grid,
                open_spaces,
                pellets_left
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Tests if the grid can accurately return true on valid positions.
        #[test]
        fn valid_pos() {
            let grid = Grid::new();
            assert!(grid.is_valid_pos(&Vector2((grid.width - 2) as i32, (grid.height - 2) as i32)));
            assert!(grid.is_valid_pos(&Vector2(1, 1)));
        }

        /// Tests if every random pos is a valid pos.
        #[test]
        fn random_pos() {
            let mut grid = Grid::new();
            let n = grid.open_spaces.len();
            for _ in 0..n {
                let pos = grid.get_random_position();
                assert!(grid.is_valid_pos(&pos));
            }
        }

        /// Tests if the grid can accurately return false on invalid positions.
        #[test]
        fn invalid_pos() {
            let grid = Grid::new();
            assert!(!grid.is_valid_pos(&Vector2(0, 0)));
            assert!(!grid.is_valid_pos(&Vector2((grid.width) as i32, (grid.height) as i32)));
            assert!(!grid.is_valid_pos(&Vector2((grid.width - 1) as i32, (grid.height - 1) as i32)));
            assert!(!grid.is_valid_pos(&Vector2(-1, -1)));
        }

        /// Tests if the grid can accurately return a Grid point on valid positions.
        #[test]
        fn valid_eat() {
            let mut grid = Grid::new();
            assert_eq!(grid.eat(&Vector2(1, 1)), Ok(GridPoint::Pellet));
            assert_eq!(grid.pellets_left, 233);
            assert_eq!(grid.eat(&Vector2(1, 3)), Ok(GridPoint::PowerPellet));
            assert_eq!(grid.pellets_left, 232);
            assert_eq!(grid.eat(&Vector2(9, 9)), Ok(GridPoint::Empty));
        }

        /// Tests if the grid replaces previously eatten points with Empty.
        #[test]
        fn check_eat_empty() {
            let mut grid = Grid::new();
            let _ = grid.eat(&Vector2(1, 1));
            assert_eq!(grid.eat(&Vector2(1, 1)), Ok(GridPoint::Empty));
            assert_eq!(grid.pellets_left, 233);
            let _ = grid.eat(&Vector2(1, 3));
            assert_eq!(grid.eat(&Vector2(1, 3)), Ok(GridPoint::Empty));
            assert_eq!(grid.pellets_left, 232);
            let _ = grid.eat(&Vector2(9, 9));
            assert_eq!(grid.eat(&Vector2(9, 9)), Ok(GridPoint::Empty));
            assert_eq!(grid.pellets_left, 232);
        }

        /// Tests if the grid can accurately return an error on invalid eat positions.
        #[test]
        fn invalid_eat() {
            let mut grid = Grid::new();
            assert_eq!(grid.eat(&Vector2(0, 0)), Err(GridPointError::InconsumableError(GridPoint::Wall)));
            assert_eq!(grid.eat(&Vector2((grid.width) as i32, (grid.height) as i32)), Err(GridPointError::BadPosError));
            assert_eq!(grid.eat(&Vector2(-1, -1)), Err(GridPointError::BadPosError));
            assert_eq!(grid.eat(&Vector2((grid.width - 1) as i32, (grid.height - 1) as i32)), Err(GridPointError::InconsumableError(GridPoint::Wall)));
        }
    }
}