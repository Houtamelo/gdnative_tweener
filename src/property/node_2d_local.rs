#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoNode2DMove: Node2D {
		fn_name: do_move,
		val: Vector2,
		property: "position",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoNode2DMoveX: Node2D {
		fn_name: do_move_x,
		val: f64,
		property: "position:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DMoveY: Node2D {
		fn_name: do_move_y,
		val: f64,
		property: "position:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DRotationDeg: Node2D {
		fn_name: do_rotation,
		val: f64,
		property: "rotation_degrees",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DRotationRad: Node2D {
		fn_name: do_rotation_rad,
		val: f64,
		property: "rotation",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DScale: Node2D {
		fn_name: do_scale,
		val: Vector2,
		property: "scale",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoNode2DScaleX: Node2D {
		fn_name: do_scale_x,
		val: f64,
		property: "scale:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DScaleY: Node2D {
		fn_name: do_scale_y,
		val: f64,
		property: "scale:y",
		tween: TweenProperty_f64,
	}
}