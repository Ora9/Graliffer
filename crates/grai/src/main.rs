use action::Timeline;
use grai::{FrameGuard, PositionError};

fn main() -> Result<(), PositionError> {
    let frame_file = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/gri.json"));

    let frame = FrameGuard::new(serde_json::from_str(frame_file).expect("frame file invalid"));

    let mut timeline = Timeline::new(frame.clone());

    frame.read(|frame| {
        println!("{}", serde_json::to_string_pretty(&frame).unwrap());
    });

    Ok(())
}
