#![doc()]

//! Helper crate to easily use the Rodio audio playback library by providing helper functions
//! 
//! This crate aims to simplify using Rodio in your projects by providing many functions to easily
//! create sinks and play audio with only a few lines of code.
//! 
//! Playing a sound is as simple as this:
//! ```
//! use rotilities::*;
//! 
//! fn main() {
//!     let (_stream, stream_handle) = init();
//!     let sink = new_sink(&stream_handle);
//!     play_audio(&sink, "path/to/file.mp3");
//!     loop{}
//! }
//! ```
//! #### A detailed documentation is coming soom.


use std::{fs::File, io::BufReader};
#[cfg(not(target_arch = "aarch64"))]
use rodio::Decoder;

///Internal helper function for this crate
#[cfg(not(target_arch = "aarch64"))]
pub fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = rodio::Decoder::new(
        BufReader::new(
            File::open(format!("{filename}"))
                .expect("Failed to open file")
        )
    );
    return source.expect("Failed to decode file")
}

///This function "initiates" the crucial part of audio output, the stream and stream handle, and is thus always required
#[cfg(not(target_arch = "aarch64"))]
pub fn init() -> (rodio::OutputStream, rodio::OutputStreamHandle) {
    return rodio::OutputStream::try_default()
        .expect("Creating OuputStream Failed")
}

///This function creates a sink (audio stream) and returns it (for example to a variable). Needs an existing stream handle.
///-> Maximum control, all Rodio sink functions work with this
///
/// # Example
///
/// ```
/// let (_stream, stream_handle) = init()
/// let sink = new_sink(&stream_handle);
/// ```
#[cfg(not(target_arch = "aarch64"))]
pub fn new_sink(stream_handle: &rodio::OutputStreamHandle) -> rodio::Sink {
    return rodio::Sink::try_new(stream_handle).expect("Sink creation failed")
}


//These functions require an existing sink and will manipulate it

/// Add an audio file to the queue of a sink.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// let sink = new_sink();
/// play_audio(&sink, "path/to/file.mp3");
/// ```
#[cfg(not(target_arch = "aarch64"))]
pub fn play_audio(sink: &rodio::Sink, filename: &str) {
    sink.append(get_source(&format!("{filename}")));
}

/// Stops audio playback and clears the queue of a sink
/// Requires an existing sink
///
/// # Example
///
/// ```
/// let sink = new_sink();
///
/// play_audio(&sink, "path/to/file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// stop_audio(&sink);
/// ```
#[cfg(not(target_arch = "aarch64"))]
pub fn stop_audio(sink: &rodio::Sink) {
    sink.stop();
}

/// Pauses audio playback of a sink without affecting the sink's queue.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// let sink = new_sink();
///
/// play_audio(&sink, path/to/file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// pause_audio(&sink);
/// ```
#[cfg(not(target_arch = "aarch64"))]
pub fn pause_audio(sink: &rodio::Sink) {
    sink.pause();
}

/// Resumes audio playback of a sink.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// let sink = new_sink();
///
/// play_audio(&sink, "path/to/file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// pause_audio(&sink);
/// std::thread::sleep(std::time::Duration::from_secs(2));
///
/// resume_audio(&sink);
/// ```
#[cfg(not(target_arch = "aarch64"))]
pub fn resume_audio(sink: &rodio::Sink) {
    sink.play();
}

/// Set the volume level of a sink.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// let sink = new_sink();
///
/// play_audio(&sink, "path/to/file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// set_audio_volume(&sink, 0.5);
/// ```
#[cfg(not(target_arch = "aarch64"))]
pub fn set_audio_volume(sink: &rodio::Sink, volume: f32) {
    sink.set_volume(volume);
}

#[cfg(target_arch = "aarch64")]
pub fn init() -> (String,String) {
    let string = String::new();
    return (string.clone(),string)
}

#[cfg(target_arch = "aarch64")]
pub fn new_sink(_p: String) {}

#[cfg(target_arch = "aarch64")]
pub fn play_audio(_p: String) {}

#[cfg(target_arch = "aarch64")]
pub fn stop_audio(_p: String) {}

#[cfg(target_arch = "aarch64")]
pub fn pause_audio(_p: String) {}

#[cfg(target_arch = "aarch64")]
pub fn resume_audio(_p: String) {}

#[cfg(target_arch = "aarch64")]
pub fn set_audio_volume(_p: String,_p2: f32) {}
