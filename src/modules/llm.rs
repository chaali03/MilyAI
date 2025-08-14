use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::settings::Settings;

#[derive(Debug, Clone)]
pub enum LlmProvider {
	Endpoint, // generic POST {prompt}
	#[cfg(feature = "llm-openai")]	OpenAi,
}

pub struct LlmClient {
	settings: Settings,
	provider: LlmProvider,
}

impl LlmClient {
	pub fn new(settings: Settings) -> Self {
		let provider = if cfg!(feature = "llm-openai") && settings.openai_api_key.is_some() { 
			#[cfg(feature = "llm-openai")]
			{ LlmProvider::OpenAi }
			#[cfg(not(feature = "llm-openai"))]
			{ LlmProvider::Endpoint }
		} else { LlmProvider::Endpoint };
		Self { settings, provider }
	}

	pub async fn generate(&self, prompt: &str) -> Result<String> {
		match self.provider {
			LlmProvider::Endpoint => self.generate_via_endpoint(prompt).await,
			#[cfg(feature = "llm-openai")]
			LlmProvider::OpenAi => self.generate_via_openai(prompt).await,
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
		let client = Client::with_api_key(api_key);
		let req = CreateChatCompletionRequestArgs::default()
			.model(model)
			.messages([
				ChatCompletionRequestMessageArgs::default().role(Role::System).content("You are Mily, a helpful, curious assistant. Respond briefly in Indonesian.").build()?,
				ChatCompletionRequestMessageArgs::default().role(Role::User).content(prompt).build()?,
			])
			.build()?;
		let resp = client.chat().create(req).await?;
		let choice = resp.choices.first().ok_or_else(|| anyhow!("no choices"))?;
		let content = choice.message.content.clone().unwrap_or_default();
		Ok(content)
	}
} 