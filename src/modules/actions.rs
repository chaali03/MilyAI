#![cfg(feature = "actions")]
use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::settings::Settings;

pub enum Action {
	OpenUrl(String),
	LaunchApp { app: String, args: Vec<String> },
	ReadFile(PathBuf),
	WriteFile { path: PathBuf, content: String },
}

pub fn execute(settings: &Settings, action: Action) -> Result<String> {
	match action {
		Action::OpenUrl(url) => {
			check_url_allowed(settings, &url)?;
			webbrowser::open(&url)?;
			Ok(format!("Opened URL: {}", url))
		}
		Action::LaunchApp { app, args } => {
			check_app_allowed(settings, &app)?;
			std::process::Command::new(&app).args(&args).spawn()?;
			Ok(format!("Launched: {} {}", app, args.join(" ")))
		}
		Action::ReadFile(path) => {
			check_path_allowed(settings, &path)?;
			let content = fs::read_to_string(&path)?;
			Ok(content)
		}
		Action::WriteFile { path, content } => {
			check_path_allowed(settings, &path)?;
			fs::write(&path, content.as_bytes())?;
			Ok(format!("Wrote: {}", path.display()))
		}
	}
}

fn check_app_allowed(settings: &Settings, app: &str) -> Result<()> {
	let allow = settings.allow_apps.as_ref().ok_or_else(|| anyhow!("No allow_apps configured"))?;
	if !allow.iter().any(|a| a.eq_ignore_ascii_case(app)) {
		return Err(anyhow!("App not allowed"));
	}
	Ok(())
}

fn check_path_allowed(settings: &Settings, path: &Path) -> Result<()> {
	let dirs = settings.allow_dirs.as_ref().ok_or_else(|| anyhow!("No allow_dirs configured"))?;
	let canon = dunce::canonicalize(path).unwrap_or(path.to_path_buf());
	for d in dirs {
		let cd = dunce::canonicalize(d).unwrap_or(d.clone());
		if canon.starts_with(&cd) { return Ok(()); }
	}
	Err(anyhow!("Path not allowed"))
}

fn check_url_allowed(settings: &Settings, url: &str) -> Result<()> {
	use url::Url;
	let host = Url::parse(url).ok().and_then(|u| u.host_str().map(|s| s.to_string()));
	if host.is_none() { return Ok(()); }
	let host = host.unwrap();
	if let Some(deny) = &settings.deny_domains { if deny.iter().any(|d| host.ends_with(d)) { return Err(anyhow!("Domain denied")); } }
	if let Some(allow) = &settings.allow_domains { if !allow.is_empty() && !allow.iter().any(|a| host.ends_with(a)) { return Err(anyhow!("Domain not in allowlist")); } }
	Ok(())
} 