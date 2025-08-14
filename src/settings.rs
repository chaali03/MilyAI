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
	#[cfg(feature = "web")]
	pub allow_domains: Option<Vec<String>>,
	#[cfg(feature = "web")]
	pub deny_domains: Option<Vec<String>>,
	#[cfg(feature = "robots")]
	pub respect_robots: Option<bool>,
	#[cfg(feature = "llm-openai")]
	pub openai_api_key: Option<String>,
	#[cfg(feature = "llm-openai")]
	pub openai_model: Option<String>,
	#[cfg(feature = "llm-ollama")]
	pub ollama_url: Option<String>,
	#[cfg(feature = "llm-ollama")]
	pub ollama_model: Option<String>,
	#[cfg(feature = "actions")]
	pub allow_dirs: Option<Vec<PathBuf>>,
	#[cfg(feature = "actions")]
	pub allow_apps: Option<Vec<String>>,
	#[cfg(feature = "actions")]
	pub actions_auto_confirm: Option<bool>,
	// Conversation tuning
	pub temperature: Option<f32>,
	pub response_max_sentences: Option<u8>,
	pub speaking_style: Option<String>,
	#[cfg(feature = "tts")]
	pub tts_voice: Option<String>,
	#[cfg(feature = "tts")]
	pub tts_rate: Option<f32>,
	#[cfg(feature = "tts")]
	pub tts_pitch: Option<f32>,
	#[cfg(feature = "tts")]
	pub tts_volume: Option<f32>,
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
	#[cfg(feature = "web")]
	if let Ok(v) = env::var("MILYAI_ALLOW_DOMAINS") { s.allow_domains = Some(v.split(',').map(|s| s.trim().to_string()).collect()); }
	#[cfg(feature = "web")]
	if let Ok(v) = env::var("MILYAI_DENY_DOMAINS") { s.deny_domains = Some(v.split(',').map(|s| s.trim().to_string()).collect()); }
	#[cfg(feature = "robots")]
	if let Ok(v) = env::var("MILYAI_RESPECT_ROBOTS") { s.respect_robots = Some(v == "1" || v.to_lowercase() == "true"); }
	#[cfg(feature = "llm-openai")]
	if let Ok(v) = env::var("OPENAI_API_KEY") { s.openai_api_key = Some(v); }
	#[cfg(feature = "llm-openai")]
	if let Ok(v) = env::var("MILYAI_OPENAI_MODEL") { s.openai_model = Some(v); }
	#[cfg(feature = "llm-ollama")]
	if let Ok(v) = env::var("MILYAI_OLLAMA_URL") { s.ollama_url = Some(v); }
	#[cfg(feature = "llm-ollama")]
	if let Ok(v) = env::var("MILYAI_OLLAMA_MODEL") { s.ollama_model = Some(v); }
	#[cfg(feature = "actions")]
	if let Ok(v) = env::var("MILYAI_ALLOW_DIRS") { s.allow_dirs = Some(v.split(';').map(|s| s.trim().into()).collect()); }
	#[cfg(feature = "actions")]
	if let Ok(v) = env::var("MILYAI_ALLOW_APPS") { s.allow_apps = Some(v.split(',').map(|s| s.trim().to_string()).collect()); }
	#[cfg(feature = "actions")]
	if let Ok(v) = env::var("MILYAI_ACTIONS_AUTO_CONFIRM") { s.actions_auto_confirm = Some(v == "1" || v.to_lowercase() == "true"); }
	if let Ok(v) = env::var("MILYAI_TEMPERATURE") { s.temperature = v.parse().ok(); }
	if let Ok(v) = env::var("MILYAI_RESPONSE_MAX_SENTENCES") { s.response_max_sentences = v.parse().ok(); }
	if let Ok(v) = env::var("MILYAI_SPEAKING_STYLE") { s.speaking_style = Some(v); }
	#[cfg(feature = "tts")]
	if let Ok(v) = env::var("MILYAI_TTS_VOICE") { s.tts_voice = Some(v); }
	#[cfg(feature = "tts")]
	if let Ok(v) = env::var("MILYAI_TTS_RATE") { s.tts_rate = v.parse().ok(); }
	#[cfg(feature = "tts")]
	if let Ok(v) = env::var("MILYAI_TTS_PITCH") { s.tts_pitch = v.parse().ok(); }
	#[cfg(feature = "tts")]
	if let Ok(v) = env::var("MILYAI_TTS_VOLUME") { s.tts_volume = v.parse().ok(); }
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
	#[cfg(feature = "web")]
	if other.allow_domains.is_some() { base.allow_domains = other.allow_domains; }
	#[cfg(feature = "web")]
	if other.deny_domains.is_some() { base.deny_domains = other.deny_domains; }
	#[cfg(feature = "robots")]
	if other.respect_robots.is_some() { base.respect_robots = other.respect_robots; }
	#[cfg(feature = "llm-openai")]
	if other.openai_api_key.is_some() { base.openai_api_key = other.openai_api_key; }
	#[cfg(feature = "llm-openai")]
	if other.openai_model.is_some() { base.openai_model = other.openai_model; }
	#[cfg(feature = "llm-ollama")]
	if other.ollama_url.is_some() { base.ollama_url = other.ollama_url; }
	#[cfg(feature = "llm-ollama")]
	if other.ollama_model.is_some() { base.ollama_model = other.ollama_model; }
	#[cfg(feature = "actions")]
	if other.allow_dirs.is_some() { base.allow_dirs = other.allow_dirs; }
	#[cfg(feature = "actions")]
	if other.allow_apps.is_some() { base.allow_apps = other.allow_apps; }
	#[cfg(feature = "actions")]
	if other.actions_auto_confirm.is_some() { base.actions_auto_confirm = other.actions_auto_confirm; }
	if other.temperature.is_some() { base.temperature = other.temperature; }
	if other.response_max_sentences.is_some() { base.response_max_sentences = other.response_max_sentences; }
	if other.speaking_style.is_some() { base.speaking_style = other.speaking_style; }
	#[cfg(feature = "tts")]
	if other.tts_voice.is_some() { base.tts_voice = other.tts_voice; }
	#[cfg(feature = "tts")]
	if other.tts_rate.is_some() { base.tts_rate = other.tts_rate; }
	#[cfg(feature = "tts")]
	if other.tts_pitch.is_some() { base.tts_pitch = other.tts_pitch; }
	#[cfg(feature = "tts")]
	if other.tts_volume.is_some() { base.tts_volume = other.tts_volume; }
	base
} 