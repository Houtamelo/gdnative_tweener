#[allow(unused_imports)]
use crate::*;
use crate::property::macros::do_full_trait;

do_full_trait! {
	pub trait DoCanvasItemSelfColor: CanvasItem {
		fn_name: do_self_color,
		val: Color,
		property: "self_modulate",
		tween: TweenProperty_Color,
	}
}

do_full_trait! {
	pub trait DoCanvasItemSelfColorR: CanvasItem {
		fn_name: do_self_color_r,
		val: f64,
		property: "self_modulate:r",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemSelfColorG: CanvasItem {
		fn_name: do_self_color_g,
		val: f64,
		property: "self_modulate:g",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemSelfColorB: CanvasItem {
		fn_name: do_self_color_b,
		val: f64,
		property: "self_modulate:b",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemSelfColorA: CanvasItem {
		fn_name: do_self_color_a,
		val: f64,
		property: "self_modulate:a",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemSelfFade: CanvasItem {
		fn_name: do_self_fade,
		val: f64,
		property: "self_modulate:a",
		tween: TweenProperty_f64,
	}
}

// bellow just needs to compile
#[allow(unused)]
#[cfg(test)]
mod tests {
	use gdnative::api::Particles2D;

	use super::*;

	unsafe fn please_compile(node_direct: &Sprite, node_ref: Ref<CanvasItem>, node_tref: TRef<Particles2D>) -> Result<()> {
		node_direct.do_self_color_r(0., 1., 5.0)?;
		node_ref.do_self_color_r(0., 1., 5.0)?;
		node_tref.do_self_color_r(0., 1., 5.0)?;

		node_direct.do_self_color_g(0., 1., 5.0)?;
		node_ref.do_self_color_g(0., 1., 5.0)?;
		node_tref.do_self_color_g(0., 1., 5.0)?;

		node_direct.do_self_color_b(0., 1., 5.0)?;
		node_ref.do_self_color_b(0., 1., 5.0)?;
		node_tref.do_self_color_b(0., 1., 5.0)?;

		node_direct.do_self_color_a(0., 1., 5.0)?;
		node_ref.do_self_color_a(0., 1., 5.0)?;
		node_tref.do_self_color_a(0., 1., 5.0)?;

		node_direct.do_self_fade(0., 1., 5.0)?;
		node_ref.do_self_fade(0., 1., 5.0)?;
		node_tref.do_self_fade(0., 1., 5.0)?;

		let color = Color::from_rgb(1., 1., 1.);
		node_direct.do_self_color(color, color, 5.0)?;
		node_ref.do_self_color(color, color, 5.0)?;
		node_tref.do_self_color(color, color, 5.0)?;

		Ok(())
	}
}