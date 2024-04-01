#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoCanvasModulateColor: CanvasModulate {
		fn_name: do_canvas_color,
		val: Color,
		property: "color",
		tween: TweenProperty_Color,
	}
}

do_full_trait! {
	pub trait DoCanvasModulateColorR: CanvasModulate {
		fn_name: do_canvas_color_r,
		val: f64,
		property: "color:r",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasModulateColorG: CanvasModulate {
		fn_name: do_canvas_color_g,
		val: f64,
		property: "color:g",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasModulateColorB: CanvasModulate {
		fn_name: do_canvas_color_b,
		val: f64,
		property: "color:b",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasModulateColorA: CanvasModulate {
		fn_name: do_canvas_color_a,
		val: f64,
		property: "color:a",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasModulateFade: CanvasModulate {
		fn_name: do_canvas_fade,
		val: f64,
		property: "color:a",
		tween: TweenProperty_f64,
	}
}
