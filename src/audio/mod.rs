//! # Audio
//!
//! Audio engine and resources

use rodio::source::{Amplify, SineWave, Source, TakeDuration};
use rodio::{OutputStream, Sink};

/// Donmaze audio engine
pub struct AudioEngine {
    sink: Sink,
}
