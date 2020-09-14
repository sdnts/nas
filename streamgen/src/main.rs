use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

mod error;

use error::StreamgenError;

fn main() -> Result<(), StreamgenError> {
    let fs_root = "/home/ozark/nas_root/root";
    let relative_path_str = "Movies/Big Buck Bunny/original.avi";

    let pathbuf = PathBuf::new().join(fs_root).join(relative_path_str);
    let pathbuf = pathbuf
        .canonicalize()
        .map_err(|_| StreamgenError::PathCanonicalizeError {
            path: pathbuf.to_owned(),
        })?;

    if !pathbuf.exists() {
        return Err(StreamgenError::NonExistentPath {
            path: relative_path_str.to_string(),
        });
    }

    if !pathbuf.is_file() {
        return Err(StreamgenError::FileResolutionError {
            pathbuf: pathbuf.to_owned(),
        });
    }

    let filename = pathbuf
        .file_name()
        .ok_or(StreamgenError::FileResolutionError {
            pathbuf: pathbuf.to_owned(),
        })?;
    let filename = filename
        .to_str()
        .ok_or(StreamgenError::FileResolutionError {
            pathbuf: pathbuf.to_owned(),
        })?;

    let parent_dir = pathbuf
        .parent()
        .ok_or(StreamgenError::ParentDirResolutionError {
            path: relative_path_str.to_string(),
        })?;
    let parent_dir_str = parent_dir
        .to_str()
        .ok_or(StreamgenError::ParentDirResolutionError {
            path: relative_path_str.to_string(),
        })?;

    let relative_parent_dir = parent_dir.strip_prefix(fs_root).map_err(|_| {
        StreamgenError::RelativeParentDirResolutionError {
            fs_root: fs_root.to_string(),
            path: parent_dir_str.to_string(),
        }
    })?;
    let relative_parent_dir_str =
        relative_parent_dir
            .to_str()
            .ok_or(StreamgenError::RelativeParentDirResolutionError {
                fs_root: fs_root.to_string(),
                path: parent_dir_str.to_string(),
            })?;

    let segment_path = parent_dir.join("segments");

    // Remove any old streamgen attempts
    fs::remove_dir_all(&segment_path).map_err(|e| StreamgenError::PreparationError {
        reason: e.to_string(),
    })?;

    // Create a directory where stream segments will go
    fs::create_dir(&segment_path).map_err(|e| StreamgenError::PreparationError {
        reason: e.to_string(),
    })?;

    // Then, let FFMPEG loose
    Command::new("ffmpeg")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(parent_dir_str)
        .arg("-i")
        .arg(filename)
        .arg("-level")
        .arg("4.0")
        .arg("-start_number")
        .arg("0")
        .arg("-f")
        .arg("hls")
        .arg("-hls_time")
        .arg("5")
        .arg("-hls_list_size")
        .arg("0")
        .arg("-hls_segment_filename")
        .arg("segments/%06d.ts")
        .arg("-hls_base_url")
        .arg(format!("/stream/{}/segments/", relative_parent_dir_str))
        .arg("-vcodec")
        .arg("libx264")
        .arg("-acodec")
        .arg("aac")
        .arg("-ar")
        .arg("44100")
        .arg("-ac")
        .arg("2")
        .arg("playlist.m3u8")
        .output()
        .map_err(|e| StreamgenError::FfmpegError { source: e })?;

    Ok(())
}
