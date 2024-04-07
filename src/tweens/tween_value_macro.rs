#[allow(unused_imports)]
use crate::*;

#[allow(unused)]
macro_rules! value_impl {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn with_duration(self, duration: f64) -> Self { 
				Self { duration, ..self }
			}
		    
		    pub fn with_ease(self, ease: Ease) -> Self { 
				Self { ease, ..self }
			}
		    
		    pub fn starting_at(self, value: $value_ty) -> Self { 
				Self { start: value, ..self }
			}
		    
		    pub fn ending_at(self, value: $value_ty) -> Self {
				Self { end: value, ..self }
			}
		    
		    fn elapsed_ratio(&self) -> f64 {
				f64::max((self.elapsed_time - self.delay) / self.duration, 0.)
			}
			
			fn check_elapsed_time(&mut self) -> f64 {
				let t = self.elapsed_ratio();
				if t <= 0. {
					return 0.;
				}
				
				let (final_t, excess_time) =
					if t < 1. {
						(t, None)
					} else {
						let excess = self.elapsed_time - self.cycle_duration();
						self.elapsed_time = f64::max(self.delay, self.elapsed_time - self.duration);
						self.cycle_count += 1;
					
						match &mut self.loop_mode {
							LoopMode::Infinite => {
								(self.elapsed_ratio(), None)
							}
							LoopMode::Finite(loop_count) => {
								if self.cycle_count < *loop_count {
									(self.elapsed_ratio(), None)
								} else {
									self.elapsed_time = self.cycle_duration();
									(1., Some(excess))
								}
							}
						}
					};
		
				if let Err(err) = self.update_value(final_t) {
					godot_warn!("{}", err);
					self.stop();
					return 0.;
				}
				
				match excess_time {
					Some(excess) => {
						self.on_finish();
						excess
					}
					None => {
						0.
					}
				}
			}
		    
		    fn cycle_duration_internal(&self) -> f64 { self.duration }
	    }
    };
}

pub(crate) use value_impl;