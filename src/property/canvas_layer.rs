#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoCanvasLayerOffset: CanvasLayer {
		fn_name: do_offset,
		val: Vector2,
		property: "offset",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerOffsetX: CanvasLayer {
		fn_name: do_offset_x,
		val: f64,
		property: "offset:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerOffsetY: CanvasLayer {
		fn_name: do_offset_y,
		val: f64,
		property: "offset:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerRotationDegrees: CanvasLayer {
		fn_name: do_rotation_degrees,
		val: f64,
		property: "rotation_degrees",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerRotationRadians: CanvasLayer {
		fn_name: do_rotation_radians,
		val: f64,
		property: "rotation",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerScale: CanvasLayer {
		fn_name: do_scale,
		val: Vector2,
		property: "scale",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerScaleX: CanvasLayer {
		fn_name: do_scale_x,
		val: f64,
		property: "scale:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerScaleY: CanvasLayer {
		fn_name: do_scale_y,
		val: f64,
		property: "scale:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasLayerFollowViewportScale: CanvasLayer {
		fn_name: do_follow_viewport_scale,
		val: f64,
		property: "follow_viewport_scale",
		tween: TweenProperty_f64,
	}
}