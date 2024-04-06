#![allow(unused)]
//todo! Under construction

#[allow(unused_imports)]
use crate::*;
use std::fmt::{Display, Formatter};
use std::iter;
use crate::id::WeakID;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub(crate) struct SequenceID(pub WeakID);

impl Display for SequenceID {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "SequenceID({:?})", self.0)
	}
}

#[derive(Debug)]
pub(crate) struct Sequence {
	pub mains: Vec<TweenFork>,
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
	current_index: usize,
	current_elapsed_time: f64,
}

#[derive(Debug)]
pub(crate) struct TweenFork {
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
	
	fn max_duration(&self) -> f64 {
		self.iter()
		    .map(|tween| {
			    match tween.total_duration() {
				    Duration::Infinite => f32::MAX as f64,
				    Duration::Finite(duration) => duration,
			    }
		    })
		    .max_by(|x, y| x.total_cmp(y))
		    .unwrap_or(0.)
	}

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
			mains: Vec::new(),
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
			current_index: 0,
			current_elapsed_time: 0.,
		}
	}

	pub fn register(self) -> Result<SequenceID> {
		let singleton =
			&mut TweensBrain::singleton().try_borrow_mut()?;

		let id = singleton.register_sequence(self.into());
		Ok(id)
	}

	pub fn append(&mut self, tween: impl Into<AnyTween>) {
		self.mains.push(TweenFork::new(tween.into()));
	}
	
	pub fn join(&mut self, tween: impl Into<AnyTween>) {
		if let Some(back) = self.mains.last_mut() {
			back.parallels.push(tween.into());
		} else {
			self.append(tween);
		}
	}
	
	pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) {
		self.inserteds.push((time, tween.into()));
	}

	pub fn bound_to(&mut self, node: &impl Inherits<Node>) -> &mut Self {
		self.bound_node = Some(unsafe { node.base() });
		self
	}

	pub fn with_delay(&mut self, delay: f64) -> &mut Self {
		self.delay = delay;
		self
	}

	pub fn with_speed_scale(&mut self, speed_scale: f64) -> &mut Self {
		self.speed_scale = speed_scale;
		self
	}

	pub fn with_pause_mode(&mut self, pause_mode: TweenPauseMode) -> &mut Self {
		self.pause_mode = pause_mode;
		self
	}

	pub fn with_process_mode(&mut self, process_mode: TweenProcessMode) -> &mut Self {
		self.process_mode = process_mode;
		self
	}

	pub fn run_once(&mut self) -> &mut Self {
		self.loop_mode = LoopMode::Finite(0);
		self
	}

	pub fn looped(&mut self, loops: u32) -> &mut Self {
		self.loop_mode = LoopMode::Finite(loops);
		self
	}

	pub fn infinite(&mut self) -> &mut Self {
		self.loop_mode = LoopMode::Infinite;
		self
	}

	pub fn when_finished(&mut self,
	                     method: Rc<String>,
	                     target: &impl Inherits<Object>,
	                     args: Vec<Variant>)
	                     -> &mut Self {
		let callback = Callback {
			target: unsafe { target.base() },
			method,
			args,
		};
		
		self.do_on_finish.push(callback);
		self
	}
}

impl Tick for Sequence {
	fn state(&self) -> State { self.state }
	fn play(&mut self) { self.state = State::Playing; }
	fn pause(&mut self) { self.state = State::Paused; }
	
	fn stop(&mut self) {
		self.state = State::Stopped;
		self.total_elapsed_time = 0.0;
		self.current_index = 0;
		self.current_elapsed_time = 0.;
	}

	fn process_mode(&self) -> TweenProcessMode { self.process_mode }
	fn pause_mode(&self) -> TweenPauseMode { self.pause_mode }
	fn bound_node(&self) -> Option<&Ref<Node>> { self.bound_node.as_ref() }
	
	fn loop_mode(&self) -> LoopMode { self.loop_mode }
	
	fn cycle_duration(&self) -> f64 {
		self.mains
		    .iter()
		    .map(TweenFork::max_duration).sum::<f64>()
	}

	fn delay(&self) -> f64 { self.delay }
	fn elapsed_time(&self) -> f64 { self.total_elapsed_time }
	fn speed_scale(&self) -> f64 { self.speed_scale }
	
	fn advance_time(&mut self, delta_time: f64) -> f64 {
		let delta_time = delta_time * self.speed_scale;
		self.total_elapsed_time += delta_time;
		self.current_elapsed_time += delta_time;
		
		for (at, tween) in self.inserteds.iter_mut() {
			//todo!()
		}

		let mut remaining_delta = delta_time;
		
		for fork in self.mains.iter_mut() {
			if remaining_delta <= 0. {
				break;
			}
			
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
		
		remaining_delta
	}

	fn callbacks_on_finish(&self) -> &[Callback] { &self.do_on_finish }

	fn force_finish(mut self) {
		self.mains
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
}