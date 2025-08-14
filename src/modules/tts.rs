#[cfg(feature = "tts")]
use anyhow::Result;

#[cfg(feature = "tts")]
pub fn speak(settings: &crate::settings::Settings, text: &str) -> Result<()> {
	let mut engine = tts::Tts::default()?;
	if let Some(v) = &settings.tts_voice {
		if let Ok(voices) = engine.voices() {
			if let Some(found) = voices.into_iter().find(|vv| vv.name.eq_ignore_ascii_case(v)) {
				let _ = engine.set_voice(&found);
			}
		}
	}
	if let Some(rate) = settings.tts_rate { let _ = engine.set_rate(rate); }
	if let Some(pitch) = settings.tts_pitch { let _ = engine.set_pitch(pitch); }
	if let Some(vol) = settings.tts_volume { let _ = engine.set_volume(vol); }
	engine.speak(text, false)?;
	Ok(())
} 