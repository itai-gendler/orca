//! Disposable audio for manually checking ReplayGain through the player.
use lofty::{
    config::WriteOptions,
    prelude::*,
    tag::{ItemKey, Tag, TagType},
};
use std::{f32::consts::TAU, fs, path::PathBuf};

const SAMPLE_RATE: u32 = 44_100;
const SECONDS: u32 = 6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/replaygain-fixtures");
    fs::create_dir_all(&output)?;
    for (filename, amplitude, gain) in [
        ("01-loud-tagged.wav", 0.5_f32, Some("-6.00 dB")),
        ("02-quiet-tagged.wav", 0.125_f32, Some("+6.00 dB")),
        ("03-reference-untagged.wav", 0.25_f32, None),
    ] {
        let path = output.join(filename);
        let sample_count = SAMPLE_RATE * SECONDS;
        let data_len = sample_count * 2;
        let mut wav = Vec::with_capacity(44 + data_len as usize);
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(36 + data_len).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16_u32.to_le_bytes());
        wav.extend_from_slice(&1_u16.to_le_bytes());
        wav.extend_from_slice(&1_u16.to_le_bytes());
        wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
        wav.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
        wav.extend_from_slice(&2_u16.to_le_bytes());
        wav.extend_from_slice(&16_u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_len.to_le_bytes());
        for index in 0..sample_count {
            let t = index as f32 / SAMPLE_RATE as f32;
            let fade =
                ((t / 0.05).min(1.0) * ((SECONDS as f32 - t) / 0.05).min(1.0)).clamp(0.0, 1.0);
            let sample = (amplitude * fade * (TAU * 440.0 * t).sin() * i16::MAX as f32) as i16;
            wav.extend_from_slice(&sample.to_le_bytes());
        }
        fs::write(&path, wav)?;
        if let Some(gain) = gain {
            let mut tag = Tag::new(TagType::Id3v2);
            tag.insert_text(
                ItemKey::TrackTitle,
                filename.trim_end_matches(".wav").into(),
            );
            tag.insert_text(ItemKey::TrackArtist, "Orca test fixture".into());
            tag.insert_text(ItemKey::ReplayGainTrackGain, gain.into());
            tag.insert_text(ItemKey::ReplayGainTrackPeak, amplitude.to_string());
            tag.save_to_path(&path, WriteOptions::default())?;
        }
    }
    println!("Generated disposable WAV files in {}", output.display());
    println!("With ReplayGain off: loud > reference > quiet.");
    println!("With ReplayGain on: all three should sound about equally loud.");
    Ok(())
}
