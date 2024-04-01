#[allow(unused_imports)]
use crate::*;
use crate::tweens::tween_base_macro::base_impl;
use crate::tweens::tween_value_macro::value_impl;
use crate::tweens::property::tween_macros::property_impl;

#[derive(Debug)]
pub struct TweenProperty_Variant {
	property: Rc<String>,
	pub target: Ref<Object>,
	pub bound_node: Option<Ref<Node>>,
	pub state: State,
	pub delay: f64,
	pub duration: f64,
	pub ease: Ease,
	pub speed_scale: f64,
	pub elapsed_time: f64,
	pub cycle_count: u32,
	pub pause_mode: TweenPauseMode,
	pub process_mode: TweenProcessMode,
	pub loop_mode: LoopMode,
	pub relative: bool,
	pub start: Variant,
	pub end: Variant,
	pub previous_value: Variant,
	pub do_on_finish: Vec<Callback>,
	lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
	relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
}

impl TweenProperty_Variant {
	pub fn new<T: _Lerp + FromVariant + ToVariant + Clone + Copy>(
		property: impl Into<String>,
		target: &impl Inherits<Object>,
		start: T,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant)
		-> Self {
		Self {
			property: Rc::new(property.into()),
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
			relative: false,
			start: start.to_variant(),
			end: end.to_variant(),
			previous_value: start.to_variant(),
			do_on_finish: Vec::new(),
			lerp_fn,
			relative_fn,
		}
	}

	pub fn new_registered<T: _Lerp + FromVariant + ToVariant + Clone + Copy>(
		property: impl Into<String>,
		target: &impl Inherits<Object>,
		start: T,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant)
		-> Result<TweenID<TweenProperty_Variant>> {
		Self::new(property, target, start, end, duration, auto_play, lerp_fn, relative_fn)
			.register::<T>()
	}

	pub fn register<T: _Lerp + FromVariant + ToVariant + Clone + Copy>(self) 
		-> Result<TweenID<TweenProperty_Variant>> { 
		let singleton = 
			&mut TweensBrain::singleton().try_borrow_mut() ?;
		
		let id = singleton.register_tween::<TweenProperty_Variant>(TweenProperty::Variant(self));
		Ok(id)
	}
	
	fn update_value(&mut self, t: f64) -> Result<()> {
		let Some(target) = (unsafe { self.target.assume_safe_if_sane() }) 
			else { bail!("Can not set property `{}` on Object, target is not sane.", self.property.as_str()) };
		
		let value_at_obj = target.get(self.property.as_str());
		
		let percent = self.ease.sample(t);
		let next_value = (self.lerp_fn)(&self.start, &self.end, percent);
		
		let target_value =
			if self.relative {
				(self.relative_fn)(&value_at_obj, &self.previous_value, &next_value)
			} else {
				next_value.clone()
			};
		
		self.previous_value = next_value;
		
		match unsafe { self.target.assume_safe_if_sane() } {
			Some(target) => {
				target.set_deferred(self.property.as_str(), target_value.to_variant());
			}
			None => {
				bail!("Can not set property `{}` on Object, target is not sane.", self.property.as_str());
			}
		}
		
		Ok(())
	}
}

property_impl!(Variant, TweenProperty_Variant, Variant);
value_impl!(Variant, TweenProperty_Variant);
base_impl!(TweenProperty_Variant);
