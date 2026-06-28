//! Background music — loops dungeon tracks on a worker thread.
//!
//! Drop `.mp3` files in `assets/music/`; the game discovers them automatically.

use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use rodio::{Decoder, OutputStream, Sink, Source};
use serde::{Deserialize, Serialize};

use super::state::GameState;

/// How background music is chosen between sessions and quests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MusicMode {
    /// Always play the saved track until the player picks another or mutes.
    #[default]
    Fixed,
    /// Rotate to the next track each time the player enters a quest from the map.
    CycleOnQuest,
}

/// One MP3 discovered under `assets/music/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredTrack {
    pub stem: String,
    pub label: String,
    pub file_name: String,
}

enum AudioCommand {
    Play { stem: String },
    Mute,
    Stop,
}

/// Handle to the background music thread.
pub struct MusicHandle {
    tx: Option<Sender<AudioCommand>>,
    _thread: Option<JoinHandle<()>>,
}

impl MusicHandle {
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel();
        match thread::Builder::new()
            .name("rust-quest-music".into())
            .spawn(move || audio_worker(rx))
        {
            Ok(handle) => Self {
                tx: Some(tx),
                _thread: Some(handle),
            },
            Err(_) => Self::disabled(),
        }
    }

    pub fn disabled() -> Self {
        Self {
            tx: None,
            _thread: None,
        }
    }

    pub fn is_available(&self) -> bool {
        self.tx.is_some()
    }

    pub fn play_stem(&self, stem: &str) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(AudioCommand::Play {
                stem: stem.to_string(),
            });
        }
    }

    pub fn set_muted(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(AudioCommand::Mute);
        }
    }

    /// Start playback on game launch from saved settings (does not advance cycle).
    pub fn launch_music(state: &mut GameState, music: &MusicHandle) {
        ensure_default_track(state);
        if state.music_muted {
            music.set_muted();
            return;
        }
        let Some(stem) = stem_for_session(state) else {
            return;
        };
        state.music_playing_stem = stem.clone();
        if state.music_mode == MusicMode::CycleOnQuest && state.music_last_stem.is_empty() {
            state.music_last_stem = stem.clone();
        }
        music.play_stem(&stem);
    }

    /// Advance cycle when entering a quest from the map.
    pub fn cycle_on_quest(state: &mut GameState, music: &MusicHandle) {
        if state.music_muted || state.music_mode != MusicMode::CycleOnQuest {
            return;
        }
        let Some(stem) = next_in_cycle(&state.music_last_stem) else {
            return;
        };
        state.music_last_stem = stem.clone();
        state.music_playing_stem = stem.clone();
        music.play_stem(&stem);
    }

    /// Resume playback after unmute (does not advance cycle).
    pub fn apply_session_playback(state: &GameState, music: &MusicHandle) {
        if state.music_muted {
            music.set_muted();
            return;
        }
        let Some(stem) = stem_for_session(state) else {
            return;
        };
        music.play_stem(&stem);
    }

    fn stop(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(AudioCommand::Stop);
        }
    }
}

impl Drop for MusicHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn music_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/music")
}

/// Scan `assets/music/` for `.mp3` files (sorted by filename).
pub fn discover_tracks() -> Vec<DiscoveredTrack> {
    let mut tracks = Vec::new();
    let Ok(entries) = fs::read_dir(music_dir()) else {
        return tracks;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("mp3") {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let stem = Path::new(file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(file_name)
            .to_string();
        tracks.push(DiscoveredTrack {
            label: label_from_stem(&stem),
            stem,
            file_name: file_name.to_string(),
        });
    }
    tracks.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    tracks
}

pub fn label_from_stem(stem: &str) -> String {
    stem.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn default_track_stem() -> Option<String> {
    discover_tracks().first().map(|t| t.stem.clone())
}

pub fn ensure_default_track(state: &mut GameState) {
    if state.music_mode == MusicMode::Fixed && state.music_track.is_empty() {
        if let Some(stem) = default_track_stem() {
            state.music_track = stem;
        }
    }
}

/// Next track in sorted rotation after `last_stem` (wraps around).
pub fn next_in_cycle(last_stem: &str) -> Option<String> {
    let tracks = discover_tracks();
    if tracks.is_empty() {
        return None;
    }
    if last_stem.is_empty() {
        return Some(tracks[0].stem.clone());
    }
    let idx = tracks
        .iter()
        .position(|t| t.stem == last_stem)
        .unwrap_or(0);
    Some(tracks[(idx + 1) % tracks.len()].stem.clone())
}

fn stem_for_session(state: &GameState) -> Option<String> {
    match state.music_mode {
        MusicMode::Fixed => {
            let tracks = discover_tracks();
            if tracks.iter().any(|t| t.stem == state.music_track) {
                Some(state.music_track.clone())
            } else {
                default_track_stem()
            }
        }
        MusicMode::CycleOnQuest => {
            if state.music_last_stem.is_empty() {
                default_track_stem()
            } else if discover_tracks()
                .iter()
                .any(|t| t.stem == state.music_last_stem)
            {
                Some(state.music_last_stem.clone())
            } else {
                default_track_stem()
            }
        }
    }
}

pub fn status_label(state: &GameState) -> String {
    if state.music_muted {
        return "🔇 muted".to_string();
    }
    match state.music_mode {
        MusicMode::Fixed => format!("🔊 {}", label_from_stem(&state.music_track)),
        MusicMode::CycleOnQuest => {
            if state.music_playing_stem.is_empty() {
                "🔀 cycle per quest".to_string()
            } else {
                format!(
                    "🔀 cycle per quest — now: {}",
                    label_from_stem(&state.music_playing_stem)
                )
            }
        }
    }
}

fn open_track(stem: &str) -> Option<Decoder<BufReader<File>>> {
    let path = music_dir().join(format!("{stem}.mp3"));
    let file = File::open(path).ok()?;
    Decoder::new(BufReader::new(file)).ok()
}

fn audio_worker(rx: Receiver<AudioCommand>) {
    let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
        return;
    };
    let Ok(sink) = Sink::try_new(&stream_handle) else {
        return;
    };

    const VOLUME: f32 = 0.35;

    for cmd in rx {
        match cmd {
            AudioCommand::Play { stem } => {
                sink.stop();
                sink.clear();
                if let Some(source) = open_track(&stem) {
                    sink.append(source.repeat_infinite());
                    sink.set_volume(VOLUME);
                    sink.play();
                }
            }
            AudioCommand::Mute => {
                sink.stop();
                sink.clear();
                sink.set_volume(0.0);
            }
            AudioCommand::Stop => {
                sink.stop();
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_bundled_tracks() {
        let tracks = discover_tracks();
        assert!(
            tracks.len() >= 3,
            "expected at least three mp3 files in assets/music"
        );
        for track in &tracks {
            assert!(music_dir().join(&track.file_name).is_file());
        }
    }

    #[test]
    fn cycle_rotates_through_all_tracks() {
        let tracks = discover_tracks();
        assert!(tracks.len() >= 2);
        let first = tracks[0].stem.clone();
        let second = next_in_cycle(&first).unwrap();
        assert_ne!(first, second);
        let mut stem = first.clone();
        for _ in 0..tracks.len() {
            stem = next_in_cycle(&stem).unwrap();
        }
        assert_eq!(stem, first);
    }

    #[test]
    fn fixed_mode_uses_saved_track() {
        let mut state = GameState::default();
        state.music_mode = MusicMode::Fixed;
        state.music_track = "mossy_gate".into();
        assert_eq!(
            stem_for_session(&state).as_deref(),
            Some("mossy_gate")
        );
    }
}
