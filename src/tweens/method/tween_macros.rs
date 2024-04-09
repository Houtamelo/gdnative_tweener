#![allow(unused)]

macro_rules! method_def {
	($value_ty: ty, $struct_ty: ident) => {
		#[derive(Debug)]
		pub struct $struct_ty {
		    pub method: GodotString,
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
			pub start: $value_ty,
			pub end: $value_ty,
			pub do_on_finish: Vec<Callback>,
			lerp_fn: fn(from: &$value_ty, to: &$value_ty, f64) -> $value_ty,
		}
	};
}

macro_rules! method_register {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn new(
				method: impl Into<GodotString>,
				target: &impl Inherits<Object>,
				start: $value_ty,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
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
					start: start.clone(),
					end,
					do_on_finish: Vec::new(),
					lerp_fn: <$value_ty>::_lerp,
				}
			}
		
			pub fn new_registered(
				method: impl Into<GodotString>,
				target: &impl Inherits<Object>,
				start: $value_ty,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
				-> Result<TweenID<$struct_ty>> {
				Self::new(method, target, start, end, duration, auto_play)
					.register()
			}
		    
		    pub fn register(self) -> Result<TweenID<$struct_ty>> {
				let singleton =
					&mut TweensController::singleton().try_borrow_mut()?;
		
				let id = singleton.register_tween::<$struct_ty>(TweenMethod::from(self));
				Ok(id)
			}
	    }
    };
}

macro_rules! method_impl {
    ($value_ty: ty, $struct_ty: ident, $struct_enum: ident) => {
	    impl FromTween for $struct_ty {
			fn from_tween(tween: &AnyTween) -> Option<&Self> {
				if let AnyTween::Method(TweenMethod::$struct_enum(t)) = tween {
					Some(t)
				} else {
					None
				}
			}
		
			fn from_tween_mut(tween: &mut AnyTween) -> Option<&mut Self> {
				if let AnyTween::Method(TweenMethod::$struct_enum(t)) = tween {
					Some(t)
				} else {
					None
				}
			}
		}
	    
	    impl From<$struct_ty> for AnyTween {
			fn from(tween: $struct_ty) -> Self {
				AnyTween::Method(tween.into()) 
			}
	    }
	    
	    impl $struct_ty {
		    pub fn starting_at(self, value: $value_ty) -> Self { 
				Self { start: value, ..self }
			}
		    
		    pub fn with_duration(self, duration: f64) -> Self { 
			    Self { duration, ..self }
			}
		    
		    pub(crate) fn set_state_internal(&mut self, state: State) {
			    self.state = state;
		    }
		    
		    fn seek_end(&mut self) {
			    let target_value = {
				    let eased_ratio = self.ease.sample(1.);
				    (self.lerp_fn)(&self.start, &self.end, eased_ratio)
			    };
			    
			    let Some(target) = (unsafe { self.target.assume_safe_if_sane() })
				    else {
				        self.on_finish();
				        return godot_error!("Cannot call method `{}`, target is not sane", self.method)
			        };
			    
			    unsafe { target.call_deferred(self.method.new_ref(), &[target_value.to_variant()]) };
			    self.on_finish();
		    }
		    
		    fn advance_time_internal(&mut self, delta_time: f64) -> Result<Option<f64>> {
			    self.elapsed_time += delta_time * self.speed_scale;
			    
				let target_value = {
					let eased_ratio = { 
						let elapsed_ratio = ratio_with_delay_duration(self.delay, self.duration, self.elapsed_time);
						self.ease.sample(elapsed_ratio)
					};
					
					(self.lerp_fn)(&self.start, &self.end, eased_ratio)
				};
				
				let excess_time = {
					let total_duration = self.delay + self.duration;
					let excess = self.elapsed_time - total_duration;
					(excess > 0.).then_some(excess)
				};
			    
			    unsafe { 
				    self.target
				        .assume_safe_if_sane()
				        .ok_or_else(|| anyhow!("Cannot call method `{}`, target is not sane", self.method))?
				        .call_deferred(self.method.new_ref(), &[target_value.to_variant()]);
			    }
			    
			    let final_excess_time = 
					match excess_time {
						Some(excess) => {
							self.cycle_count += 1;
						
							match &mut self.loop_mode {
								LoopMode::Infinite => {
									self.elapsed_time = self.delay + excess;
									None
								}
								LoopMode::Finite(loop_count) => { 
									if self.cycle_count < *loop_count { 
										self.elapsed_time = self.delay + excess;
										None 
									} else {
										self.elapsed_time -= excess;
										Some(excess)
									}
								}
							}
						}
						None => None,
					};
				
				Ok(final_excess_time.inspect(|_| self.on_finish()))
			}
	    }
    };
}

pub(crate) use {
	method_def,
	method_register,
	method_impl,
};
use crate::prelude::ratio_with_delay_duration;