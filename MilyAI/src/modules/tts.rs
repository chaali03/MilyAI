#[cfg(feature = "tts")]
use anyhow::Result;

#[cfg(feature = "tts")]
pub fn speak(text: &str) -> Result<()> {
	let mut engine = tts::Tts::default()?;
	engine.speak(text, false)?;
	Ok(())
} 