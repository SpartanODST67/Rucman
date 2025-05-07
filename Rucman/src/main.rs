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
