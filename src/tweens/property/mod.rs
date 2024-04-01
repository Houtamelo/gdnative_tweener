#[allow(unused_imports)]
use crate::*;
use enum_dispatch::enum_dispatch;

mod tween_typed;
mod tween_variant;
mod tween_macros;

#[allow(unused_imports)] pub use tween_typed::*;
#[allow(unused_imports)] pub use tween_variant::*;

#[enum_dispatch(Tick)]
#[derive(Debug)]
pub enum TweenProperty {
	i64(TweenProperty_i64),
	f64(TweenProperty_f64),
	String(TweenProperty_String),
	Color(TweenProperty_Color),
	Vector2(TweenProperty_Vector2),
	Vector3(TweenProperty_Vector3),
	Variant(TweenProperty_Variant),
}

