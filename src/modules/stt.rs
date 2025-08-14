#![cfg(feature = "stt-vosk")]
use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use vosk::{Model, Recognizer};

pub fn transcribe_once(model_path: Option<&std::path::Path>) -> Result<String> {
	transcribe_for_secs(model_path, 4)
}

pub fn transcribe_for_secs(model_path: Option<&std::path::Path>, seconds: u64) -> Result<String> {
	let model_path = model_path.ok_or_else(|| anyhow!("STT model path not set in config"))?;
	if !Path::new(model_path).exists() { return Err(anyhow!("Model not found: {:?}", model_path)); }
	let model = Model::new(model_path.to_str().unwrap())?;
	let host = cpal::default_host();
	let device = host.default_input_device().ok_or_else(|| anyhow!("No default input device"))?;
	let config = device.default_input_config()?;
	let sample_rate = config.sample_rate().0;
	let mut recognizer = Recognizer::new(&model, sample_rate as f32)?;
	let transcript: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
	let transcript_clone = transcript.clone();

	let stream = match config.sample_format() {
		cpal::SampleFormat::F32 => device.build_input_stream(&config.into(), move |data: &[f32], _| {
			let bytes: &[u8] = bytemuck::cast_slice(data);
			let _ = recognizer.accept_waveform(bytes);
			let part = recognizer.partial_result();
			if !part.text().is_empty() { *transcript_clone.lock().unwrap() = part.text().to_string(); }
		}, err_fn)?,
		cpal::SampleFormat::I16 => device.build_input_stream(&config.into(), move |data: &[i16], _| {
			let bytes: &[u8] = bytemuck::cast_slice(data);
			let _ = recognizer.accept_waveform(bytes);
			let part = recognizer.partial_result();
			if !part.text().is_empty() { *transcript_clone.lock().unwrap() = part.text().to_string(); }
		}, err_fn)?,
		cpal::SampleFormat::U16 => device.build_input_stream(&config.into(), move |data: &[u16], _| {
			let bytes: &[u8] = bytemuck::cast_slice(data);
			let _ = recognizer.accept_waveform(bytes);
			let part = recognizer.partial_result();
			if !part.text().is_empty() { *transcript_clone.lock().unwrap() = part.text().to_string(); }
		}, err_fn)?,
		_ => return Err(anyhow!("Unsupported sample format")),
	};
	stream.play()?;
	std::thread::sleep(std::time::Duration::from_secs(seconds));
	drop(stream);
	let final_res = recognizer.final_result();
	let mut s = transcript.lock().unwrap().clone();
	if !final_res.text().is_empty() { s = final_res.text().to_string(); }
	Ok(s)
}

/// Listen until trailing silence exceeds `silence_ms` or `max_secs` reached, then return final transcript.
pub fn transcribe_until_silence(
	model_path: Option<&std::path::Path>,
	silence_ms: u64,
	max_secs: u64,
) -> Result<String> {
	let model_path = model_path.ok_or_else(|| anyhow!("STT model path not set in config"))?;
	if !Path::new(model_path).exists() { return Err(anyhow!("Model not found: {:?}", model_path)); }
	let model = Model::new(model_path.to_str().unwrap())?;
	let host = cpal::default_host();
	let device = host.default_input_device().ok_or_else(|| anyhow!("No default input device"))?;
	let config = device.default_input_config()?;
	let sample_rate = config.sample_rate().0 as usize;
	let mut recognizer = Recognizer::new(&model, sample_rate as f32)?;
	let transcript: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
	let transcript_clone = transcript.clone();
	let last_sound_time = Arc::new(Mutex::new(Instant::now()));
	let last_sound_time_cl = last_sound_time.clone();

	// Simple energy-based VAD
	let energy_threshold = 0.003; // adjust if needed
	let frame_ms = 20usize;
	let frame_len = sample_rate * frame_ms / 1000;

	let start_time = Instant::now();
	let stream = match config.sample_format() {
		cpal::SampleFormat::F32 => device.build_input_stream(&config.into(), move |data: &[f32], _| {
			let bytes: &[u8] = bytemuck::cast_slice(data);
			let _ = recognizer.accept_waveform(bytes);
			// VAD: compute mean abs over frames
			for chunk in data.chunks(frame_len.max(1)) {
				let energy = chunk.iter().map(|s| s.abs()).sum::<f32>() / (chunk.len().max(1) as f32);
				if energy > energy_threshold {
					*last_sound_time_cl.lock().unwrap() = Instant::now();
				}
			}
			let part = recognizer.partial_result();
			if !part.text().is_empty() { *transcript_clone.lock().unwrap() = part.text().to_string(); }
		}, err_fn)?,
		cpal::SampleFormat::I16 => device.build_input_stream(&config.into(), move |data: &[i16], _| {
			let bytes: &[u8] = bytemuck::cast_slice(data);
			let _ = recognizer.accept_waveform(bytes);
			for chunk in data.chunks(frame_len.max(1)) {
				let energy = chunk.iter().map(|s| (i16::abs(*s) as f32) / (i16::MAX as f32)).sum::<f32>() / (chunk.len().max(1) as f32);
				if energy > energy_threshold {
					*last_sound_time_cl.lock().unwrap() = Instant::now();
				}
			}
			let part = recognizer.partial_result();
			if !part.text().is_empty() { *transcript_clone.lock().unwrap() = part.text().to_string(); }
		}, err_fn)?,
		cpal::SampleFormat::U16 => device.build_input_stream(&config.into(), move |data: &[u16], _| {
			let bytes: &[u8] = bytemuck::cast_slice(data);
			let _ = recognizer.accept_waveform(bytes);
			for chunk in data.chunks(frame_len.max(1)) {
				let energy = chunk.iter().map(|s| ((*s as i32 - u16::MAX as i32/2).abs() as f32) / (i16::MAX as f32)).sum::<f32>() / (chunk.len().max(1) as f32);
				if energy > energy_threshold {
					*last_sound_time_cl.lock().unwrap() = Instant::now();
				}
			}
			let part = recognizer.partial_result();
			if !part.text().is_empty() { *transcript_clone.lock().unwrap() = part.text().to_string(); }
		}, err_fn)?,
		_ => return Err(anyhow!("Unsupported sample format")),
	};
	stream.play()?;
	loop {
		let since = Instant::now().duration_since(*last_sound_time.lock().unwrap());
		if since >= Duration::from_millis(silence_ms) { break; }
		if start_time.elapsed() >= Duration::from_secs(max_secs) { break; }
		std::thread::sleep(Duration::from_millis(20));
	}
	drop(stream);
	let final_res = recognizer.final_result();
	let mut s = transcript.lock().unwrap().clone();
	if !final_res.text().is_empty() { s = final_res.text().to_string(); }
	Ok(s)
}

fn err_fn(err: cpal::StreamError) {
	eprintln!("Audio stream error: {}", err);
} 