use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

mod error;

use error::StreamgenError;

pub fn generate_stream_segments_for_path(
    path: &Path,
    fs_root: &Path,
) -> Result<(), StreamgenError> {
    // Resolve the given path completely
    let pathbuf = path
        .canonicalize()
        .map_err(|_| StreamgenError::PathCanonicalizeError {
            path: path.to_owned(),
        })?;

    // If this path does not exist on disk, return early
    if !pathbuf.exists() {
        return Err(StreamgenError::NonExistentPath {
            path: path.to_owned(),
        });
    }

    // If this path is not a file, return early
    if !pathbuf.is_file() {
        return Err(StreamgenError::FileResolutionError {
            pathbuf: pathbuf.to_owned(),
        });
    }

    let path_str = pathbuf
        .to_str()
        .ok_or(StreamgenError::FileResolutionError {
            pathbuf: pathbuf.to_owned(),
        })?;

    // Figure out the name of the file without the extension.
    // This will be the name of the directory that will hold everything to do with this stream (AKA the stream directory)
    let mut filename_sans_extension = pathbuf.clone();
    filename_sans_extension.set_extension("");

    // Figure out the path of this file's parent
    let parent_dir = pathbuf
        .parent()
        .ok_or(StreamgenError::ParentDirResolutionError {
            path: path.to_owned(),
        })?;

    // Figure out the path of the stream directory
    let stream_dir = parent_dir.join(filename_sans_extension);
    let stream_dir_str = stream_dir
        .to_str()
        .ok_or(StreamgenError::StreamDirResolutionError {
            path: path.to_owned(),
        })?;

    // Create an (empty) stream directory
    if stream_dir.exists() {
        // Remove any old streamgen attempts
        fs::remove_dir_all(&stream_dir).map_err(|e| StreamgenError::PreparationError {
            reason: e.to_string(),
        })?;
    }
    fs::create_dir(&stream_dir).map_err(|e| StreamgenError::PreparationError {
        reason: e.to_string(),
    })?;

    // Figure out the path for the stream segments
    let segment_dir = stream_dir.join("segments");

    // Create the segments directory
    fs::create_dir(&segment_dir).map_err(|e| StreamgenError::PreparationError {
        reason: e.to_string(),
    })?;

    // Figure out the HLS base URL. This will be where the HTML client will request stream segments from
    // We want this to be `/stream/<relative-path-to-file>/segments` for API design reasons
    let hls_base_url = segment_dir.strip_prefix(fs_root).map_err(|_| {
        StreamgenError::HlsBaseUrlResolutionError {
            pathbuf: path.to_owned(),
        }
    })?;
    let hls_base_url = hls_base_url
        .to_str()
        .ok_or(StreamgenError::HlsBaseUrlResolutionError {
            pathbuf: path.to_owned(),
        })?;
    let hls_base_url = format!("/stream/{}/", hls_base_url);

    // Then, let FFMPEG loose
    Command::new("ffmpeg")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(stream_dir_str)
        .arg("-i")
        .arg(path_str)
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
        .arg(hls_base_url)
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

// Not much of a test is it
#[test]
fn test() {
    let path = Path::new("/home/ozark/nas_root/root/Movies/Wildlife.mp4");
    let fs_root = Path::new("/home/ozark/nas_root/root");
    generate_stream_segments_for_path(path, fs_root);
}
