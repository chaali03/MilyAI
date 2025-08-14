use anyhow::{anyhow, Result};
use crate::memory::MemoryStore;
use crate::settings::Settings;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::llm::LlmClient;

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
	llm: LlmClient,
}

impl Agent {
	pub fn new(settings: Settings) -> Result<Self> {
		let memory = Arc::new(MemoryStore::new(&settings)?);
		let profile = AgentProfile {
			name: settings.agent_name.clone().unwrap_or_else(|| "Mily".to_string()),
			curiosity: settings.curiosity.unwrap_or(0.6),
			persona: settings.persona.clone().unwrap_or_else(|| "Ramah, ingin tahu, membantu".to_string()),
		};
		let llm = LlmClient::new(settings.clone());
		Ok(Self { settings, memory, profile, llm })
	}

	pub async fn respond(&mut self, user_input: &str) -> Result<String> {
		let context = self.memory.recall_recent(8)?;
		let prompt = self.build_prompt(user_input, &context)?;
		let reply = self.llm.generate(&prompt).await.unwrap_or_else(|_| "[offline] LLM unavailable".to_string());
		self.memory.append_interaction(user_input, &reply)?;
		Ok(reply)
	}

	#[cfg(feature = "web")]
	pub async fn summarize_and_learn(&mut self, source: &str, text: &str) -> Result<String> {
		let instruction = format!(
			"Ringkas konten berikut dalam 5-8 poin (bahasa Indonesia), fokuskan pada fakta inti dan insight. Sumber: {src}\n\n{body}",
			src = source,
			body = text,
		);
		let context = self.memory.recall_recent(4)?;
		let prompt = format!("<SYSTEM>Anda adalah {name} yang ingin tahu dan sedang belajar dari web.</SYSTEM>\n<CONTEXT>\n{ctx}\n</CONTEXT>\n<USER>\n{inst}\n</USER>", name = self.profile.name, ctx = context, inst = instruction);
		let summary = self.llm.generate(&prompt).await.unwrap_or_else(|_| "[offline] summary unavailable".to_string());
		let note_user = format!("LEARN FROM: {src}", src = source);
		self.memory.append_interaction(&note_user, &summary)?;
		Ok(summary)
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
} 