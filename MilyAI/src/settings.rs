use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
	pub agent_name: Option<String>,
	pub persona: Option<String>,
	pub curiosity: Option<f32>,
	pub llm_endpoint: Option<String>,
	pub memory_path: Option<PathBuf>,
	#[cfg(feature = "stt-vosk")]
	pub stt_model_path: Option<PathBuf>,
	#[cfg(feature = "web")]
	pub web_user_agent: Option<String>,
	#[cfg(feature = "web")]
	pub learn_urls: Option<Vec<String>>,
	#[cfg(feature = "web")]
	pub learn_interval_secs: Option<u64>,
}

pub fn load(path: Option<&str>) -> Result<Settings> {
	let mut s = Settings::default();
	let file_path = match path {
		Some(p) => PathBuf::from(p),
		None => {
			let proj = ProjectDirs::from("com", "MilyAI", "milyai").expect("dirs");
			let cfg_dir = proj.config_dir();
			fs::create_dir_all(cfg_dir)?;
			cfg_dir.join("config.yaml")
		}
	};
	if file_path.exists() {
		let text = fs::read_to_string(&file_path)?;
		let file_cfg: Settings = serde_yaml::from_str(&text)?;
		s = merge(s, file_cfg);
	}
	if let Ok(v) = env::var("MILYAI_AGENT_NAME") { s.agent_name = Some(v); }
	if let Ok(v) = env::var("MILYAI_PERSONA") { s.persona = Some(v); }
	if let Ok(v) = env::var("MILYAI_CURIOSITY") { s.curiosity = v.parse().ok(); }
	if let Ok(v) = env::var("MILYAI_LLM_ENDPOINT") { s.llm_endpoint = Some(v); }
	#[cfg(feature = "web")]
	if let Ok(v) = env::var("MILYAI_WEB_USER_AGENT") { s.web_user_agent = Some(v); }
	#[cfg(feature = "web")]
	if let Ok(v) = env::var("MILYAI_LEARN_INTERVAL_SECS") { s.learn_interval_secs = v.parse().ok(); }
	Ok(s)
}

fn merge(mut base: Settings, other: Settings) -> Settings {
	if other.agent_name.is_some() { base.agent_name = other.agent_name; }
	if other.persona.is_some() { base.persona = other.persona; }
	if other.curiosity.is_some() { base.curiosity = other.curiosity; }
	if other.llm_endpoint.is_some() { base.llm_endpoint = other.llm_endpoint; }
	if other.memory_path.is_some() { base.memory_path = other.memory_path; }
	#[cfg(feature = "stt-vosk")]
	if other.stt_model_path.is_some() { base.stt_model_path = other.stt_model_path; }
	#[cfg(feature = "web")]
	if other.web_user_agent.is_some() { base.web_user_agent = other.web_user_agent; }
	#[cfg(feature = "web")]
	if other.learn_urls.is_some() { base.learn_urls = other.learn_urls; }
	#[cfg(feature = "web")]
	if other.learn_interval_secs.is_some() { base.learn_interval_secs = other.learn_interval_secs; }
	base
} 