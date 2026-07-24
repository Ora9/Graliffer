use std::{error, path::PathBuf};

use crate::Frame;

#[iftree::include_file_tree("paths = '/assets/**'")]
pub struct Example {
    relative_path: &'static str,
    contents_str: &'static str,
}

impl Frame {
    pub fn from_example(file_name: &str) -> Option<Self> {
        ASSETS.iter().find_map(|file| {
            if PathBuf::from(file_name) == PathBuf::from(file.relative_path).file_stem()? {
                let frame = serde_json::from_str(file.contents_str).unwrap_or_else(|err| {
                    panic!(
                        "example must contain valid frame, trying to parse `{}`, got error {}",
                        file.relative_path, err
                    )
                });
                Some(frame)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn fetch_each() {
        for file in ASSETS.iter() {
            assert!(
                Frame::from_example(
                    PathBuf::from(file.relative_path)
                        .file_stem()
                        .expect("expected valid path in `ASSETS`")
                        .to_str()
                        .unwrap()
                )
                .is_some()
            );
        }
    }
}
