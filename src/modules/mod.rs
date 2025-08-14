#[cfg(feature = "tts")]
pub mod tts;
#[cfg(feature = "stt-vosk")]
pub mod stt;
#[cfg(feature = "camera")]
pub mod camera;
#[cfg(all(feature = "stt-vosk", feature = "tts"))]
pub mod voice;
#[cfg(feature = "web")]
pub mod web; 