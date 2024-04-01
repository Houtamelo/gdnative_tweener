#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoNode2DGlobalMove: Node2D {
		fn_name: do_global_move,
		val: Vector2,
		property: "global_position",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoNode2DGlobalMoveX: Node2D {
		fn_name: do_global_move_x,
		val: f64,
		property: "global_position:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DGlobalMoveY: Node2D {
		fn_name: do_global_move_y,
		val: f64,
		property: "global_position:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DGlobalRotationDeg: Node2D {
		fn_name: do_global_rotation,
		val: f64,
		property: "global_rotation_degrees",
		tween: TweenProperty_f64,
	}
}
	
do_full_trait! {
	pub trait DoNode2DGlobalRotationRad: Node2D {
		fn_name: do_global_rotation_rad,
		val: f64,
		property: "global_rotation",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DGlobalScale: Node2D {
		fn_name: do_global_scale,
		val: Vector2,
		property: "global_scale",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoNode2DGlobalScaleX: Node2D {
		fn_name: do_global_scale_x,
		val: f64,
		property: "global_scale:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoNode2DGlobalScaleY: Node2D {
		fn_name: do_global_scale_y,
		val: f64,
		property: "global_scale:y",
		tween: TweenProperty_f64,
	}
}
