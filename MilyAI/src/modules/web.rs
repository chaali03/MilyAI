#![cfg(feature = "web")]
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};

use crate::settings::Settings;

pub async fn fetch_text(settings: &Settings, url: &str) -> Result<String> {
	let client = reqwest::Client::builder()
		.user_agent(settings.web_user_agent.clone().unwrap_or_else(|| "MilyAI/0.1 (+https://example.com)".to_string()))
		.build()?;
	let resp = client.get(url).send().await?;
	if !resp.status().is_success() { return Err(anyhow!("Fetch failed: {}", resp.status())); }
	let content_type = resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("").to_lowercase();
	let body = resp.text().await?;
	if content_type.contains("text/html") || body.contains("<html") {
		Ok(extract_text_from_html(&body))
	} else {
		Ok(body)
	}
}

fn extract_text_from_html(html: &str) -> String {
	let doc = Html::parse_document(html);
	let body_sel = Selector::parse("body").unwrap();
	let script_sel = Selector::parse("script,style,noscript").unwrap();
	let mut text = String::new();
	if let Some(body) = doc.select(&body_sel).next() {
		for node in body.children() {
			if let Some(elem) = node.value().as_element() {
				let name = elem.name();
				if name == "script" || name == "style" || name == "noscript" { continue; }
			}
			let frag = Html::parse_fragment(&node.html());
			for t in frag.root_element().text() {
				let s = t.trim();
				if !s.is_empty() {
					text.push_str(s);
					text.push('\n');
				}
			}
		}
	}
	text
} 