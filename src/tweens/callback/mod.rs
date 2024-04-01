#[allow(unused_imports)]
use crate::*;
use crate::internal_prelude::tween_base_macro::base_impl;

#[derive(Debug)]
pub struct TweenCallback {
	pub(crate) callback: Callback,
	pub bound_node: Option<Ref<Node>>,
	pub state: State,
	pub delay: f64,
	pub speed_scale: f64,
	pub elapsed_time: f64,
	pub cycle_count: u32,
	pub pause_mode: TweenPauseMode,
	pub process_mode: TweenProcessMode,
	pub loop_mode: LoopMode,
	pub do_on_finish: Vec<Callback>,
}

impl TweenCallback {
	pub fn new(method: impl Into<String>,
	           target: &impl Inherits<Object>,
	           args: Vec<Variant>,
	           delay: f64,
	           auto_play: AutoPlay)
	           -> Self {
		Self {
			callback: Callback {
				method: Rc::new(method.into()),
				target: unsafe { target.base() },
				args,
			},
			bound_node: None,
			state: match auto_play.0 {
				true => State::Playing,
				false => State::Paused,
			},
			delay,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			do_on_finish: Vec::new(),
		}
	}

	pub fn new_registered(method: impl Into<String>,
	                      target: &impl Inherits<Object>,
	                      args: Vec<Variant>,
	                      delay: f64,
	                      auto_play: AutoPlay)
	                      -> Result<TweenID<TweenCallback>> {
		Self::new(method, target, args, delay, auto_play)
			.register()
	}

	pub fn register(self) -> Result<TweenID<TweenCallback>> {
		let singleton =
			&mut TweensBrain::singleton().try_borrow_mut()?;

		let id = singleton.register_tween::<TweenCallback>(self);
		Ok(id)
	}

	fn cycle_duration_internal(&self) -> f64 {
		self.delay
	}
}

impl TweenCallback {
	pub fn target(&self) -> Ref<Object> { self.callback.target }
	pub fn method(&self) -> Rc<String> { Rc::clone(&self.callback.method) }
	pub fn args(&self) -> &[Variant] { &self.callback.args }

	fn check_elapsed_time(&mut self) -> f64 {
		let excess = self.elapsed_time - self.delay;
		if excess > 0. {
			return 0.;
		}

		unsafe { self.callback.invoke().log_if_err() };
		self.cycle_count += 1;
		self.elapsed_time = f64::max(0., self.elapsed_time - self.delay);

		match &mut self.loop_mode {
			LoopMode::Infinite => {
				0.
			},
			LoopMode::Finite(loop_count) => {
				if self.cycle_count >= *loop_count {
					self.on_finish();
					self.elapsed_time = self.delay;
					excess
				} else {
					0.
				}
			}
		}
	}
	
	fn update_value(&mut self, t: f64) -> Result<()> {
		if t >= 1. {
			unsafe { self.callback.invoke() }
		} else {
			Ok(())
		}
	}
}

base_impl!(TweenCallback);

impl FromTween for TweenCallback {
	fn from_tween(tween: &AnyTween) -> Option<&Self> {
		if let AnyTween::Callback(t) = tween {
			Some(t)
		} else {
			None
		}
	}
	
	fn from_tween_mut(tween: &mut AnyTween) -> Option<&mut Self> {
		if let AnyTween::Callback(t) = tween {
			Some(t)
		} else {
			None
		}
	}
}
