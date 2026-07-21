use std::{any::Any, cell::RefCell, rc::Rc};

use action::{AnyAction, Timeline};
use grai::{Cell, Frame, FrameGuard, Grid, GridAction, Head, Position, PositionError, Stack};

fn main() -> Result<(), PositionError> {
    let mut frame = FrameGuard::new(Frame {
        grid: Grid::new(),
        head: Head::default(),
        stack: Stack::default(),
    });

    let mut timeline = Timeline::new(frame.clone());

    timeline.act(AnyAction::new(GridAction::Set(
        Position::from_numeric(5, 8).unwrap(),
        Cell::new_trim("yey"),
    )));

    timeline.act(AnyAction::new(GridAction::Set(
        Position::from_numeric(0, 0).unwrap(),
        Cell::new_trim("gri"),
    )));

    frame.read(|frame| {
        let frame_json = serde_json::to_string_pretty(&frame).unwrap();

        println!("{frame_json}");
    });

    // {
    //     let frame = frame.try_borrow_mut().unwrap();

    //     let frame_json = serde_json::to_string_pretty::<Frame>(&frame).unwrap();
    //     println!("{}", frame_json);

    //     let frame_from = serde_json::from_str::<Frame>(&frame_json);
    //     dbg!(frame_from);
    // }

    Ok(())
}
