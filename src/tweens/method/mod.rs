#[allow(unused_imports)]
use crate::*;
use enum_dispatch::enum_dispatch;

mod tween_macros;
mod tween_typed;
mod tween_variant;

pub use tween_typed::*;
pub use tween_variant::*;

#[enum_dispatch(Tick)]
#[derive(Debug)]
pub enum TweenMethod {
	i64(TweenMethod_i64),
	f64(TweenMethod_f64),
	String(TweenMethod_String),
	Color(TweenMethod_Color),
	Vector2(TweenMethod_Vector2),
	Vector3(TweenMethod_Vector3),
	Variant(TweenMethod_Variant),
}