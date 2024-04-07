#[allow(unused_imports)]
use crate::*;
use std::fmt::{Display, Formatter};
use std::iter;
use crate::id::WeakID;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct SequenceID(pub WeakID);

impl Display for SequenceID {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "SequenceID({:?})", self.0)
	}
}

#[derive(Debug)]
pub struct Sequence {
	pub tweens: Vec<TweenFork>,
	pub inserteds: Vec<(f64, AnyTween)>,
	pub bound_node: Option<Ref<Node>>,
	pub state: State,
	pub delay: f64,
	pub speed_scale: f64,
	pub total_elapsed_time: f64,
	pub pause_mode: TweenPauseMode,
	pub process_mode: TweenProcessMode,
	pub loop_mode: LoopMode,
	pub do_on_finish: Vec<Callback>,
}

#[derive(Debug)]
pub struct TweenFork {
	pub main: AnyTween,
	pub parallels: Vec<AnyTween>,
}

impl TweenFork {
	pub fn new(main: AnyTween) -> Self {
		Self {
			main,
			parallels: Vec::new(),
		}
	}
	
	#[allow(unused)]
	fn iter(&self) -> impl Iterator<Item = &AnyTween> {
		iter::once(&self.main)
			.chain(self.parallels.iter())
	}
	
	fn iter_mut(&mut self) -> impl Iterator<Item = &mut AnyTween> {
		iter::once(&mut self.main)
			.chain(self.parallels.iter_mut())
	}
	
	fn into_iter(self) -> impl Iterator<Item = AnyTween> {
		iter::once(self.main)
			.chain(self.parallels.into_iter())
	}
}

impl Sequence {
	pub fn new() -> Self {
		Self {
			tweens: Vec::new(),
			inserteds: Vec::new(),
			bound_node: None,
			state: State::Playing,
			delay: 0.,
			speed_scale: 1.,
			total_elapsed_time: 0.,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			do_on_finish: Vec::new(),
		}
	}

	pub fn register(self) -> Result<SequenceID> {
		let singleton =
			&mut TweensController::singleton().try_borrow_mut()?;

		let id = singleton.register_sequence(self.into());
		Ok(id)
	}

	pub fn append(&mut self, any_tween: impl Into<AnyTween>) {
		let mut tween = any_tween.into();
		tween.pause();
		self.tweens.push(TweenFork::new(tween));
	}
	
	pub fn join(&mut self, any_tween: impl Into<AnyTween>) {
		let mut tween = any_tween.into();
		tween.pause();
		if let Some(back) = self.tweens.last_mut() {
			back.parallels.push(tween);
		} else {
			self.append(tween);
		}
	}
	
	pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) {
		self.inserteds.push((time, tween.into()));
	}

	pub fn bound_to(self, node: &impl Inherits<Node>) -> Self {
		Self { bound_node: Some(unsafe { node.base() }), ..self }
	}

	pub fn with_delay(self, delay: f64) -> Self {
		Self { delay, ..self }
	}

	pub fn with_speed_scale(self, speed_scale: f64) -> Self {
		Self { speed_scale, ..self }
	}

	pub fn with_pause_mode(self, pause_mode: TweenPauseMode) -> Self {
		Self { pause_mode, ..self }
	}

	pub fn with_process_mode(self, process_mode: TweenProcessMode) -> Self {
		Self { process_mode, ..self }
	}

	pub fn run_once(self) -> Self {
		Self { loop_mode: LoopMode::Finite(0), ..self }
	}

	pub fn looped(self, loops: u32) -> Self {
		Self { loop_mode: LoopMode::Finite(loops), ..self }
	}

	pub fn infinite(self) -> Self {
		Self { loop_mode: LoopMode::Infinite, ..self }
	}

	pub fn when_finished(mut self,
	                     method: impl Into<GodotString>,
	                     target: &impl Inherits<Object>,
	                     args: Vec<Variant>)
	                     -> Self {
		let callback = Callback {
			target: unsafe { target.base() },
			method: method.into(),
			args,
		};
		
		self.do_on_finish.push(callback);
		self
	}
	
	pub fn play(&mut self) { self.state = State::Playing; }
	pub fn pause(&mut self) { self.state = State::Paused; }

	pub fn stop(&mut self) {
		self.state = State::Stopped;
		self.total_elapsed_time = 0.0;
		
		self.tweens
			.iter_mut()
			.flat_map(TweenFork::iter_mut)
			.for_each(|tween| {
				tween.stop();
				tween.pause();
			});
		
		self.inserteds
			.iter_mut()
			.for_each(|(_, tween)| {
				tween.stop();
				tween.pause();
			});
	}

	pub fn advance_time(&mut self, delta_time: f64) {
		let delta_time = delta_time * self.speed_scale;
		self.total_elapsed_time += delta_time;
		
		for (at, tween) in self.inserteds.iter_mut() {
			match tween.state() {
				State::Playing => {
					tween.advance_time(delta_time);
				}
				State::Paused => {
					if *at <= self.total_elapsed_time {
						let above_at = self.total_elapsed_time - *at;
						tween.play();
						tween.advance_time(above_at);
					}
				}
				State::Stopped => {}
			}
		}

		let mut remaining_delta = delta_time;
		let mut mains_iter = self.tweens.iter_mut();
		
		while let Some(fork) = mains_iter.next()
			&& remaining_delta > 0. {
			remaining_delta =
				fork.iter_mut()
				    .map(|tween| {
					    match tween.state() {
						    State::Playing => {
							    tween.advance_time(remaining_delta)
						    }
						    State::Paused => {
							    tween.play();
							    tween.advance_time(remaining_delta)
						    }
						    State::Stopped => remaining_delta,
					    } 
				    }).min_by(f64::total_cmp)
					.unwrap_or(remaining_delta);
		}
		
		if remaining_delta > 0. {
			self.on_finish();
		} 
	}

	pub fn force_finish(mut self) {
		self.tweens
			.drain(..)
			.for_each(|fork| { 
				fork.into_iter()
					.for_each(AnyTween::force_finish)
			});
		
		self.inserteds
			.drain(..)
			.for_each(|(_, tween)| tween.force_finish());
		
		self.on_finish();
	}

	pub fn on_finish(&mut self) {
		self.stop();

		self.do_on_finish
		    .iter()
		    .for_each(|callback| unsafe {
			    callback.invoke().log_if_err();
		    });
	}

	fn is_bounded_dead(&self) -> bool {
		self.bound_node
		    .is_some_and(|node| unsafe {
			    !node.is_instance_sane()
		    })
	}

	pub fn tick_process(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.process_mode != TweenProcessMode::IDLE
			|| self.pause_mode == TweenPauseMode::PROCESS
			|| (self.pause_mode == TweenPauseMode::BOUND
			&& self.bound_node
			       .is_some_and(|node| unsafe {
				       !node.assume_safe().is_processing()
			       })) {
			return;
		}

		self.advance_time(delta_time);
	}

	pub fn tick_physics(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.process_mode != TweenProcessMode::PHYSICS
			|| self.pause_mode == TweenPauseMode::PROCESS
			|| (self.pause_mode == TweenPauseMode::BOUND
			&& self.bound_node
			       .is_some_and(|node| unsafe {
				       !node.assume_safe().is_physics_processing()
			       })) {
			return;
		}

		self.advance_time(delta_time);
	}

	pub fn tick_independent(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.pause_mode != TweenPauseMode::PROCESS {
			return;
		}

		self.advance_time(delta_time);
	}
}