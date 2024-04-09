#[allow(unused_imports)]
use crate::*;

#[allow(unused)]
macro_rules! base_impl {
    ($struct_ty: ident) => {
		impl $struct_ty {
			pub fn bound_to(self, node: &impl Inherits<Node>) -> Self {
				Self { bound_node: Some(unsafe { node.base() }), ..self }
			}
			
			pub fn unbound(self) -> Self {
				Self { bound_node: None, ..self }
			}
		    
		    pub fn with_delay(self, delay: f64) -> Self { 
			    Self { delay, ..self }
		    }
		    
			pub fn with_speed_scale(self, speed_scale: f64) -> Self  { 
			    Self { speed_scale, ..self }
		    }
		    
		    pub fn with_pause_mode(self, pause_mode: TweenPauseMode) -> Self  { 
			    Self { pause_mode, ..self }
		    }
		    
			pub fn with_process_mode(self, process_mode: TweenProcessMode) -> Self  { 
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
								 target: &impl Inherits<Object>,
								 method: impl Into<GodotString>,
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
		}
		
		impl Tick for $struct_ty {
			fn state(&self) -> State { self.state }
			
			fn play(&mut self) {
				self.set_state_internal(State::Playing);
			}
		
			fn pause(&mut self) {
				self.set_state_internal(State::Paused);
			}
		
			fn stop(&mut self) {
				self.set_state_internal(State::Stopped);
			}
			
			fn process_mode(&self) -> TweenProcessMode { self.process_mode }
			fn pause_mode(&self) -> TweenPauseMode { self.pause_mode }
			fn bound_node(&self) -> Option<&Ref<Node>> { self.bound_node.as_ref() }
			fn loop_mode(&self) -> LoopMode { self.loop_mode }
			fn delay(&self) -> f64 { self.delay }
			
			fn elapsed_time(&self) -> f64 { self.elapsed_time }
			fn speed_scale(&self) -> f64 { self.speed_scale }
		
			fn advance_time(&mut self, delta_time: f64) -> Option<f64> {
				match self.advance_time_internal(delta_time) {
					Ok(excess) => excess,
					Err(err) => {
						godot_error!("{err}");
						Some(delta_time)
					}
				}
			}
		
			fn callbacks_on_finish(&self) -> &[Callback] { &self.do_on_finish }
		
			fn force_finish(mut self) {
				match self.state() {
					| State::Playing
					| State::Paused => {
						self.seek_end();
					}
					State::Stopped => {}
				}
			}
		}
    };
}

pub(crate) use base_impl;