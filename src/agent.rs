use anyhow::{anyhow, Result};
use crate::memory::MemoryStore;
use crate::settings::Settings;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
	pub name: String,
	pub curiosity: f32,
	pub persona: String,
}

pub struct Agent {
	settings: Settings,
	memory: Arc<MemoryStore>,
	profile: AgentProfile,
}

impl Agent {
	pub fn new(settings: Settings) -> Result<Self> {
		let memory = Arc::new(MemoryStore::new(&settings)?);
		let profile = AgentProfile {
			name: settings.agent_name.clone().unwrap_or_else(|| "Mily".to_string()),
			curiosity: settings.curiosity.unwrap_or(0.6),
			persona: settings.persona.clone().unwrap_or_else(|| "Ramah, ingin tahu, membantu".to_string()),
		};
		Ok(Self { settings, memory, profile })
	}

	pub async fn respond(&mut self, user_input: &str) -> Result<String> {
		let context = self.memory.recall_recent(8)?;
		let prompt = self.build_prompt(user_input, &context)?;
		let reply = self.call_llm(&prompt).await?;
		self.memory.append_interaction(user_input, &reply)?;
		Ok(reply)
	}

	fn build_prompt(&self, user_input: &str, context: &str) -> Result<String> {
		let mut seed: u64 = rand::thread_rng().gen();
		let system = format!(
			"Anda adalah {name}, asisten AI berbahasa Indonesia yang ingin tahu dan berkembang. Persona: {persona}. \nGunakan nada yang sopan, ringkas.\n",
			name = self.profile.name,
			persona = self.profile.persona,
		);
		let prompt = format!(
			"<SYSTEM>\n{system}\n<CONTEXT>\n{context}\n</CONTEXT>\n<USER>\n{user}\n</USER>\n",
			system = system,
			context = context,
			user = user_input,
		);
		let _ = seed; // reserved for stochastic settings later
		Ok(prompt)
	}

	async fn call_llm(&self, prompt: &str) -> Result<String> {
		match &self.settings.llm_endpoint {
			Some(url) => {
				let client = reqwest::Client::new();
				#[derive(Serialize)]
				struct Req<'a> { prompt: &'a str }
				#[derive(Deserialize)]
				struct Resp { text: String }
				let resp = client.post(url).json(&Req { prompt }).send().await?;
				if !resp.status().is_success() {
					return Err(anyhow!("LLM request failed: {}", resp.status()));
				}
				let data: Resp = resp.json().await?;
				Ok(data.text)
			}
			None => {
				// Placeholder offline reply so the app runs
				Ok("[Mode offline] Saya mendengar Anda. Sambungkan LLM dengan --llm-endpoint di config untuk jawaban cerdas.".to_string())
			}
		}
	}
} 