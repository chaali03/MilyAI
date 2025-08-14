#![cfg(feature = "web")]
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use url::Url;

use crate::settings::Settings;

pub async fn fetch_text(settings: &Settings, url: &str) -> Result<String> {
	check_domain_policy(settings, url)?;
	if should_block_by_robots(settings, url).await? { return Err(anyhow!("Blocked by robots.txt")); }
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

fn check_domain_policy(settings: &Settings, url: &str) -> Result<()> {
	let host = Url::parse(url).ok().and_then(|u| u.host_str().map(|s| s.to_string()));
	if host.is_none() { return Ok(()); }
	let host = host.unwrap();
	if let Some(deny) = &settings.deny_domains { if deny.iter().any(|d| host.ends_with(d)) { return Err(anyhow!("Domain denied")); } }
	if let Some(allow) = &settings.allow_domains { if !allow.is_empty() && !allow.iter().any(|a| host.ends_with(a)) { return Err(anyhow!("Domain not in allowlist")); } }
	Ok(())
}

async fn should_block_by_robots(settings: &Settings, url: &str) -> Result<bool> {
	#[cfg(feature = "robots")]
	{
		if settings.respect_robots.unwrap_or(true) {
			let parsed = Url::parse(url)?;
			let robots_url = format!("{}://{}/robots.txt", parsed.scheme(), parsed.host_str().unwrap_or(""));
			let txt = reqwest::get(&robots_url).await.ok().and_then(|r| r.text().await.ok()).unwrap_or_default();
			let agent = settings.web_user_agent.clone().unwrap_or_else(|| "MilyAI".to_string());
			let rules = robotstxt::DefaultMatcher::default();
			let parsed = robotstxt::RobotFileParser::parse(&txt);
			return Ok(!parsed.allowed_by_robots(&rules, &agent, &Url::parse(url)?));
		}
	}
	Ok(false)
}

fn extract_text_from_html(html: &str) -> String {
	let doc = Html::parse_document(html);
	let body_sel = Selector::parse("body").unwrap();
	let mut text = String::new();
	if let Some(body) = doc.select(&body_sel).next() {
		for t in body.text() {
			let s = t.trim();
			if !s.is_empty() { text.push_str(s); text.push('\n'); }
		}
	}
	text
} 