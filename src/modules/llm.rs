use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::settings::Settings;

#[derive(Debug, Clone)]
pub enum LlmProvider {
	Endpoint,
	#[cfg(feature = "llm-openai")]	OpenAi,
	#[cfg(feature = "llm-ollama")]	Ollama,
	Offline,
}

pub struct LlmClient {
	settings: Settings,
	provider: LlmProvider,
}

impl LlmClient {
	pub fn new(settings: Settings) -> Self {
		let provider = {
			#[allow(unused_mut)]
			let mut p = LlmProvider::Offline;
			#[cfg(feature = "llm-openai")]
			if settings.openai_api_key.is_some() { p = LlmProvider::OpenAi; }
			#[cfg(feature = "llm-ollama")]
			if settings.ollama_url.is_some() { p = LlmProvider::Ollama; }
			if settings.llm_endpoint.is_some() { p = LlmProvider::Endpoint; }
			p
		};
		Self { settings, provider }
	}

	pub async fn generate(&self, prompt: &str) -> Result<String> {
		match self.provider {
			LlmProvider::Endpoint => self.generate_via_endpoint(prompt).await,
			#[cfg(feature = "llm-openai")]
			LlmProvider::OpenAi => self.generate_via_openai(prompt).await,
			#[cfg(feature = "llm-ollama")]
			LlmProvider::Ollama => self.generate_via_ollama(prompt).await,
			LlmProvider::Offline => Ok("[offline] Connect a local LLM (Ollama) or set llm_endpoint".to_string()),
		}
	}

	async fn generate_via_endpoint(&self, prompt: &str) -> Result<String> {
		let url = self.settings.llm_endpoint.as_ref().ok_or_else(|| anyhow!("llm_endpoint not configured"))?;
		#[derive(Serialize)]
		struct Req<'a> { prompt: &'a str }
		#[derive(Deserialize)]
		struct Resp { text: String }
		let client = reqwest::Client::new();
		let resp = client.post(url).json(&Req { prompt }).send().await?;
		if !resp.status().is_success() { return Err(anyhow!("LLM request failed: {}", resp.status())); }
		let data: Resp = resp.json().await?;
		Ok(data.text)
	}

	#[cfg(feature = "llm-openai")]
	async fn generate_via_openai(&self, prompt: &str) -> Result<String> {
		use async_openai::types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, Role};
		use async_openai::Client;
		let api_key = self.settings.openai_api_key.clone().ok_or_else(|| anyhow!("OPENAI_API_KEY not set"))?;
		let model = self.settings.openai_model.clone().unwrap_or_else(|| "gpt-4o-mini".to_string());
		let temperature = self.settings.temperature.unwrap_or(0.6);
		let client = Client::with_api_key(api_key);
		let req = CreateChatCompletionRequestArgs::default()
			.model(model)
			.temperature(temperature)
			.messages([
				ChatCompletionRequestMessageArgs::default().role(Role::System).content("You are Mily, a warm, natural, concise Indonesian conversationalist. Match the user's tone (friendly, caring). Prefer 1-3 sentences unless asked for detail.").build()?,
				ChatCompletionRequestMessageArgs::default().role(Role::User).content(prompt).build()?,
			])
			.build()?;
		let resp = client.chat().create(req).await?;
		let choice = resp.choices.first().ok_or_else(|| anyhow!("no choices"))?;
		let content = choice.message.content.clone().unwrap_or_default();
		Ok(content)
	}

	#[cfg(feature = "llm-ollama")]
	async fn generate_via_ollama(&self, prompt: &str) -> Result<String> {
		#[derive(Serialize)]
		struct Req<'a> { model: &'a str, prompt: &'a str, stream: bool, temperature: f32 }
		#[derive(Deserialize)]
		struct Resp { response: String }
		let base = self.settings.ollama_url.clone().unwrap_or_else(|| "http://127.0.0.1:11434".to_string());
		let model = self.settings.ollama_model.clone().unwrap_or_else(|| "llama3.1:8b".to_string());
		let url = format!("{}/api/generate", base);
		let temperature = self.settings.temperature.unwrap_or(0.6);
		let client = reqwest::Client::new();
		let resp = client.post(&url).json(&Req { model: &model, prompt, stream: false, temperature }).send().await?;
		if !resp.status().is_success() { return Err(anyhow!("Ollama request failed: {}", resp.status())); }
		let data: Resp = resp.json().await?;
		Ok(data.response)
	}
} 