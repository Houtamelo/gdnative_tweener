#[allow(unused_imports)]
use crate::*;

#[allow(unused)]
macro_rules! property_def {
    ($value_ty: ty, $struct_ty: ident) => {
	    #[derive(Debug, Clone)]
		pub struct $struct_ty {
		    pub property: GodotString,
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
			lerp_mode: LerpMode<$value_ty>,
			pub start: $value_ty,
			pub end: $value_ty,
			pub do_on_finish: Vec<Callback>,
		}
    };
}

#[allow(unused)]
macro_rules! property_register {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn new(
				property: impl Into<GodotString>,
				target: &impl Inherits<Object>,
				start: $value_ty,
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
					duration,
					ease: Ease::Linear,
					speed_scale: 1.,
					elapsed_time: 0.,
					cycle_count: 0,
					pause_mode: TweenPauseMode::STOP,
					process_mode: TweenProcessMode::IDLE,
					loop_mode: LoopMode::Finite(0),
					lerp_mode: LerpMode::Absolute,
					start: start.clone(),
					end,
					do_on_finish: Vec::new(),
				}
			}
		
			pub fn new_registered(
				property: impl Into<GodotString>,
				target: &impl Inherits<Object>,
				start: $value_ty,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
				-> Result<TweenID<$struct_ty>> {
				Self::new(property, target, start, end, duration, auto_play)
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

#[allow(unused)]
macro_rules! property_impl {
    ($value_ty: ty, $struct_ty: ident, $struct_enum: ident  $(, $lerp_fn: expr, $relative_fn: expr)?) => {
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
		    pub(crate) fn set_state_internal(&mut self, new_state: State) {
			    if self.state == new_state {
				    return;
			    }
			    
			    self.state = new_state;
			    let current_ratio = self.elapsed_ratio();
			    
			    match self.state {					    
				    State::Playing => {
					    if let LerpMode::Flexible { starting_ratio } = &mut self.lerp_mode {
						    *starting_ratio = current_ratio;
						    let variant = unsafe {
							    self.target
							        .assume_safe()
							        .get_indexed(self.property.new_ref())
						    };
						    
						    self.start =
						    variant.try_to::<$value_ty>()
						           .unwrap();
					    }
				    },
				    State::Stopped => {
						self.elapsed_time = 0.0;
						self.cycle_count = 0;
				    },
				    State::Paused => {}
			    }
		    }
		    
		    pub fn is_relative(&self) -> bool {
				matches!(self.lerp_mode, LerpMode::Relative { .. })
			}
		    
		    pub fn is_absolute(&self) -> bool {
				matches!(self.lerp_mode, LerpMode::Absolute { .. })
			}
		    
		    pub fn lerp_flexible(mut self) -> Self {
				match self.lerp_mode {
					| LerpMode::Relative{..} 
					| LerpMode::Absolute => {
						let value_at_obj = unsafe { 
							self.target
								.assume_safe()
								.get_indexed(self.property.new_ref())
								.try_to::<$value_ty>()
								.unwrap()
						};
						
						self.start = value_at_obj;
		
						let starting_ratio = self.elapsed_ratio();
						self.lerp_mode = LerpMode::Flexible { starting_ratio };
					}
					LerpMode::Flexible {..} => {},
				}
		
				self
			}
			
			pub fn lerp_absolute(mut self) -> Self {
				self.lerp_mode = LerpMode::Absolute;
				self
			}
			
			pub fn starting_at_current(mut self) -> Result<Self> {
				let target = unsafe {
					self.target
					    .assume_safe_if_sane()
					    .ok_or_else(|| anyhow!("Target is not sane."))?
				};
		
				let variant = target.get_indexed(self.property.new_ref());
				
				let current =
					variant.try_to::<$value_ty>()
						   .map_err(|err| anyhow!(
								"Target property `{}` is not of type `{}`, got: `{:?}`. \n\
								 Error: {}", self.property, variant, std::any::type_name::<$value_ty>(), err))?;
		
				self.start = current.into();
				Ok(self)
			}

			$(
			pub fn lerp_relative(mut self) -> Self {
				match self.lerp_mode {
					| LerpMode::Flexible{..} 
					| LerpMode::Absolute => {
						let ratio = self.elapsed_ratio();
						
						self.lerp_mode = LerpMode::Relative {
							previous_value: $lerp_fn(&self.start, &self.end, ratio)
						};
					}
					LerpMode::Relative { .. } => {},
				};
		
				self
			}
			
			fn update_value(&mut self, t: f64) -> Result<()> {
				let Some(target) = (unsafe { self.target.assume_safe_if_sane() }) 
					else { bail!("Can not set property `{}` on Object, target is not sane.", self.property) };
				
				let value_at_obj =
					match target.get_indexed(self.property.new_ref()).try_to::<$value_ty>() {
						Ok(value) => { value }
						Err(err) => {
							bail!("Can not get property `{}` on Object, target is not of type `{}`.\n\
								   Error: {}", self.property, std::any::type_name::<$value_ty>(), err)
						}
					};
				
				let ratio = self.ease.sample(t);
				
				let target_value =
					match &mut self.lerp_mode {
						LerpMode::Flexible { starting_ratio } => {
							let actual_ratio = $crate::tweens::property::actual_ratio(*starting_ratio, ratio);
							$lerp_fn(&self.start, &self.end, actual_ratio)
						},
						LerpMode::Relative { previous_value } => {
							let next_value   = $lerp_fn(&self.start, &self.end, ratio);
							let target_value = $relative_fn(&value_at_obj, &previous_value, &next_value);
							*previous_value  = next_value;
							target_value
						},
						LerpMode::Absolute => {
							$lerp_fn(&self.start, &self.end, ratio)
						},
					};
				
				unsafe { 
					match self.target.assume_safe_if_sane() {
						Some(target) => {
							target.call_deferred("set_indexed", &[self.property.to_variant(), target_value.to_variant()]);
						}
						None => {
							bail!("Can not set property `{}` on Object, target is not sane.", self.property);
						} 
					}
				}
				
				Ok(())
			})?
		}
    };
}

pub(crate) use {property_def, property_register, property_impl};