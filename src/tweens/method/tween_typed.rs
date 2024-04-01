#[allow(unused_imports)]
use crate::*;

use super::tween_macros::{method_def, method_register, method_impl};
use crate::tweens::tween_value_macro::value_impl;
use crate::tweens::tween_base_macro::base_impl;

method_def!(i64, TweenMethod_i64);
method_register!(i64, TweenMethod_i64);
method_impl!(i64, TweenMethod_i64, i64, i64::_lerp);
value_impl!(i64, TweenMethod_i64);
base_impl!(TweenMethod_i64);

method_def!(f64, TweenMethod_f64);
method_register!(f64, TweenMethod_f64);
method_impl!(f64, TweenMethod_f64, f64, f64::_lerp);
value_impl!(f64, TweenMethod_f64);
base_impl!(TweenMethod_f64);

method_def!(GodotString, TweenMethod_GodotString);
method_register!(GodotString, TweenMethod_GodotString);
method_impl!(GodotString, TweenMethod_GodotString, String, GodotString::_lerp);
value_impl!(GodotString, TweenMethod_GodotString);
base_impl!(TweenMethod_GodotString);

method_def!(Color, TweenMethod_Color);
method_register!(Color, TweenMethod_Color);
method_impl!(Color, TweenMethod_Color, Color, Color::_lerp);
value_impl!(Color, TweenMethod_Color);
base_impl!(TweenMethod_Color);

method_def!(Vector2, TweenMethod_Vector2);
method_register!(Vector2, TweenMethod_Vector2);
method_impl!(Vector2, TweenMethod_Vector2, Vector2, Vector2::_lerp);
value_impl!(Vector2, TweenMethod_Vector2);
base_impl!(TweenMethod_Vector2);

method_def!(Vector3, TweenMethod_Vector3);
method_register!(Vector3, TweenMethod_Vector3);
method_impl!(Vector3, TweenMethod_Vector3, Vector3, Vector3::_lerp);
value_impl!(Vector3, TweenMethod_Vector3);
base_impl!(TweenMethod_Vector3);