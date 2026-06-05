use grai::{Cell, Grid, Position};

fn main() {
    let mut grid = Grid::new();

    grid.set(Position::ORIGIN, Cell::new_trim("ouinon"));

    dbg!(grid);
}
