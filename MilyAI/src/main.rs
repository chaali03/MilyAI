mod modules;
mod agent;
mod memory;
mod settings;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "milyai", version, about = "MilyAI: Modular AI Assistant", long_about = None)]
struct Cli {
	/// Path to config file (YAML). Defaults to $APPDATA/milyai/config.yaml or OS equivalent
	#[arg(long)]
	config: Option<String>,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// Run interactive assistant (text REPL)
	Run,
	/// Say a message via TTS (requires --features tts)
	#[cfg(feature = "tts")]
	Say { text: String },
	/// Listen on mic and transcribe one utterance (requires --features stt-vosk)
	#[cfg(feature = "stt-vosk")]
	Listen,
	/// Capture one camera frame to file (requires --features camera)
	#[cfg(feature = "camera")]
	Snapshot { output: String },
	/// Voice mode with wake word (requires --features tts,stt-vosk)
	#[cfg(all(feature = "stt-vosk", feature = "tts"))]
	Voice,
}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let cli = Cli::parse();
	let settings = settings::load(cli.config.as_deref())?;

	match cli.command.unwrap_or(Commands::Run) {
		Commands::Run => run_repl(settings).await?,
		#[cfg(feature = "tts")]
		Commands::Say { text } => modules::tts::speak(&text)?,
		#[cfg(feature = "stt-vosk")]
		Commands::Listen => {
			let transcript = modules::stt::transcribe_once(settings.stt_model_path.as_deref())?;
			println!("{}", transcript);
		}
		#[cfg(feature = "camera")]
		Commands::Snapshot { output } => {
			modules::camera::snapshot(&output)?;
			println!("Saved snapshot to {}", output);
		}
		#[cfg(all(feature = "stt-vosk", feature = "tts"))]
		Commands::Voice => run_voice(settings).await?,
	}

	Ok(())
}

async fn run_repl(settings: settings::Settings) -> Result<()> {
	use std::io::{self, Write};
	let mut agent = agent::Agent::new(settings)?;
	println!("MilyAI ready. Type 'exit' to quit.");
	loop {
		print!("> ");
		io::stdout().flush()?;
		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		let msg = input.trim();
		if msg.eq_ignore_ascii_case("exit") || msg.eq_ignore_ascii_case("quit") {
			break;
		}
		let response = agent.respond(msg).await?;
		println!("{}", response);
		#[cfg(feature = "tts")]
		{
			let _ = modules::tts::speak(&response);
		}
	}
	Ok(())
}

#[cfg(all(feature = "stt-vosk", feature = "tts"))]
async fn run_voice(settings: settings::Settings) -> Result<()> {
	use std::time::Duration;
	let mut agent = agent::Agent::new(settings.clone())?;
	println!("Voice mode. Say 'Milly' to wake me. Ctrl+C to exit.");
	loop {
		if let Some(query) = modules::voice::listen_for_wake_and_query(settings.stt_model_path.as_deref())? {
			if query.trim().is_empty() { continue; }
			let reply = agent.respond(&query).await?;
			println!("You: {}\nMily: {}", query, reply);
			let _ = modules::tts::speak(&reply);
			// small cooldown to avoid re-triggering immediately
			tokio::time::sleep(Duration::from_millis(800)).await;
		}
	}
} 