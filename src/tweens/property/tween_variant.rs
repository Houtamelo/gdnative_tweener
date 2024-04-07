#[allow(unused_imports)]
use crate::*;
use crate::tweens::property::LerpMode;
use crate::tweens::tween_base_macro::base_impl;
use crate::tweens::tween_value_macro::value_impl;
use crate::tweens::property::tween_macros::property_impl;

#[derive(Debug, Clone)]
pub struct TweenProperty_Variant {
	pub property: GodotString,
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
	loop_mode: LoopMode,
	pub start: Variant,
	pub end: Variant,
	lerp_mode: LerpMode<Variant>,
	pub do_on_finish: Vec<Callback>,
	lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
	relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
}

impl TweenProperty_Variant {
	pub fn new<T: _Lerp + FromVariant + ToVariant + Clone + Copy>(
		property: impl Into<GodotString>,
		target: &impl Inherits<Object>,
		start: T,
		end: T,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant)
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
			duration,
			ease: Ease::Linear,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			lerp_mode: LerpMode::Absolute,
			start: start.to_variant(),
			end: end.to_variant(),
			do_on_finish: Vec::new(),
			lerp_fn,
			relative_fn,
		}
	}

	pub fn new_registered<T: _Lerp + FromVariant + ToVariant + Clone + Copy>(
		property: impl Into<GodotString>,
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
			&mut TweensController::singleton().try_borrow_mut() ?;
		
		let id = singleton.register_tween::<TweenProperty_Variant>(TweenProperty::Variant(self));
		Ok(id)
	}

	pub fn lerp_relative(mut self) -> Self {
		match self.lerp_mode {
			| LerpMode::Flexible{..} 
			| LerpMode::Absolute => {
				let ratio = self.elapsed_ratio();
				
				self.lerp_mode = LerpMode::Relative {
					previous_value: (self.lerp_fn)(&self.start, &self.end, ratio)
				};
			}
			LerpMode::Relative { .. } => {},
		};

		self
	}
	
	fn update_value(&mut self, t: f64) -> Result<()> {
		let Some(target) = (unsafe { self.target.assume_safe_if_sane() }) 
			else { bail!("Can not set property `{}` on Object, target is not sane.", self.property) };
		
		let value_at_obj = target.get_indexed(self.property.new_ref());
		
		let ratio = self.ease.sample(t);

		let target_value =
			match &mut self.lerp_mode {
				LerpMode::Flexible { starting_ratio } => {
					let actual_ratio = actual_ratio(*starting_ratio, ratio);
					(self.lerp_fn)(&self.start, &self.end, actual_ratio)
				},
				LerpMode::Relative { previous_value } => {
					let next_value = (self.lerp_fn)(&self.start, &self.end, ratio);
					let target_value = (self.relative_fn)(&value_at_obj, &previous_value, &next_value);
					*previous_value = next_value;
					target_value
				},
				LerpMode::Absolute => {
					(self.lerp_fn)(&self.start, &self.end, ratio)
				},
			};

		unsafe { match self.target.assume_safe_if_sane() {
			Some(target) => {
				target.call_deferred("set_indexed", &[self.property.to_variant(), target_value.to_variant()]);
			}
			None => {
				bail!("Can not set property `{}` on Object, target is not sane.", self.property);
			}
		} }
		
		Ok(())
	}
}

property_impl!(Variant, TweenProperty_Variant, Variant);
value_impl!(Variant, TweenProperty_Variant);
base_impl!(TweenProperty_Variant);
