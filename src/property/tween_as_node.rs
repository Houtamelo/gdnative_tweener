#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoTweenPlaybackSpeed: Tween {
		fn_name: do_playback_speed,
		val: f64,
		property: "playback_speed",
		tween: TweenProperty_f64,
	}
}