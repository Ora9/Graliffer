use std::{any::Any, cell::RefCell, rc::Rc};

use grai::{
    AnyAction, Cell, Frame, Grid, GridAction, Head, HeadAction, Position, PositionError, Stack,
    Timeline,
};
use serde::Serialize;

fn main() -> Result<(), PositionError> {
    let mut frame = Rc::new(RefCell::new(Frame {
        grid: Grid::new(),
        head: Head::default(),
        stack: Stack::default(),
    }));

    // frame.serialize(serde_json::Serializer::new(writer));

    let mut timeline = Timeline::new(frame.clone());

    timeline.act(AnyAction::new(GridAction::Set(
        Position::from_numeric(5, 8).unwrap(),
        Cell::new_trim("yey"),
    )));

    {
        // let frame = frame.clone();
        // let frame = frame.as_ref();
        // let res = {
        let frame = frame.try_borrow_mut().unwrap();
        // state.act(&action)
        // };

        // let stack_json = serde_json::to_string(&frame.stack).unwrap();
        // let stack = serde_json::from_str::<Stack>(&stack_json);
        // dbg!(stack);

        // let head_json = serde_json::to_string(&frame.head).unwrap();
        // let head = serde_json::from_str::<Head>(&head_json);
        // dbg!(head);

        let frame_json = serde_json::to_string_pretty::<Frame>(&frame).unwrap();
        println!("{}", frame_json);

        let frame_from = serde_json::from_str::<Frame>(&frame_json);
        dbg!(frame_from);
    }

    // dbg!(serde_json::to_string());
    // dbg!(&frame);

    Ok(())
}
