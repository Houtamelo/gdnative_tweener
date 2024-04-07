#[allow(unused_imports)]
use crate::*;
use super::macros::do_full_trait;

do_full_trait! {
	pub trait DoCanvasItemColor: CanvasItem {
		fn_name: do_color,
		val: Color,
		property: "modulate",
		tween: TweenProperty_Color,
	}
}

do_full_trait! {
	pub trait DoCanvasItemColorR: CanvasItem {
		fn_name: do_color_r,
		val: f64,
		property: "modulate:r",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemColorG: CanvasItem {
		fn_name: do_color_g,
		val: f64,
		property: "modulate:g",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemColorB: CanvasItem {
		fn_name: do_color_b,
		val: f64,
		property: "modulate:b",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemColorA: CanvasItem {
		fn_name: do_color_a,
		val: f64,
		property: "modulate:a",
		tween: TweenProperty_f64,
	}
}

do_full_trait! {
	pub trait DoCanvasItemFade: CanvasItem {
		fn_name: do_fade,
		val: f64,
		property: "modulate:a",
		tween: TweenProperty_f64,
	}
}

// bellow just needs to compile
#[allow(unused)]
#[cfg(test)]
mod tests {
	use super::*;

	unsafe fn test(node_direct: &Sprite, node_ref: Ref<CanvasItem>, node_tref: TRef<Particles2D>) -> Result<()> {
		node_direct.do_color_r( 1., 5.)?;
		node_ref.do_color_r(1., 5.)?;
		node_tref.do_color_r(1., 5.)?;

		node_direct.do_color_g(1., 5.)?;
		node_ref.do_color_g(1., 5.)?;
		node_tref.do_color_g(1., 5.)?;

		node_direct.do_color_b(1., 5.)?;
		node_ref.do_color_b(1., 5.)?;
		node_tref.do_color_b(1., 5.)?;

		node_direct.do_color_a( 1., 5.)?;
		node_ref.do_color_a(1., 5.)?;
		node_tref.do_color_a(1., 5.)?;

		node_direct.do_fade(1., 5.)?;
		node_ref.do_fade(1., 5.)?;
		node_tref.do_fade(1., 5.)?;

		let color = Color::from_rgb(1., 1., 1.);
		node_direct.do_color(color, 5.)?;
		node_ref.do_color(color, 5.)?;
		node_tref.do_color(color, 5.)?;
		
		Ok(())
	}

	#[derive(NativeClass)]
	#[inherit(Node2D)]
	struct Test {}

	#[methods]
	impl Test {
		fn new(_owner: &Node2D) -> Self {
			_owner.do_color_r(1., 2.0);

			Self {}
		}

		#[method]
		fn _ready(&self, #[base] owner: &Node2D) {
			owner.do_color_r(1., 2.0);
		}
	}
}