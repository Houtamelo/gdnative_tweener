#[allow(unused_imports)]
use crate::*;

pub trait DoCallback {
	fn do_callback(
		&self,
		fn_name: impl Into<GodotString>,
		args: Vec<Variant>,
		duration: f64)
		-> TweenCallback;
}

impl<T: Inherits<Object>> DoCallback for T {
	fn do_callback(
		&self,
		fn_name: impl Into<GodotString>,
		args: Vec<Variant>,
		duration: f64)
		-> TweenCallback {

		let tween = TweenCallback::new(
			fn_name, self, args, duration, AutoPlay(true));

		if let Some(node) = unsafe { self.base().assume_safe_if_sane().map(|obj| obj.cast::<Node>()).flatten() } {
			tween.bound_to(&node)
		} else {
			tween
		}
	}
}