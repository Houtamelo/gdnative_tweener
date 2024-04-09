#[allow(unused_imports)]
use crate::*;

pub trait DoMethod<Val: _Lerp + FromVariant + ToVariant>: Sized {
	type Tween;

	fn do_method(&self,
	             method: impl Into<GodotString>,
	             start_val: Val,
	             end_val: Val,
	             duration: f64)
	             -> Self::Tween;
}

macro_rules! do_method_impl {
    ($val: ty, $tween: ty) => {
	    impl<T: Inherits<Object>> DoMethod<$val> for T {
			type Tween = $tween;
		
			fn do_method(&self,
						 method: impl Into<GodotString>,
						 start_val: $val,
			             end_val: $val,
			             duration: f64)
			             -> Self::Tween {
				let method = method.into();
				let tween = <$tween>::new(method, self, start_val, end_val, duration, AutoPlay(true));
				
				if let Some(node) = (unsafe { self.base().assume_safe_if_sane().map(|obj| obj.cast::<Node>()).flatten() }) {
					tween.bound_to(&node)
				} else {
					tween
				}
			}
		}
	};
}

do_method_impl!(i64, TweenMethod_i64);
do_method_impl!(f64, TweenMethod_f64);
do_method_impl!(GodotString, TweenMethod_String);
do_method_impl!(Color, TweenMethod_Color);
do_method_impl!(Vector2, TweenMethod_Vector2);
do_method_impl!(Vector3, TweenMethod_Vector3);

pub trait DoMethodVariant {
	fn do_method_var<Val: _Lerp + FromVariant + ToVariant + Clone>(
		&self,
		method: impl Into<GodotString>,
		start_val: Val,
		end_val: Val,
		duration: f64)
		-> TweenMethod_Variant;
}

impl<T: Inherits<Object>> DoMethodVariant for T {
	fn do_method_var<Val: _Lerp + FromVariant + ToVariant + Clone>(
		&self,
		method: impl Into<GodotString>,
		start_val: Val,
		end_val: Val,
		duration: f64)
		-> TweenMethod_Variant {
		let lerp_fn = |from: &Variant, to: &Variant, t: f64| -> Variant {
			let from = from.to::<Val>().unwrap();
			let to = to.to::<Val>().unwrap();
			Val::_lerp(&from, &to, t).to_variant()
		};

		let method = method.into();

		let tween = TweenMethod_Variant::new(
			method, self, start_val, end_val, duration, AutoPlay(true), lerp_fn);

		if let Some(node) = unsafe { self.base().assume_safe_if_sane().map(|obj| obj.cast::<Node>()).flatten() } {
			tween.bound_to(&node)
		} else {
			tween
		}
	}
}