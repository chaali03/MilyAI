#![cfg(feature = "feeds")]
use anyhow::Result;
use rss::Channel;

pub async fn fetch_feed_text(url: &str) -> Result<Vec<String>> {
	let body = reqwest::get(url).await?.bytes().await?;
	let ch = Channel::read_from(&body[..])?;
	let mut items = Vec::new();
	for i in ch.items() {
		let title = i.title().unwrap_or("");
		let desc = i.description().unwrap_or("");
		let content = format!("{}\n{}", title, desc).trim().to_string();
		if !content.is_empty() { items.push(content); }
	}
	Ok(items)
} 