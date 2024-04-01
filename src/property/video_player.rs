#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoVideoPlayerVolumeDb: VideoPlayer {
		fn_name: do_volume_db,
		val: f64,
		property: "volume_db",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoVideoPlayerVolume: VideoPlayer {
		fn_name: do_volume,
		val: f64,
		property: "volume",
		tween: TweenProperty_f64,
	}
}
