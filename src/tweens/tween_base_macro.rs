#[allow(unused_imports)]
use crate::*;

#[allow(unused)]
macro_rules! base_impl {
    ($struct_ty: ident) => {
		impl $struct_ty {
			pub fn bound_to(&mut self, node: &impl Inherits<Node>) -> &mut Self {
				self.bound_node = Some(unsafe { node.base() });
			    self
			}
		    
		    pub fn with_delay(&mut self, delay: f64) -> &mut Self { 
			    self.delay = delay;
			    self
		    }
		    
			pub fn with_speed_scale(&mut self, speed_scale: f64) -> &mut Self  { 
			    self.speed_scale = speed_scale;
			    self
		    }
		    
		    pub fn with_pause_mode(&mut self, pause_mode: TweenPauseMode) -> &mut Self  { 
			    self.pause_mode = pause_mode;
			    self
		    }
		    
			pub fn with_process_mode(&mut self, process_mode: TweenProcessMode) -> &mut Self  { 
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
		
		impl Tick for $struct_ty {
			fn state(&self) -> State { self.state }
			
			fn play(&mut self) {
				self.state = State::Playing;
			}
		
			fn pause(&mut self) {
				self.state = State::Paused;
			}
		
			fn stop(&mut self) {
				self.state = State::Stopped;
				self.elapsed_time = 0.0;
				self.cycle_count = 0;
			}
			
			fn process_mode(&self) -> TweenProcessMode { self.process_mode }
			fn pause_mode(&self) -> TweenPauseMode { self.pause_mode }
			fn bound_node(&self) -> Option<&Ref<Node>> { self.bound_node.as_ref() }
			fn loop_mode(&self) -> LoopMode { self.loop_mode }
			fn cycle_duration(&self) -> f64 { self.cycle_duration_internal() }
			fn delay(&self) -> f64 { self.delay }
			
			fn elapsed_time(&self) -> f64 { self.elapsed_time }
			fn speed_scale(&self) -> f64 { self.speed_scale }
		
			fn advance_time(&mut self, delta_time: f64) -> f64 {
				self.elapsed_time += delta_time * self.speed_scale;
				self.check_elapsed_time()
			}
		
			/*
			fn seek(&mut self, time: f64) {
				self.elapsed_time = time;
				self.check_elapsed_time();
			}
			*/
			
		
			fn callbacks_on_finish(&self) -> &[Callback] { &self.do_on_finish }
		
			fn force_finish(mut self) {
				match self.state() {
					| State::Playing
					| State::Paused => {
						self.update_value(1.)
						    .log_if_err();
		
						self.on_finish();
					}
					State::Stopped => {}
				}
			}
		}
    };
}

pub(crate) use base_impl;