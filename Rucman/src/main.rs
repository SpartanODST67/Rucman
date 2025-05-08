mod grid;
use grid::grid::{Grid, GridPoint, GridPointError}; //grid.rs -> mod grid -> Grid stuct et al

mod point;
use point::Point;


fn main() {
    let grid = Grid::new();
    println!("{grid:?}");
    print_screen(&grid);
}

fn print_screen(grid: &Grid) {
    for row in grid.get_grid() {
        let mut row_string = String::from("");
        for col in row {
            row_string.push(char::from(*col));
        }
        print!("{row_string}\n");
    }
}
