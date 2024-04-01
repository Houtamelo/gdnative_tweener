#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoRangeValue: Range {
		fn_name: do_value,
		val: f64,
		property: "value",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoRangeRatio: Range {
		fn_name: do_ratio,
		val: f64,
		property: "ratio",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoTextureProgressTintUnder: TextureProgress {
		fn_name: do_tint_under,
		val: Color,
		property: "tint_under",
		tween: TweenProperty_Color,
	}
}

do_full_trait! {
	pub trait DoTextureProgressTintOver: TextureProgress {
		fn_name: do_tint_over,
		val: Color,
		property: "tint_over",
		tween: TweenProperty_Color,
	}
}

do_full_trait! {
	pub trait DoTextureProgressTintProgress: TextureProgress {
		fn_name: do_tint_progress,
		val: Color,
		property: "tint_progress",
		tween: TweenProperty_Color,
	}
}

do_full_trait! {
	pub trait DoTextureProgressRadialInitialAngle: TextureProgress {
		fn_name: do_radial_initial_angle,
		val: f64,
		property: "radial_initial_angle",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoTextureProgressRadialFillDegrees: TextureProgress {
		fn_name: do_radial_fill_degrees,
		val: f64,
		property: "radial_fill_degrees",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoTextureProgressRadialCenterOffset: TextureProgress {
		fn_name: do_radial_center_offset,
		val: Vector2,
		property: "radial_center_offset",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoTextureProgressRadialCenterOffsetX: TextureProgress {
		fn_name: do_radial_center_offset_x,
		val: f64,
		property: "radial_center_offset:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoTextureProgressRadialCenterOffsetY: TextureProgress {
		fn_name: do_radial_center_offset_y,
		val: f64,
		property: "radial_center_offset:y",
		tween: TweenProperty_f64,
	}
}

