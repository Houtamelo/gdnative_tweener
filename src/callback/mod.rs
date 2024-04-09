#[allow(unused_imports)]
use crate::*;

pub trait DoDelayedMethod {
	fn do_delayed_method(
		&self,
		fn_name: impl Into<GodotString>,
		args: Vec<Variant>,
		delay: f64)
		-> TweenCallback;
}

impl<T: Inherits<Object>> DoDelayedMethod for T {
	fn do_delayed_method(
		&self,
		fn_name: impl Into<GodotString>,
		args: Vec<Variant>,
		delay: f64)
		-> TweenCallback {

		let tween = TweenCallback::new_method(
			self, fn_name, args, delay, AutoPlay(true));

		if let Some(node) = unsafe { self.base().assume_safe_if_sane().map(|obj| obj.cast::<Node>()).flatten() } {
			tween.bound_to(&node)
		} else {
			tween
		}
	}
}

pub fn do_delayed_call(delay: f64, f: impl Fn() + 'static) -> TweenCallback {
	TweenCallback::new_closure(f, delay, AutoPlay(true))
}