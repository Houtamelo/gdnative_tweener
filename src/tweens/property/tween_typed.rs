#[allow(unused_imports)]
use crate::*;

use crate::tweens::tween_base_macro::base_impl;
use crate::tweens::tween_value_macro::value_impl;
use super::tween_macros::*;

property_def!(i64, TweenProperty_i64);
property_register!(i64, TweenProperty_i64);
property_impl!(i64, TweenProperty_i64, i64);
value_impl!(i64, TweenProperty_i64);
base_impl!(TweenProperty_i64);

property_def!(f64, TweenProperty_f64);
property_register!(f64, TweenProperty_f64);
property_impl!(f64, TweenProperty_f64, f64);
value_impl!(f64, TweenProperty_f64);
base_impl!(TweenProperty_f64);

property_def!(GodotString, TweenProperty_String);
property_register!(GodotString, TweenProperty_String);
property_impl!(GodotString, TweenProperty_String, String);
value_impl!(GodotString, TweenProperty_String);
base_impl!(TweenProperty_String);

property_def!(Color, TweenProperty_Color);
property_register!(Color, TweenProperty_Color);
property_impl!(Color, TweenProperty_Color, Color);
value_impl!(Color, TweenProperty_Color);
base_impl!(TweenProperty_Color);

property_def!(Vector2, TweenProperty_Vector2);
property_register!(Vector2, TweenProperty_Vector2);
property_impl!(Vector2, TweenProperty_Vector2, Vector2);
value_impl!(Vector2, TweenProperty_Vector2);
base_impl!(TweenProperty_Vector2);

property_def!(Vector3, TweenProperty_Vector3);
property_register!(Vector3, TweenProperty_Vector3);
property_impl!(Vector3, TweenProperty_Vector3, Vector3);
value_impl!(Vector3, TweenProperty_Vector3);
base_impl!(TweenProperty_Vector3);
