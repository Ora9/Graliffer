use action::{AnyAction, Timeline};
use grai::{Cell, Frame, FrameGuard, Grid, GridAction, Head, Position, PositionError, Stack};

fn main() -> Result<(), PositionError> {
    let frame_file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/gri.json"));

    let frame = FrameGuard::new(serde_json::from_str(frame_file).expect("frame file invalid"));

    // let frame = FrameGuard::new(Frame {
    //     grid: Grid::new(),
    //     head: Head::default(),
    //     stack: Stack::default(),
    // });

    let mut timeline = Timeline::new(frame.clone());

    frame.read(|frame| {
        let frame_json = serde_json::to_string_pretty(&frame).unwrap();

        println!("{frame_json}");
    });

    Ok(())
}
