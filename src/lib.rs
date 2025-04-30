#![doc()]
//! # Description
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
//!     let sink = new_sink();
//!     play_audio(&sink, "file.mp3");
//!     loop{}
//! }
//! 
//! ### NOTE THAT EVEN THE LATEST VERSION CURRENTLY DOES NOT SEEM TO WORK
//! #### I am trying to find and fix the issue right now
//! ```


use std::{fs::File, io::BufReader};
use rodio::Decoder;

///Internal helper function for this crate
pub fn get_source(filename: &str) -> Decoder<BufReader<File>> {
    let source = rodio::Decoder::new(
        BufReader::new(
            File::open(format!("./{filename}"))
                .expect("Failed to open file")
        )
    );
    return source.expect("Failed to decode file")
}


///This function creates a sink (audio stream) and returns it (for example to a variable).
///-> Maximum control, all Rodio sink functions work with this
///
/// # Example
///
/// ```
/// let sink = new_sink();
/// ```
pub fn new_sink() -> rodio::Sink {
    let (_stream,stream_handle) = rodio::OutputStream::try_default()
        .expect("Creating OuputStream Failed");
    let sink = rodio::Sink::try_new(&stream_handle)
        .expect("Sink creation failed");
    return sink;
}


//These functions require an existing sink and will manipulate it

/// Add an audio file to the queue of the given sink.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// let sink = new_sink();
/// play_audio(&sink, "file.mp3");
/// ```
pub fn play_audio(sink: rodio::Sink, filename: &str) -> rodio::Sink {
    sink.append(get_source(filename));
    return sink
}

/// Stops audio playback and clears the queue of a sink
/// Requires an existing sink
///
/// # Example
///
/// ```
/// //this example plays the sound "file.mp3" for 5 seconds, then stops it.
///
/// let sink = new_sink();
///
/// play_audio(&sink, "file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// stop_audio(&sink);
/// ```
pub fn stop_audio(sink: rodio::Sink) -> rodio::Sink {
    sink.stop();
    return sink;
}

/// Pauses audio playback of a sink without affecting the sink's queue.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// //this example plays the sound "file.mp3" for 5 seconds, then pauses it
///
/// let sink = new_sink();
///
/// play_audio(&sink, "file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// pause_audio(&sink);
/// ```
pub fn pause_audio(sink: rodio::Sink) -> rodio::Sink {
    sink.pause();
    return sink;
}

/// Resumes audio playback of a sink.
/// Requires and existing sink
///
/// # Example
///
/// ```
/// //this example plays the sound "file.mp3" for 5 seconds, pauses it for 2 seconds, then resumes it
///
/// let sink = new_sink();
///
/// play_audio(&sink, "file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// pause_audio(&sink);
/// std::thread::sleep(std::time::Duration::from_secs(2));
///
/// resume_audio(&sink);
/// ```
pub fn resume_audio(sink: rodio::Sink) -> rodio::Sink {
    sink.play();
    return sink;
}

/// Set the volume level of a sink.
/// Requires an existing sink
///
/// # Example
///
/// ```
/// //this example plays the sound "file.mp3" and halves the volume after 5 seconds.
///
/// let sink = new_sink();
///
/// play_audio(&sink, "file.mp3");
/// std::thread::sleep(std::time::Duration::from_secs(5));
///
/// set_audio_volume(&sink, 0.5);
/// ```
pub fn set_audio_volume(sink: rodio::Sink, volume: f32) -> rodio::Sink {
    sink.set_volume(volume);
    return sink;
}


/// This function will create its own Sink and directly play audio using it.
/// -> Less Control but useful for playing short or multiple overlapping sounds
pub fn play_audio_once(filename: &str){
    let sink = new_sink();
    play_audio(sink, filename);
}
