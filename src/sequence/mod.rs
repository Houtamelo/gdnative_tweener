pub mod id;
pub use id::*;

#[allow(unused_imports)]
use crate::*;

#[derive(Debug)]
pub struct Sequence {
	pub queue: Vec<Vec<ForkElement>>,
	pub inserteds: Vec<(f64, InsertedElement)>,
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
pub enum ForkElement {
	Tween(AnyTween),
	Sequence(Sequence),
	Callback { invoked: bool, callback: Callback },
	Interval { total_time: f64, elapsed_time: f64 },
}

impl From<AnyTween> for ForkElement {
	fn from(value: AnyTween) -> Self {
		Self::Tween(value)
	}
}

#[derive(Debug)]
pub enum InsertedElement {
	Tween(AnyTween),
	Sequence(Sequence),
	Callback { invoked: bool, callback: Callback },
}

impl From<AnyTween> for InsertedElement {
	fn from(value: AnyTween) -> Self {
		Self::Tween(value)
	}
}

impl Sequence {
	pub fn new() -> Self {
		Self {
			queue: Vec::new(),
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
		
		match self.state {
			| State::Playing
			| State::Paused => {
				tween.pause();
			}
			State::Stopped => {
				tween.stop();
			}
		}
		
		self.queue.push(vec![tween.into()]);
	}
	
	pub fn append_sequence(&mut self, mut sequence: Sequence) {
		match self.state {
			| State::Playing
			| State::Paused => {
				sequence.pause();
			}
			State::Stopped => {
				sequence.stop();
			}
		}
		
		self.queue.push(vec![ForkElement::Sequence(sequence)]);
	}
	
	pub fn append_call(&mut self, f: impl Fn() + 'static) {
		let callback = Callback::Closure(Box::new(f));
		self.queue.push(vec![ForkElement::Callback { invoked: false, callback }]);
	}
	
	pub fn append_method(
		&mut self,
		target: &impl Inherits<Object>,
		method: impl Into<GodotString>,
		args: Vec<Variant>,
	) {
		let callback = Callback::GodotMethodCall(GodotMethodCall {
			target: unsafe { target.base() },
			method: method.into(),
			args,
		});

		self.queue.push(vec![ForkElement::Callback { invoked: false, callback }]);
	}
	
	pub fn append_interval(&mut self, time: f64) {
		self.queue.push(vec![ForkElement::Interval { total_time: time, elapsed_time: 0. }]);
	}
	
	pub fn append_many_in_parallel(&mut self, tweens: impl IntoIterator<Item: Into<AnyTween>>) {
		let mut added_first = false;
		
		for tween in tweens {
			let tween = tween.into();
			
			if added_first {
				self.join(tween);
			} else {
				self.append(tween);
				added_first = true;
			}
		}
	}
	
	pub fn join(&mut self, any_tween: impl Into<AnyTween>) {
		let mut tween = any_tween.into();

		match self.state {
			| State::Playing
			| State::Paused => {
				tween.pause();
			}
			State::Stopped => {
				tween.stop();
			}
		}
		
		if let Some(back) = self.queue.last_mut() {
			back.push(tween.into());
		} else {
			self.append(tween);
		}
	}
	
	pub fn join_sequence(&mut self, mut sequence: Sequence) {
		match self.state {
			| State::Playing
			| State::Paused => {
				sequence.pause();
			}
			State::Stopped => {
				sequence.stop();
			}
		}
		
		if let Some(back) = self.queue.last_mut() {
			back.push(ForkElement::Sequence(sequence));
		} else {
			self.queue.push(vec![ForkElement::Sequence(sequence)]);
		}
	}
	
	pub fn join_call(&mut self, f: impl Fn() + 'static) {
		let callback = Callback::Closure(Box::new(f));
		
		if let Some(back) = self.queue.last_mut() {
			back.push(ForkElement::Callback { invoked: false, callback });
		} else {
			self.queue.push(vec![ForkElement::Callback { invoked: false, callback }]);
		}
	}
	
	pub fn join_method(
		&mut self,
		target: &impl Inherits<Object>,
		method: impl Into<GodotString>,
		args: Vec<Variant>) {
		let callback = Callback::GodotMethodCall(GodotMethodCall {
			target: unsafe { target.base() },
			method: method.into(),
			args,
		});

		if let Some(back) = self.queue.last_mut() {
			back.push(ForkElement::Callback { invoked: false, callback });
		} else {
			self.queue.push(vec![ForkElement::Callback { invoked: false, callback }]);
		}
	}
	
	pub fn join_many(&mut self, tweens: impl IntoIterator<Item: Into<AnyTween>>) {
		for tween in tweens {
			self.join(tween.into());
		}
	}
	
	pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) {
		let mut tween = tween.into();

		match self.state {
			| State::Playing
			| State::Paused => {
				tween.pause();
			}
			State::Stopped => {
				tween.stop();
			}
		}
		
		self.inserteds.push((time, tween.into()));
	}
	
	pub fn insert_sequence(&mut self, time: f64, mut sequence: Sequence) {
		match self.state {
			| State::Playing
			| State::Paused => {
				sequence.pause();
			}
			State::Stopped => {
				sequence.stop();
			}
		}
		
		self.inserteds.push((time, InsertedElement::Sequence(sequence)));
	}
	
	pub fn insert_call(&mut self, time: f64, f: impl Fn() + 'static) {
		let callback = Callback::Closure(Box::new(f));
		self.inserteds.push((time, InsertedElement::Callback { invoked: false, callback }));
	}
	
	pub fn insert_method(
		&mut self,
		time: f64,
		target: &impl Inherits<Object>,
		method: impl Into<GodotString>,
		args: Vec<Variant>) {
		let callback = Callback::GodotMethodCall(GodotMethodCall {
			target: unsafe { target.base() },
			method: method.into(),
			args,
		});

		self.inserteds.push((time, InsertedElement::Callback { invoked: false, callback }));
	}

	pub fn bound_to(self, node: &impl Inherits<Node>) -> Self {
		Self { bound_node: Some(unsafe { node.base() }), ..self }
	}
	
	pub fn unbound(self) -> Self {
		Self { bound_node: None, ..self }
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

	pub fn call_when_finished(mut self, f: impl Fn() + 'static) -> Self {
		let closure = Callback::Closure(Box::new(f));
		self.do_on_finish.push(closure);
		self
	}

	pub fn method_when_finished(
		mut self,
		target: &impl Inherits<Object>,
		method: impl Into<GodotString>,
		args: Vec<Variant>)
		-> Self {
		let method_call = Callback::GodotMethodCall(GodotMethodCall {
			target: unsafe { target.base() },
			method: method.into(),
			args,
		});

		self.do_on_finish.push(method_call);
		self
	}
	
	pub fn play(&mut self) {
		if self.state == State::Playing {
			return;
		}
		
		let from_begin = self.state == State::Stopped;
		
		self.state = State::Playing;

		self.queue
		    .iter_mut()
		    .flat_map(|vec| vec.iter_mut())
		    .for_each(|fork_element| {
			    match fork_element {
				    ForkElement::Tween(tween) => {
					    if from_begin {
						    tween.stop();
						    tween.pause();
					    } else {
						    tween.pause();
					    }
				    }
				    ForkElement::Sequence(seq) => {
					    if from_begin {
						    seq.stop();
						    seq.pause();
					    } else {
						    seq.pause();
					    }
				    }
				    ForkElement::Callback { invoked, .. } => {
					    if from_begin {
						    *invoked = false;
					    }
				    }
				    ForkElement::Interval { elapsed_time, .. } => {
					    if from_begin {
						    *elapsed_time = 0.;
					    }
				    }
			    }
		    });

		self.inserteds
		    .iter_mut()
		    .for_each(|(_, inserted_element)| {
			    match inserted_element {
				    InsertedElement::Tween(tween) => {
					    if from_begin {
						    tween.stop();
						    tween.pause();
					    } else {
						    tween.pause();
					    }
				    }
				    InsertedElement::Sequence(seq) => {
					    if from_begin {
						    seq.stop();
						    seq.pause();
					    } else {
						    seq.pause();
					    }
				    }
				    InsertedElement::Callback { invoked, .. } => {
					    if from_begin {
						    *invoked = false;
					    }
				    }
			    }
		    });
	}
	
	pub fn pause(&mut self) {
		self.state = State::Paused;
	}

	pub fn stop(&mut self) {
		if self.state == State::Stopped {
			return;
		}
		
		self.state = State::Stopped;
		self.total_elapsed_time = 0.0;
		
		self.queue
			.iter_mut()
			.flat_map(|vec| vec.iter_mut())
			.for_each(|fork_element| {
				match fork_element {
					ForkElement::Tween(tween) => {
						tween.stop();
					}
					ForkElement::Sequence(seq) => {
						seq.stop();
					}
					ForkElement::Callback { invoked, .. } => {
						*invoked = false;
					}
					ForkElement::Interval { elapsed_time, .. } => {
						*elapsed_time = 0.;
					}
				}
			});
		
		self.inserteds
			.iter_mut()
			.for_each(|(_, inserted_element)| {
				match inserted_element {
					InsertedElement::Tween(tween) => {
						tween.stop();
					}
					InsertedElement::Sequence(seq) => {
						seq.stop();
					}
					InsertedElement::Callback { invoked, .. } => {
						*invoked = false;
					}
				}
			});
	}

	pub fn advance_time(&mut self, delta_time: f64) -> Option<f64> {
		let delta_time = delta_time * self.speed_scale;
		self.total_elapsed_time += delta_time;
		
		for (at, inserted_element) in self.inserteds.iter_mut() {
			match inserted_element {
				InsertedElement::Tween(tween) => {
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
				InsertedElement::Sequence(seq) => {
					match seq.state {
						State::Playing => {
							seq.advance_time(delta_time);
						}
						State::Paused => {
							if *at <= self.total_elapsed_time {
								let above_at = self.total_elapsed_time - *at;
								seq.play();
								seq.advance_time(above_at);
							}
						}
						State::Stopped => {}
					}
				}
				InsertedElement::Callback { invoked, callback } => {
					if *invoked == false && *at >= self.total_elapsed_time {
						*invoked = true;
						unsafe { callback.invoke().log_if_err() };
					}
				}
			}
		}

		let mut remaining_delta = delta_time;
		let mut queue_iter = self.queue.iter_mut();
		
		while let Some(fork) = queue_iter.next() && remaining_delta > 0. {
			remaining_delta =
				fork.iter_mut()
				    .filter_map(|fork_element| {
					    match fork_element {
						    ForkElement::Tween(tween) => {
							    match tween.state() {
								    State::Playing => {
									    tween.advance_time(remaining_delta)
								    }
								    State::Paused => {
									    tween.play();
									    tween.advance_time(remaining_delta)
								    }
								    State::Stopped => Some(remaining_delta),
							    }
						    }
						    ForkElement::Sequence(seq) => {
							    match seq.state {
								    State::Playing => {
									    seq.advance_time(remaining_delta)
								    }
								    State::Paused => {
									    seq.play();
									    seq.advance_time(remaining_delta)
								    }
								    State::Stopped => Some(remaining_delta),
							    }
						    }
						    ForkElement::Callback { invoked, callback } => {
							    if *invoked == false {
								    *invoked = true;
								    unsafe { callback.invoke().log_if_err() };
							    }

							    Some(remaining_delta)
						    }
						    ForkElement::Interval { total_time, elapsed_time } => {
							    *elapsed_time += remaining_delta;
							    
							    let above_total = *elapsed_time - *total_time;
							    (above_total > 0.).then_some(above_total)							    
						    }
					    }
				    }).min_by(f64::total_cmp).unwrap_or(-1.);
		}
		
		if remaining_delta > 0. {
			self.on_finish();
		}
		
		(remaining_delta > 0.).then_some(remaining_delta)
	}

	pub fn force_finish(mut self) {
		self.queue
			.drain(..)
			.for_each(|fork| { 
				fork.into_iter()
					.for_each(|fork_element| {
						match fork_element {
							ForkElement::Tween(tween) => {
								tween.force_finish();
							}
							ForkElement::Sequence(seq) => {
								seq.force_finish();
							}
							ForkElement::Callback { invoked, callback } => {
								if invoked == false {
									unsafe { callback.invoke().log_if_err() };
								}
							}
							ForkElement::Interval { .. } => {}
						}
					})
			});
		
		self.inserteds
			.drain(..)
			.for_each(|(_, inserted_element)| {
				match inserted_element {
					InsertedElement::Tween(tween) => {
						tween.force_finish()
					}
					InsertedElement::Sequence(seq) => {
						seq.force_finish()
					}
					InsertedElement::Callback { invoked, callback } => {
						if invoked == false {
							unsafe { callback.invoke().log_if_err() };
						}
					}
				}
			});
		
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