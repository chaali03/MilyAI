#![cfg(all(feature = "stt-vosk", feature = "tts"))]
use anyhow::Result;
use crate::modules::{stt, tts};
use crate::agent::Agent;
use crate::settings::Settings;

pub async fn run(settings: Settings) -> Result<()> {
	let mut agent = Agent::new(settings.clone())?;
	println!("Voice chat mode. Speak; it will auto-detect silence. Ctrl+C to exit.");
	loop {
		let user = stt::transcribe_until_silence(settings.stt_model_path.as_deref(), 900, 20)?;
		if user.trim().is_empty() { continue; }
		println!("You: {}", user);
		let reply = agent.respond(&user).await?;
		println!("Mily: {}", reply);
		let _ = tts::speak(&reply);
	}
} 