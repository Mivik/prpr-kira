//! Playable chunks of audio that are loaded into memory all at once.

mod data;
mod handle;
mod settings;
mod sound;

pub use data::*;
pub use handle::*;
pub use settings::*;
pub use sound::PlaybackState;

use crate::{tween::Tween, PlaybackRate, Volume};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
	SetVolume(Volume, Tween),
	SetPlaybackRate(PlaybackRate, Tween),
	SetPanning(f64, Tween),
	Pause(Tween),
	Resume(Tween),
	Stop(Tween),
	SeekBy(f64),
	SeekTo(f64),
}
