use grai::{Cell, Grid, Position, PositionError};

fn main() -> Result<(), PositionError> {
    let mut grid = Grid::new();

    grid.set(
        Position::from_numeric(50, 0).unwrap(),
        Cell::new_trim("ouinon"),
    );

    dbg!(grid);

    Ok(())
}
