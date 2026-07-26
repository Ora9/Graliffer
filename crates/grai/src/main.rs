use act::Timeline;
use grai::{Frame, FrameAction, FrameGuard, PositionError};

fn dbg_frame(frame: &FrameGuard) {
    frame.read(|frame| {
        println!("{}", serde_json::to_string_pretty(&frame).unwrap());
    });
}

fn main() -> Result<(), PositionError> {
    let frame = FrameGuard::new(Frame::from_example("inst").unwrap());

    let mut timeline = Timeline::new(frame.clone());

    dbg_frame(&frame);

    for _ in 0..5 {
        timeline.act(FrameAction::Step).unwrap();
        dbg_frame(&frame);
    }

    Ok(())
}
