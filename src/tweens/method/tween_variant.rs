#[allow(unused_imports)]
use crate::*;
use crate::internal_prelude::tween_base_macro::base_impl;
use crate::internal_prelude::tween_value_macro::value_impl;
use crate::tweens::method::tween_macros::method_impl;

#[derive(Debug, Clone)]
pub struct TweenMethod_Variant {
	pub method: GodotString,
	pub target: Ref<Object>,
	pub bound_node: Option<Ref<Node>>,
	state: State,
	pub delay: f64,
	pub duration: f64,
	pub ease: Ease,
	pub speed_scale: f64,
	pub elapsed_time: f64,
	pub cycle_count: u32,
	pub pause_mode: TweenPauseMode,
	pub process_mode: TweenProcessMode,
	pub loop_mode: LoopMode,
	pub start: Variant,
	pub end: Variant,
	pub do_on_finish: Vec<Callback>,
	lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
}

impl TweenMethod_Variant {
	pub fn new<T: _Lerp + FromVariant + ToVariant + Clone>(
		method: impl Into<GodotString>,
		target: &impl Inherits<Object>,
		start: T,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant)
		-> Self {
		Self {
			method: method.into(),
			target: unsafe { target.base() },
			bound_node: None,
			state: match auto_play.0 {
				true => State::Playing,
				false => State::Paused,
			},
			delay: 0.,
			duration,
			ease: Ease::Linear,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			start: start.to_variant(),
			end: end.to_variant(),
			do_on_finish: Vec::new(),
			lerp_fn,
		}
	}

	pub fn new_registered<T: _Lerp + FromVariant + ToVariant + Clone>(
		method: impl Into<GodotString>,
		target: &impl Inherits<Object>,
		start: T,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant)
		-> Result<TweenID<TweenMethod_Variant>> {
		Self::new(method, target, start, end, duration, auto_play, lerp_fn)
			.register::<T>()
	}

	pub fn register<T: _Lerp + FromVariant + ToVariant + Clone>(self) -> Result<TweenID<TweenMethod_Variant>> {
		let singleton =
			&mut TweensController::singleton().try_borrow_mut()?;

		let id = singleton.register_tween::<TweenMethod_Variant>(TweenMethod::Variant(self));
		Ok(id)
	}
}

method_impl!(Variant, TweenMethod_Variant, Variant);
value_impl!(Variant, TweenMethod_Variant);
base_impl!(TweenMethod_Variant);