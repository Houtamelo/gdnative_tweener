#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoControlAnchorBottom: Control {
		fn_name: do_anchor_bottom,
		val: f64,
		property: "anchor_bottom",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlAnchorLeft: Control {
		fn_name: do_anchor_left,
		val: f64,
		property: "anchor_left",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlAnchorRight: Control {
		fn_name: do_anchor_right,
		val: f64,
		property: "anchor_right",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlAnchorTop: Control {
		fn_name: do_anchor_top,
		val: f64,
		property: "anchor_top",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlMarginBottom: Control {
		fn_name: do_margin_bottom,
		val: f64,
		property: "margin_bottom",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlMarginLeft: Control {
		fn_name: do_margin_left,
		val: f64,
		property: "margin_left",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlMarginRight: Control {
		fn_name: do_margin_right,
		val: f64,
		property: "margin_right",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlMarginTop: Control {
		fn_name: do_margin_top,
		val: f64,
		property: "margin_top",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectMinSize: Control {
		fn_name: do_rect_min_size,
		val: Vector2,
		property: "rect_min_size",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoControlRectMinSizeX: Control {
		fn_name: do_rect_min_size_x,
		val: f64,
		property: "rect_min_size:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectMinSizeY: Control {
		fn_name: do_rect_min_size_y,
		val: f64,
		property: "rect_min_size:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectSize: Control {
		fn_name: do_rect_size,
		val: Vector2,
		property: "rect_size",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoControlRectSizeX: Control {
		fn_name: do_rect_size_x,
		val: f64,
		property: "rect_size:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectSizeY: Control {
		fn_name: do_rect_size_y,
		val: f64,
		property: "rect_size:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectGlobalPosition: Control {
		fn_name: do_rect_global_position,
		val: Vector2,
		property: "rect_global_position",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoControlRectGlobalPositionX: Control {
		fn_name: do_rect_global_position_x,
		val: f64,
		property: "rect_global_position:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectGlobalPositionY: Control {
		fn_name: do_rect_global_position_y,
		val: f64,
		property: "rect_global_position:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectPosition: Control {
		fn_name: do_rect_position,
		val: Vector2,
		property: "rect_position",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoControlRectPositionX: Control {
		fn_name: do_rect_position_x,
		val: f64,
		property: "rect_position:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectPositionY: Control {
		fn_name: do_rect_position_y,
		val: f64,
		property: "rect_position:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectScale: Control {
		fn_name: do_rect_scale,
		val: Vector2,
		property: "rect_scale",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoControlRectScaleX: Control {
		fn_name: do_rect_scale_x,
		val: f64,
		property: "rect_scale:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectScaleY: Control {
		fn_name: do_rect_scale_y,
		val: f64,
		property: "rect_scale:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectPivotOffset: Control {
		fn_name: do_rect_pivot_offset,
		val: Vector2,
		property: "rect_pivot_offset",
		tween: TweenProperty_Vector2,
	}
}

do_full_trait! {
	pub trait DoControlRectPivotOffsetX: Control {
		fn_name: do_rect_pivot_offset_x,
		val: f64,
		property: "rect_pivot_offset:x",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectPivotOffsetY: Control {
		fn_name: do_rect_pivot_offset_y,
		val: f64,
		property: "rect_pivot_offset:y",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoControlRectRotationDeg: Control {
		fn_name: do_rect_rotation_deg,
		val: f64,
		property: "rect_rotation", // Godot does not have radians for rect rotation
		tween: TweenProperty_f64,
	}
}



