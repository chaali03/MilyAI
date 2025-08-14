#![cfg(all(feature = "stt-vosk", feature = "tts"))]
use anyhow::Result;
use crate::modules::stt;

const WAKE_WORDS: &[&str] = &["milly", "mil ai", "milay", "meli"]; // include common mishears

pub fn listen_for_wake_and_query(model_path: Option<&std::path::Path>) -> Result<Option<String>> {
	let heard = stt::transcribe_for_secs(model_path, 2)?;
	let heard_lower = heard.to_lowercase();
	let is_wake = WAKE_WORDS.iter().any(|w| heard_lower.contains(w));
	if !is_wake { return Ok(None); }
	Ok(Some(stt::transcribe_for_secs(model_path, 5)?))
} 