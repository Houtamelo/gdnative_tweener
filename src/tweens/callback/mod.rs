#[allow(unused_imports)]
use crate::*;
use crate::internal_prelude::tween_base_macro::base_impl;

#[derive(Debug)]
pub struct TweenCallback {
	pub callback: Callback,
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
	pub fn new_method(
		target: &impl Inherits<Object>,
		method: impl Into<GodotString>,
		args: Vec<Variant>,
		delay: f64,
		auto_play: AutoPlay)
		-> Self {
		Self {
			callback: Callback::GodotMethodCall(
				GodotMethodCall {
					target: unsafe { target.base() },
					method: method.into(),
					args,
				}),
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
	
	pub fn new_closure(
		closure: impl Fn() + 'static,
		delay: f64,
		auto_play: AutoPlay)
		-> Self {
		Self {
			callback: Callback::Closure(Box::new(closure)),
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

	pub fn register(self) -> Result<TweenID<TweenCallback>> {
		let singleton =
			&mut TweensController::singleton().try_borrow_mut()?;

		let id = singleton.register_tween::<TweenCallback>(self);
		Ok(id)
	}
}

impl TweenCallback {
	pub fn callback(&self) -> &Callback { &self.callback }
	
	fn seek_end(&mut self) {
		unsafe { self.callback.invoke().log_if_err() };
		self.on_finish();
	}

	fn advance_time_internal(&mut self, delta_time: f64) -> Result<Option<f64>> {
		self.elapsed_time += delta_time * self.speed_scale;
		
		let excess = self.elapsed_time - self.delay;
		if excess <= 0. {
			return Ok(None);
		}

		unsafe { self.callback.invoke().log_if_err() };
		
		self.cycle_count += 1;
		self.elapsed_time = excess;

		match &mut self.loop_mode {
			LoopMode::Infinite => {
				Ok(None)
			},
			LoopMode::Finite(loop_count) => {
				if self.cycle_count >= *loop_count {
					self.on_finish();
					self.elapsed_time = self.delay;
					Ok(Some(excess))
				} else {
					Ok(None)
				}
			}
		}
	}
	
	pub(crate) fn set_state_internal(&mut self, new_state: State) {
		self.state = new_state;
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
