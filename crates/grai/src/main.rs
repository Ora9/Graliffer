use std::{cell::RefCell, rc::Rc};

use grai::{
    ActionBox, Cell, Frame, Grid, GridAction, Head, HeadAction, Position, PositionError, Stack,
    Timeline,
};

fn main() -> Result<(), PositionError> {
    let mut frame = Rc::new(RefCell::new(Frame {
        grid: Grid::new(),
        head: Head::default(),
        stack: Stack::default(),
    }));

    let timeline = Timeline::new(frame.clone());

    timeline
        .test(Box::new(GridAction::Set(
            Position::from_numeric(5, 8).unwrap(),
            Cell::new_trim("yey"),
        )))
        .unwrap();

    dbg!(&frame);

    Ok(())
}
