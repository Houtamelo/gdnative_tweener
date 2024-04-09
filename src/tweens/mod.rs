use std::any::Any;
use std::fmt::{Debug, Formatter};
#[allow(unused_imports)]
use crate::*;
use enum_dispatch::enum_dispatch;

mod callback;
mod method;
mod property;
mod id;
pub mod lerping;
pub mod tween_base_macro;
pub mod tween_value_macro;

#[allow(unused_imports)] pub use callback::*;
#[allow(unused_imports)] pub use method::*;
#[allow(unused_imports)] pub use property::*;
#[allow(unused_imports)] pub use id::*;

pub enum Callback {
	GodotMethodCall(GodotMethodCall),
	Closure(Box<dyn Fn() + 'static>)
}

impl Debug for Callback {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Callback::GodotMethodCall(method) => { write!(f, "{method:?}") }
			Callback::Closure(closure) => {
				write!(f, "Closure id: {:?}", closure.type_id())
			}
		}
	}
}

impl Callback {
	pub unsafe fn invoke(&self) -> Result<()> {
		match self {
			Callback::GodotMethodCall(method) => { method.invoke() }
			Callback::Closure(closure) => {
				closure();
				Ok(())
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct GodotMethodCall {
	pub target: Ref<Object>,
	pub method: GodotString,
	pub args: Vec<Variant>,
}

impl GodotMethodCall {
	pub unsafe fn invoke(&self) -> Result<()> {
		let target =  
			self.target
				.assume_safe_if_sane()
				.ok_or_else(|| anyhow!("Target is not sane."))?;

		target.call_deferred(self.method.new_ref(), &self.args);
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToVariant, FromVariant)]
pub struct AutoPlay(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToVariant, FromVariant)]
pub enum State {
	Playing,
	Paused,
	Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToVariant, FromVariant)]
pub enum LoopMode {
	Finite(u32),
	Infinite,
}

#[derive(Debug, Clone, Copy)]
pub enum Duration {
	Finite(f64),
	Infinite
}

#[enum_dispatch]
pub trait Tick: Sized {
	fn state(&self) -> State;
	fn play(&mut self);
	fn pause(&mut self);
	fn stop(&mut self);

	fn is_playing(&self) -> bool { self.state() == State::Playing }
	fn is_paused(&self) -> bool { self.state() == State::Paused }
	fn is_stopped(&self) -> bool { self.state() == State::Stopped }
	
	fn process_mode(&self) -> TweenProcessMode;
	fn pause_mode(&self) -> TweenPauseMode;
	fn bound_node(&self) -> Option<&Ref<Node>>;
	
	fn loop_mode(&self) -> LoopMode;
	fn delay(&self) -> f64;
	
	fn elapsed_time(&self) -> f64;
	fn speed_scale(&self) -> f64;
	
	fn advance_time(&mut self, delta_time: f64) -> Option<f64>;
	fn callbacks_on_finish(&self) -> &[Callback];
	
	/*
	fn seek(&mut self, time: f64);

	fn do_absolute_step(&mut self, delta: f64) {
		let current = self.elapsed_time();
		self.seek(current + delta);
	}

	fn do_scaled_step(&mut self, delta: f64) {
		let current = self.elapsed_time();
		self.seek(current + delta * self.speed_scale());
	}
	*/

	fn on_finish(&mut self) {
		self.stop();

		self.callbacks_on_finish()
			.iter()
		    .for_each(|callback| unsafe { 
			    callback.invoke().log_if_err() });
	}
	
	fn force_finish(self);

	fn is_bounded_dead(&self) -> bool {
		self.bound_node()
		    .is_some_and(|node| unsafe {
			    !node.is_instance_sane()
		    })
	}
	
	fn tick_process(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.process_mode() != TweenProcessMode::IDLE
			|| self.pause_mode() == TweenPauseMode::PROCESS
			|| (self.pause_mode() == TweenPauseMode::BOUND
			&& self.bound_node()
			       .is_some_and(|node| unsafe { 
				       !node.assume_safe().is_processing() })) {
			return;
		}

		self.advance_time(delta_time);
	}
	
	fn tick_physics(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.process_mode() != TweenProcessMode::PHYSICS
			|| self.pause_mode() == TweenPauseMode::PROCESS
			|| (self.pause_mode() == TweenPauseMode::BOUND
			&& self.bound_node()
			       .is_some_and(|node| unsafe { 
				       !node.assume_safe().is_physics_processing() })) {
			return;
		}

		self.advance_time(delta_time);
	}
	
	fn tick_independent(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.pause_mode() != TweenPauseMode::PROCESS {
			return;
		}

		self.advance_time(delta_time);
	}
}

#[enum_dispatch(Tick)]
#[derive(Debug)]
pub enum AnyTween {
	Property(TweenProperty),
	Method(TweenMethod),
	Callback(TweenCallback),
}

pub trait FromTween {
	fn from_tween(tween: &AnyTween) -> Option<&Self>;
	fn from_tween_mut(tween: &mut AnyTween) -> Option<&mut Self>;
}

pub(crate) fn ratio_with_delay_duration(delay: f64, duration: f64, elapsed_time: f64) -> f64 {
	f64::max((elapsed_time - delay) / duration, 0.)
}