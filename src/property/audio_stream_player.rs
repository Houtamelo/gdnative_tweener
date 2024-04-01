#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoAudioStreamPlayerVolumeDb: AudioStreamPlayer {
		fn_name: do_volume_db,
		val: f64,
		property: "volume_db",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoAudioStreamPlayerPitchScale: AudioStreamPlayer {
		fn_name: do_pitch_scale,
		val: f64,
		property: "pitch_scale",
		tween: TweenProperty_f64,
	}
}