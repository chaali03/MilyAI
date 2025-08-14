use anyhow::Result;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf, sync::Arc};

use crate::settings::Settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRecord {
	pub when: DateTime<Utc>,
	pub role: String,
	pub text: String,
}

pub struct MemoryStore {
	path: PathBuf,
	inner: Arc<Mutex<()>>, // simple file lock
}

impl MemoryStore {
	pub fn new(settings: &Settings) -> Result<Self> {
		let proj = ProjectDirs::from("com", "MilyAI", "milyai").expect("dirs");
		let data_dir = proj.data_dir();
		fs::create_dir_all(data_dir)?;
		let path = settings
			.memory_path
			.clone()
			.unwrap_or_else(|| data_dir.join("memory.ndjson"))
		;
		Ok(Self { path, inner: Arc::new(Mutex::new(())) })
	}

	pub fn append_interaction(&self, user: &str, assistant: &str) -> Result<()> {
		let _guard = self.inner.lock();
		let mut file = fs::OpenOptions::new().create(true).append(true).open(&self.path)?;
		let u = MessageRecord { when: Utc::now(), role: "user".into(), text: user.into() };
		let a = MessageRecord { when: Utc::now(), role: "assistant".into(), text: assistant.into() };
		writeln!(file, "{}", serde_json::to_string(&u)?)?;
		writeln!(file, "{}", serde_json::to_string(&a)?)?;
		Ok(())
	}

	pub fn recall_recent(&self, limit_pairs: usize) -> Result<String> {
		let _guard = self.inner.lock();
		let content = match fs::read_to_string(&self.path) {
			Ok(s) => s,
			Err(_) => String::new(),
		};
		let mut rows: Vec<MessageRecord> = Vec::new();
		for line in content.lines().rev() {
			if line.trim().is_empty() { continue; }
			if let Ok(r) = serde_json::from_str::<MessageRecord>(line) {
				rows.push(r);
			}
			if rows.len() >= limit_pairs * 2 { break; }
		}
		rows.reverse();
		let mut buf = String::new();
		for r in rows {
			buf.push_str(&format!("[{when}] {role}: {text}\n",
				when = r.when.to_rfc3339(),
				role = r.role,
				text = r.text,
			));
		}
		Ok(buf)
	}
} 