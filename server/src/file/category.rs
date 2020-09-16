use serde::Serialize;
use std::cmp::Ordering;

#[derive(Debug, Serialize, PartialEq, Eq, Ord)]
pub enum NASFileCategory {
    Directory,
    Audio,
    Video,
    StreamPlaylist,
    StreamSegment,
    Document,
    Image,
    Unknown,
}

impl PartialOrd for NASFileCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if matches!(self, Self::Directory) && matches!(other, Self::Directory) {
            Some(Ordering::Less)
        } else if matches!(self, Self::Directory) && matches!(other, Self::Directory) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
