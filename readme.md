# Kira

#### [crates.io](https://crates.io/crates/kira) | [docs](https://docs.rs/kira/) | [book](https://tesselode.github.io/kira/)

Kira is a backend-agnostic library to create expressive audio for games. It
provides parameters for smoothly adjusting properties of sounds, a flexible
mixer for applying effects to audio, and a clock system for precisely timing
audio events.

## Examples

### Playing a sound multiple times simultaneously

```rust
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::cpal::CpalBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

// Create an audio manager. This plays sounds and manages resources.
let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
let sound_data = StaticSoundData::from_file("sound.ogg", StaticSoundSettings::default())?;
manager.play(sound_data.clone())?;
// After a couple seconds...
manager.play(sound_data.clone())?;
// Cloning the sound data will not use any extra memory.
```

### Gradually speeding up a sound over time

```rust
use std::time::Duration;

use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::cpal::CpalBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	tween::Tween,
};

let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
let sound_data = StaticSoundData::from_file("sound.ogg", StaticSoundSettings::new())?;
let mut sound = manager.play(sound_data)?;
// Start smoothly adjusting the playback rate parameter.
sound.set_playback_rate(
	2.0,
	Tween {
		duration: Duration::from_secs(3),
		..Default::default()
	},
);
```

### Playing a sound with a low-pass filter applied

This makes the audio sound muffled.

```rust
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::cpal::CpalBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	track::{
		TrackBuilder,
		effect::filter::FilterBuilder,
	},
};

let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
// Create a mixer sub-track with a filter.
let track = manager.add_sub_track({
	let mut builder = TrackBuilder::new();
	builder.add_effect(FilterBuilder::new().cutoff(1000.0));
	builder
})?;
// Play the sound on the track.
let sound_data = StaticSoundData::from_file(
	"sound.ogg",
	StaticSoundSettings::new().track(&track),
)?;
manager.play(sound_data)?;
```

### Playing sounds in time with a musical beat

```rust
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::cpal::CpalBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	ClockSpeed,
};

const TEMPO: f64 = 120.0;

let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
// Create a clock that ticks 120 times per second. In this case,
// each tick is one musical beat. We can use a tick to represent any
// arbitrary amount of time.
let mut clock = manager.add_clock(ClockSpeed::TicksPerMinute(TEMPO))?;
// Play a sound 2 ticks (beats) from now.
let sound_data_1 = StaticSoundData::from_file(
	"sound1.ogg",
	StaticSoundSettings::new().start_time(clock.time() + 2),
)?;
manager.play(sound_data_1)?;
// Play a different sound 4 ticks (beats) from now.
let sound_data_2 = StaticSoundData::from_file(
	"sound2.ogg",
	StaticSoundSettings::new().start_time(clock.time() + 4),
)?;
manager.play(sound_data_2)?;
// Start the clock.
clock.start()?;
```

## Platform support

Kira is mainly meant for desktop platforms. Most testing has occurred on
Windows, but it has been used successfully used on Linux.

Kira can also be used in wasm environments with the following limitations:

- Static sounds cannot be loaded from files
- Streaming sounds are not supported because they make heavy use of threads

If you'd like to help improve wasm support, please reach out!

## Roadmap

Features I'd like to have:

- More mixer effects (EQ, compressor, better reverb, etc.)
- C API
- 3d audio

## Running the example crates

To run one of the examples, run this command from the root directory of the
repo:

```
cargo run -p [example_name]
```

Where `[example_name]` is the name of one of the folders in crates/examples
(besides the "assets" folder).

## Contributing

I'd love for other people to get involved with development! Since the library is
still in the early stages, I'm open to all kinds of input - bug reports, feature
requests, design critiques, etc. Feel free to open an issue or pull request!

## License

This project is licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE)
- MIT license (LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `kira` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
