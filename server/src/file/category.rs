use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
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
