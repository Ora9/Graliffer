use grai::{Cell, Frame, Grid, GridAction, Head, HeadAction, Position, PositionError, Stack};

fn main() -> Result<(), PositionError> {
    let mut frame = Frame {
        grid: Grid::new(),
        head: Head::default(),
        stack: Stack::default(),
    };

    frame.act(HeadAction::MoveTo(
        frame.head.position.checked_increment_x_by(5).unwrap(),
    ));

    dbg!(frame.act(GridAction::Set(frame.head.position, Cell::new_trim("pro"))));

    dbg!(&frame);
    // grid.set(
    //     Position::from_numeric(50, 0).unwrap(),
    //     Cell::new_trim("ouinon"),
    // );

    // dbg!(grid);

    Ok(())
}
