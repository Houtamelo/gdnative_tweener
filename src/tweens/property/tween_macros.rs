#[allow(unused_imports)]
use crate::*;

#[allow(unused)]
macro_rules! property_def {
    ($value_ty: ty, $struct_ty: ident) => {
	    #[derive(Debug)]
		pub struct $struct_ty {
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
			pub start: $value_ty,
			pub end: $value_ty,
		    pub previous_value: $value_ty,
			pub do_on_finish: Vec<Callback>,
		}
    };
}

#[allow(unused)]
macro_rules! property_register {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn new(
				property: impl Into<String>,
				target: &impl Inherits<Object>,
				start: $value_ty,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
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
					start: start.clone(),
					end,
					previous_value: start,
					do_on_finish: Vec::new(),
				}
			}
		
			pub fn new_registered(
				property: impl Into<String>,
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
					&mut TweensBrain::singleton().try_borrow_mut()?;
		
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
	    
	    impl $struct_ty {
			pub fn property(&self) -> Rc<String> { self.property.clone() }
			
			pub fn as_relative(&mut self) -> &mut Self { 
				self.relative = true;
				self
			}
		    
		    pub fn is_absolute(&self) -> bool {
				!self.relative
			}
			
			pub fn as_absolute(&mut self) -> &mut Self {
				self.relative = false;
				self
			}
			
			pub fn starting_at_current(&mut self) -> Result<&mut Self> {
				let target = unsafe {
					self.target
					    .assume_safe_if_sane()
					    .ok_or_else(|| anyhow!("Target is not sane."))?
				};
		
				let variant = target.get(self.property.as_str());
				
				let current =
					variant.try_to::<$value_ty>()
						   .map_err(|err| anyhow!(
								"Target property `{}` is not of type `{}`, got: {:?}.\n\
								 Error: {}", self.property.as_str(), variant, std::any::type_name::<$value_ty>(), err))?;
		
				self.start = current.into();
				Ok(self)
			}

			$(
			fn update_value(&mut self, t: f64) -> Result<()> {
				let Some(target) = (unsafe { self.target.assume_safe_if_sane() }) 
					else { bail!("Can not set property `{}` on Object, target is not sane.", self.property.as_str()) };
				
				let value_at_obj =
					match target.get(self.property.as_str()).try_to::<$value_ty>() {
						Ok(value) => { value }
						Err(err) => {
							bail!("Can not get property `{}` on Object, target is not of type `{}`.\n\
								   Error: {}", self.property.as_str(), std::any::type_name::<$value_ty>(), err)
						}
					};
				
				let percent = self.ease.sample(t);
				let next_value = $lerp_fn(&self.start, &self.end, percent);
				
				let target_value =
					if self.relative {
						$relative_fn(&value_at_obj, &self.previous_value, &next_value)
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
			})?
		}
    };
}

pub(crate) use {property_def, property_register, property_impl};