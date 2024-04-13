#[allow(unused_imports)]
use crate::*;
use crate::tweens::tween_base_macro::base_impl;
use crate::tweens::tween_value_macro::value_impl;
use crate::tweens::property::tween_macros::property_impl;

#[derive(Debug)]
pub struct TweenProperty_Variant {
	pub property: GodotString,
	pub target: Ref<Object>,
	pub bound_node: Option<Ref<Node>>,
	pub state: State,
	pub delay: f64,
	pub ease: Ease,
	pub speed_scale: f64,
	pub elapsed_time: f64,
	pub cycle_count: u32,
	pub pause_mode: TweenPauseMode,
	pub process_mode: TweenProcessMode,
	pub loop_mode: LoopMode,
	pub end: Variant,
	pub lerp_mode: LerpMode<Variant>,
	pub do_on_finish: Vec<Callback>,
	lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
	relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
	step_fn: fn(from: &Variant, to: &Variant, speed: f64, t: f64) -> (Variant, StepResult),
	distance_fn: fn(from: &Variant, to: &Variant) -> f64,
}

impl TweenProperty_Variant {
	pub fn new<T: _Lerp + FromVariant + ToVariant + Clone>(
		property: impl Into<GodotString>,
		target: &impl Inherits<Object>,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
		step_fn: fn(from: &Variant, to: &Variant, speed: f64, t: f64) -> (Variant, StepResult),
		distance_fn: fn(from: &Variant, to: &Variant) -> f64)
		-> Self {
		Self {
			property: property.into(),
			target: unsafe { target.base() },
			bound_node: None,
			state: match auto_play.0 {
				true => State::Playing,
				false => State::Paused,
			},
			delay: 0.,
			ease: Ease::Linear,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			lerp_mode: LerpMode::Absolute { duration, start: None },
			end: end.to_variant(),
			do_on_finish: Vec::new(),
			lerp_fn,
			relative_fn,
			step_fn,
			distance_fn,
		}
	}

	pub fn new_registered<T: _Lerp + FromVariant + ToVariant + Clone>(
		property: impl Into<GodotString>,
		target: &impl Inherits<Object>,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
		step_fn: fn(from: &Variant, to: &Variant, speed: f64, t: f64) -> (Variant, StepResult),
		distance_fn: fn(from: &Variant, to: &Variant) -> f64)
		-> Result<TweenID<TweenProperty_Variant>> {
		Self::new(property, target, end, duration, auto_play, lerp_fn, relative_fn, step_fn, distance_fn)
			.register::<T>()
	}

	pub fn register<T: _Lerp + FromVariant + ToVariant + Clone>(self) 
		-> Result<TweenID<TweenProperty_Variant>> { 
		let singleton = 
			&mut TweensController::singleton().try_borrow_mut() ?;
		
		let id = singleton.register_tween::<TweenProperty_Variant>(TweenProperty::Variant(self));
		Ok(id)
	}
}

property_impl!(Variant, TweenProperty_Variant, Variant);
value_impl!(Variant, TweenProperty_Variant);
base_impl!(TweenProperty_Variant);
