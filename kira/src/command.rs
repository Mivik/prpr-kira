use std::hash::Hash;

use crate::{
	arrangement::{Arrangement, ArrangementId},
	instance::{InstanceId, InstanceSettings},
	mixer::effect::Effect,
	mixer::effect::EffectId,
	mixer::{effect::EffectSettings, SubTrackId, TrackIndex, TrackSettings},
	parameter::{ParameterId, Tween},
	playable::Playable,
	sequence::{Sequence, SequenceId},
	sound::{Sound, SoundId},
	tempo::Tempo,
	value::Value,
};

#[derive(Debug, Clone)]
pub(crate) enum ResourceCommand {
	AddSound(SoundId, Sound),
	RemoveSound(SoundId),
	AddArrangement(ArrangementId, Arrangement),
	RemoveArrangement(ArrangementId),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum InstanceCommand {
	Play(InstanceId, Playable, Option<SequenceId>, InstanceSettings),
	SetInstanceVolume(InstanceId, Value<f64>),
	SetInstancePitch(InstanceId, Value<f64>),
	SetInstancePanning(InstanceId, Value<f64>),
	PauseInstance(InstanceId, Option<Tween>),
	ResumeInstance(InstanceId, Option<Tween>),
	StopInstance(InstanceId, Option<Tween>),
	PauseInstancesOf(Playable, Option<Tween>),
	ResumeInstancesOf(Playable, Option<Tween>),
	StopInstancesOf(Playable, Option<Tween>),
	PauseInstancesOfSequence(SequenceId, Option<Tween>),
	ResumeInstancesOfSequence(SequenceId, Option<Tween>),
	StopInstancesOfSequence(SequenceId, Option<Tween>),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum MetronomeCommand {
	SetMetronomeTempo(Value<Tempo>),
	StartMetronome,
	PauseMetronome,
	StopMetronome,
}

#[derive(Debug, Clone)]
pub(crate) enum SequenceCommand<CustomEvent: Copy + Eq + Hash> {
	StartSequence(SequenceId, Sequence<CustomEvent>),
	MuteSequence(SequenceId),
	UnmuteSequence(SequenceId),
	PauseSequence(SequenceId),
	ResumeSequence(SequenceId),
	StopSequence(SequenceId),
}

#[derive(Debug)]
pub(crate) enum MixerCommand {
	AddSubTrack(SubTrackId, TrackSettings),
	RemoveSubTrack(SubTrackId),
	AddEffect(TrackIndex, EffectId, Box<dyn Effect>, EffectSettings),
	RemoveEffect(EffectId),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ParameterCommand {
	AddParameter(ParameterId, f64),
	RemoveParameter(ParameterId),
	SetParameter(ParameterId, f64, Option<Tween>),
}

#[derive(Debug)]
pub(crate) enum Command<CustomEvent: Copy + Eq + Hash> {
	Resource(ResourceCommand),
	Instance(InstanceCommand),
	Metronome(MetronomeCommand),
	Sequence(SequenceCommand<CustomEvent>),
	Mixer(MixerCommand),
	Parameter(ParameterCommand),
	EmitCustomEvent(CustomEvent),
}

impl<CustomEvent: Copy + Eq + Hash> From<ResourceCommand> for Command<CustomEvent> {
	fn from(command: ResourceCommand) -> Self {
		Self::Resource(command)
	}
}

impl<CustomEvent: Copy + Eq + Hash> From<InstanceCommand> for Command<CustomEvent> {
	fn from(command: InstanceCommand) -> Self {
		Self::Instance(command)
	}
}

impl<CustomEvent: Copy + Eq + Hash> From<MetronomeCommand> for Command<CustomEvent> {
	fn from(command: MetronomeCommand) -> Self {
		Self::Metronome(command)
	}
}

impl<CustomEvent: Copy + Eq + Hash> From<SequenceCommand<CustomEvent>> for Command<CustomEvent> {
	fn from(command: SequenceCommand<CustomEvent>) -> Self {
		Self::Sequence(command)
	}
}

impl<CustomEvent: Copy + Eq + Hash> From<MixerCommand> for Command<CustomEvent> {
	fn from(command: MixerCommand) -> Self {
		Self::Mixer(command)
	}
}

impl<CustomEvent: Copy + Eq + Hash> From<ParameterCommand> for Command<CustomEvent> {
	fn from(command: ParameterCommand) -> Self {
		Self::Parameter(command)
	}
}
