#[allow(unused_imports)]
use crate::*;
use crate::property::macros::do_full_trait;

do_full_trait! {
	pub trait DoPathFollowOffset: PathFollow2D {
		fn_name: do_offset,
		val: f64,
		property: "offset",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoPathFollowUnitOffset: PathFollow2D {
		fn_name: do_unit_offset,
		val: f64,
		property: "unit_offset",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoPathFollowHOffset: PathFollow2D {
		fn_name: do_h_offset,
		val: f64,
		property: "h_offset",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoPathFollowVOffset: PathFollow2D {
		fn_name: do_v_offset,
		val: f64,
		property: "v_offset",
		tween: TweenProperty_f64,
	}
}
