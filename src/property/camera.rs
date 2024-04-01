#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoCameraZoom: Camera2D {
		fn_name: do_zoom,
		val: Vector2,
		property: "zoom",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoCameraZoomX: Camera2D {
		fn_name: do_zoom_x,
		val: f64,
		property: "zoom:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraZoomY: Camera2D {
		fn_name: do_zoom_y,
		val: f64,
		property: "zoom:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraOffset: Camera2D {
		fn_name: do_offset,
		val: Vector2,
		property: "offset",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoCameraOffsetX: Camera2D {
		fn_name: do_offset_x,
		val: f64,
		property: "offset:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraOffsetY: Camera2D {
		fn_name: do_offset_y,
		val: f64,
		property: "offset:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraOffsetH: Camera2D {
		fn_name: do_offset_h,
		val: f64,
		property: "offset_h",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraOffsetV: Camera2D {
		fn_name: do_offset_v,
		val: f64,
		property: "offset_v",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraDragMarginLeft: Camera2D {
		fn_name: do_drag_margin_left,
		val: f64,
		property: "drag_margin_left",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraDragMarginRight: Camera2D {
		fn_name: do_drag_margin_right,
		val: f64,
		property: "drag_margin_right",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraDragMarginTop: Camera2D {
		fn_name: do_drag_margin_top,
		val: f64,
		property: "drag_margin_top",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCameraDragMarginBottom: Camera2D {
		fn_name: do_drag_margin_bottom,
		val: f64,
		property: "drag_margin_bottom",
		tween: TweenProperty_f64,
	}
}
