#[allow(unused_imports)]
use crate::*;

macro_rules! property_def {
    ($value_ty: ty, $struct_ty: ident) => {
	    #[derive(Debug)]
		pub struct $struct_ty {
		    pub property: GodotString,
			pub target: Ref<Object>,
			pub bound_node: Option<Ref<Node>>,
			state: State,
			pub delay: f64,
			pub ease: Ease,
			pub speed_scale: f64,
			pub elapsed_time: f64,
		    pub cycle_count: u32,
			pub pause_mode: TweenPauseMode, 
			pub process_mode: TweenProcessMode,
			pub loop_mode: LoopMode,
			pub lerp_mode: LerpMode<$value_ty>,
			pub end: $value_ty,
			pub do_on_finish: Vec<Callback>,
		    lerp_fn: fn(from: &$value_ty, to: &$value_ty, f64) -> $value_ty,
			relative_fn: fn(value_at_obj: &$value_ty, previous_calc: &$value_ty, next_calc: &$value_ty) -> $value_ty,
			step_fn: fn(from: &$value_ty, to: &$value_ty, speed: f64, t: f64) -> ($value_ty, StepResult),
		    distance_fn: fn(from: &$value_ty, to: &$value_ty) -> f64,
		}
    };
}

macro_rules! property_register {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn new(
				property: impl Into<GodotString>,
				target: &impl Inherits<Object>,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
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
					end,
					do_on_finish: Vec::new(),
					lerp_fn: <$value_ty>::_lerp,
					relative_fn: <$value_ty>::add_relative,
					step_fn: <$value_ty>::step,
					distance_fn: <$value_ty>::distance,
				}
			}
		
			pub fn new_registered(
				property: impl Into<GodotString>,
				target: &impl Inherits<Object>,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
				-> Result<TweenID<$struct_ty>> {
				Self::new(property, target, end, duration, auto_play)
					.register()
			}
		    
		    pub fn register(self) -> Result<TweenID<$struct_ty>> {
				let singleton =
					&mut TweensController::singleton().try_borrow_mut()?;
		
				let id = singleton.register_tween::<$struct_ty>(TweenProperty::from(self));
				Ok(id)
			} 
	    }
    };
}

macro_rules! property_impl {
    ($value_ty: ty, $struct_ty: ident, $struct_enum: ident) => {
	    impl FromTween for $struct_ty {
			fn from_tween(tween: &AnyTween) -> Option<&Self> {
				if let AnyTween::Property(TweenProperty::$struct_enum(t)) = tween {
					Some(t)
				} else {
					None
				}
			}
		
			fn from_tween_mut(tween: &mut AnyTween) -> Option<&mut Self> {
				if let AnyTween::Property(TweenProperty::$struct_enum(t)) = tween {
					Some(t)
				} else {
					None
				}
			}
		}
	    
	    impl From<$struct_ty> for AnyTween {
			fn from(tween: $struct_ty) -> Self {
				AnyTween::Property(tween.into()) 
			}
	    }
	    
	    impl $struct_ty {
		    pub fn with_duration(self, duration: f64) -> Self { 
			    match self.lerp_mode {
					LerpMode::Absolute { start, .. } => {
						Self { lerp_mode: LerpMode::Absolute { duration, start }, ..self }
					}
					LerpMode::Relative { origin, .. } => {
						Self { lerp_mode: LerpMode::Relative { duration, origin }, ..self }
					}
				    LerpMode::SpeedBased { .. } => { 
						godot_warn!("Duration is not used in current lerp mode (SpeedBased).");
						self
					}
			    }
			}
		    
		    pub(crate) fn set_state_internal(&mut self, new_state: State) {
				if self.state == new_state {
					return;
				}
		
				self.state = new_state;
		
				if let State::Stopped = self.state {
					self.elapsed_time = 0.0;
					self.cycle_count = 0; 
				}
			}
		    
		    pub fn starting_at(self, value: $value_ty) -> Self {
			    match self.lerp_mode {
					LerpMode::Absolute { duration, .. } => {
						Self { lerp_mode: LerpMode::Absolute { duration, start: Some(value) }, ..self }
					}
				    LerpMode::SpeedBased { .. } => {
						godot_warn!("Starting value is not used in current lerp mode (SpeedBased).");
						self
					}
				    LerpMode::Relative { duration, .. } => {
						Self { lerp_mode: LerpMode::Relative { duration, origin: value }, ..self }
					}
			    }
			}
		    
		    pub fn evaluate_start_on_play(self) -> Self {
			    if let LerpMode::Absolute { duration, .. } = self.lerp_mode {
				    Self { lerp_mode: LerpMode::Absolute { duration, start: None }, ..self }
			    } else {
				    godot_warn!("Evaluating start value on play is only used in `Absolute` lerp mode.");
				    self
			    }
			}
		    
		    pub fn is_absolute(&self) -> bool {
				matches!(self.lerp_mode, LerpMode::Absolute{..})
			}
		    
		    pub fn is_relative(&self) -> bool {
				matches!(self.lerp_mode, LerpMode::Relative { .. })
			}
		    
		    pub fn is_speed_based(&self) -> bool {
				matches!(self.lerp_mode, LerpMode::SpeedBased { .. })
			}
			
			pub fn as_absolute(self) -> Self {
				match self.lerp_mode {
					LerpMode::SpeedBased { speed, .. } => {
						let val_at_obj =
							match eval_property(&self.target, &self.property) {
								Ok(val) => val,
								Err(err) => {
									godot_error!("{err}");
									return self;
								}
							};
							
						let distance = (self.distance_fn)(&val_at_obj, &self.end);
						let duration = distance / speed;
						
						Self { lerp_mode: LerpMode::Absolute { duration, start: None }, ..self }
					}
					LerpMode::Relative { duration, origin } => {
						Self { lerp_mode: LerpMode::Absolute { duration, start: Some(origin) }, ..self }
					}
					LerpMode::Absolute { .. } => {
						self
					},
				}
			}
		    
		    pub fn as_speed_based(self, speed: f64) -> Self {
				Self { lerp_mode: LerpMode::SpeedBased { speed, t_sum: 0. }, ..self }
			}
		    
			pub fn as_relative(self, origin: $value_ty) -> Self {
				match self.lerp_mode {
					LerpMode::Absolute { duration, .. } => {
						Self { lerp_mode: LerpMode::Relative { duration, origin }, ..self }
					}
					LerpMode::SpeedBased { speed, .. } => {
						let distance = (self.distance_fn)(&origin, &self.end);
						let duration = distance / speed;
						
						Self { lerp_mode: LerpMode::Relative { duration, origin }, ..self }
					}
					LerpMode::Relative { duration, .. } => {
						Self { lerp_mode: LerpMode::Relative { duration, origin }, ..self }
					},
				}
			}
		    
		    fn seek_end(&mut self) {
			    let Some(target) = (unsafe { self.target.assume_safe_if_sane() }) 
					else {
						self.on_finish();
						return godot_error!("Can not set property `{}` on Object, target is not sane.", self.property);
					};
			    
			    let target_value = 
				    match &mut self.lerp_mode {
						| LerpMode::Absolute { .. }
						| LerpMode::SpeedBased { .. } => {
							self.end.clone()
						}
						LerpMode::Relative { duration, origin } => {
							let val_at_obj =
								match eval_property(&self.target, &self.property) {
									Ok(val) => val,
									Err(err) => {
										self.on_finish();
										return godot_error!("{err}");
									}
								};
							
							let previous_relative = {
								let previous_eased_ratio = {
									let previous_ratio = ratio_with_delay_duration(self.delay, *duration, self.elapsed_time);
									self.ease.sample(previous_ratio)
								};
								
								(self.lerp_fn)(&origin, &self.end, previous_eased_ratio)
							};
							
							(self.relative_fn)(&val_at_obj, &previous_relative, &self.end)
						}
					};
			    
			    target.set_indexed(self.property.new_ref(), target_value.to_variant());
			    
			    self.on_finish();
		    }
			
			fn advance_time_internal(&mut self, delta_time: f64) -> Result<Option<f64>> {
			    self.elapsed_time += delta_time * self.speed_scale;
			    
			    if self.elapsed_time < self.delay {
				    return Ok(None);
			    }
			    
				let Some(target) = (unsafe { self.target.assume_safe_if_sane() }) 
					else {
						self.stop();
						bail!("Can not set property `{}` on Object, target is not sane.", self.property)
					};

				let (target_value, excess_time) =
					match &mut self.lerp_mode {
						LerpMode::Absolute { duration, start } => {
							let start_val = 
								match &start {
									Some(val) => val,
									None => {
										let val_at_obj = eval_property(&self.target, &self.property)?;
										start.replace(val_at_obj);
										start.as_ref().unwrap()
									}
								};
							
							let target_value = {
								let elapsed_ratio = ratio_with_delay_duration(self.delay, *duration, self.elapsed_time);
								let eased_ratio = self.ease.sample(elapsed_ratio.min(1.));
								(self.lerp_fn)(start_val, &self.end, eased_ratio)
							};
							
							let excess_time = {
								let total_duration = self.delay + *duration;
								let excess = self.elapsed_time - total_duration;
								(excess > 0.).then_some(excess)
							};
							
							(target_value, excess_time)
						}
						LerpMode::SpeedBased { speed, t_sum } => {
							let (target_value, step_result) = {
								let val_at_obj = eval_property(&self.target, &self.property)?;
								(self.step_fn)(&val_at_obj, &self.end, *speed, delta_time + *t_sum)
							};
							
							let excess_time = 
								match step_result {
									StepResult::Unfinished { accumulated_t } => {
										*t_sum = accumulated_t;
										None
									}
									StepResult::Finished { excess_time } => {
										*t_sum = 0.;
										Some(excess_time)
									}
								};
							
							(target_value, excess_time)
						}
						LerpMode::Relative { duration, origin } => {
							let target_value = {
								let val_at_obj = eval_property(&self.target, &self.property)?;
								
								let previous_eased_ratio = {
									let previous_ratio = ratio_with_delay_duration(self.delay, *duration, self.elapsed_time - delta_time);
									self.ease.sample(previous_ratio)
								};
								
								let next_eased_ratio = {
									let elapsed_ratio = ratio_with_delay_duration(self.delay, *duration, self.elapsed_time);
									self.ease.sample(elapsed_ratio)
								};
								
								let previous_relative = (self.lerp_fn)(&origin, &self.end, previous_eased_ratio);
								let next_relative = (self.lerp_fn)(&origin, &self.end, next_eased_ratio);
								
								(self.relative_fn)(&val_at_obj, &previous_relative, &next_relative)
							};
							
							let excess_time = {
								let total_duration = self.delay + *duration;
								let excess = self.elapsed_time - total_duration;
								(excess > 0.).then_some(excess)
							};
							
							(target_value, excess_time)
						}
					};
				
			    target.set_indexed(self.property.new_ref(), target_value.to_variant());
				
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

pub(crate) use {property_def, property_register, property_impl};