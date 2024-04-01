#![allow(unused)]

macro_rules! method_def {
	($value_ty: ty, $struct_ty: ident) => {
		#[derive(Debug)]
		pub struct $struct_ty {
		    method: Rc<String>,
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
		}
	};
}

macro_rules! method_register {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn new(
				method: impl Into<String>,
				target: &impl Inherits<Object>,
				start: $value_ty,
				end: $value_ty,
				duration: f64,
				auto_play: AutoPlay)
				-> Self {
				Self {
					method: Rc::new(method.into()),
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
				}
			}
		
			pub fn new_registered(
				method: impl Into<String>,
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
					&mut TweensBrain::singleton().try_borrow_mut()?;
		
				let id = singleton.register_tween::<$struct_ty>(TweenMethod::from(self));
				Ok(id)
			}
	    }
    };
}

macro_rules! method_impl {
    ($value_ty: ty, $struct_ty: ident, $struct_enum: ident  $(, $lerp_fn: expr)?) => {
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
	    
	    impl $struct_ty {
		    pub fn method(&self) -> Rc<String> { self.method.clone() }
		    
		    $(
		    fn update_value(&mut self, t: f64) -> Result<()> {
				let percent = self.ease.sample(t);
				let target_value = $lerp_fn(&self.start, &self.end, percent);
				
				match unsafe { self.target.assume_safe_if_sane() } {
					Some(target) => {
						unsafe { target.call_deferred(self.method.as_str(), &[target_value.to_variant()]) };
					}
					None => {
						bail!("Can not invoke `{}`, target is not sane.", self.method.as_str());
					}
				}
				
				Ok(())
			})?
	    }
    };
}

pub(crate) use {
	method_def,
	method_register,
	method_impl,
};