#[allow(unused_imports)]
use crate::*;

use lazy_static::lazy_static;
use gdnative::api::node::PauseMode;
use crate::id::{ID, WeakID};

#[derive(NativeClass)]
#[user_data(GoodCellData<TweensBrain>)]
#[inherit(Node)]
pub struct TweensBrain {
	tweens: HashMap<ID, AnyTween>,
	sequences: HashMap<ID, Sequence>,
}

const SINGLETON_NAME: &str = "tweens_controller";

lazy_static! {
	static ref SINGLETON: Instance<TweensBrain> = unsafe {
		let object: TRef<'static, Object> =
			Engine::godot_singleton()
				.get_singleton(SINGLETON_NAME)
				.expect(format!("Singleton with name `{SINGLETON_NAME}` does not exist").as_str())
				.assume_safe();
		
		let node: TRef<'static, Node> = 
			object.cast::<Node>()
				  .expect(format!("Singleton with name `{SINGLETON_NAME}` is not a Node").as_str());
		
		node.set_pause_mode(PauseMode::PROCESS.into());
		
		node.cast_instance::<TweensBrain>()
		    .expect(format!("Singleton with name `{SINGLETON_NAME}` is not a TweensController").as_str())
		    .claim()
	};
}

#[methods]
impl TweensBrain {
	fn new(_owner: &Node) -> Self {
		Self {
			tweens: HashMap::new(),
			sequences: HashMap::new(),
		}
	}
	
	#[method]
	unsafe fn _process(&mut self, #[base] owner: &Node, delta_time: f64) {
		let Some(tree_ref) = owner.get_tree()
			else {
				godot_error!("TweensBrain::_process: owner has no SceneTree. Owner name: {}", owner.name());
				return;
			};
		
		let Some(tree) = tree_ref.assume_safe_if_sane()
			else {
				godot_error!("TweensBrain::_process: owner's SceneTree is not sane. Owner name: {}", owner.name());
				return;
			};
		
		self.tick_process(delta_time, tree.is_paused());
	}

	#[method]
	unsafe fn _physics_process(&mut self, delta_time: f64) {
		self.tick_physics(delta_time);
	}
}

impl TweensBrain {
	unsafe fn tick_process(&mut self, delta_time: f64, is_tree_paused: bool) {
		self.tweens
		    .retain(|_, tween| {
			    if is_tree_paused {
				    tween.tick_independent(delta_time);
			    } else {
				    tween.tick_process(delta_time);
			    }

			    match tween.state() {
				    | State::Playing
				    | State::Paused => { true }
				    State::Stopped => { false }
			    }
		    });
		
		self.sequences
		    .retain(|_, sequence| {
			    if is_tree_paused {
				    sequence.tick_independent(delta_time);
			    } else {
				    sequence.tick_process(delta_time);
			    }

			    match sequence.state() {
				    | State::Playing
				    | State::Paused => { true }
				    State::Stopped => { false }
			    }
		    });
	}

	unsafe fn tick_physics(&mut self, delta_time: f64) {
		self.tweens
		    .retain(|_, tween| {
			    tween.tick_physics(delta_time);
			    
			    match tween.state() {
				    | State::Playing
				    | State::Paused => { true }
				    State::Stopped => { false }
			    }
		    });
	
		self.sequences
		    .retain(|_, sequence| {
			    sequence.tick_physics(delta_time);

			    match sequence.state() {
				    | State::Playing
				    | State::Paused => { true }
				    State::Stopped => { false }
			    }
		    });
	}
	
	pub fn singleton() -> &'static GoodCellData<TweensBrain> {
		SINGLETON.script()
	}

	pub fn get_tween(&self, id: ID) -> Option<&AnyTween> {
		self.tweens.get(&id)
	}
	
	pub fn get_tween_mut(&mut self, id: ID) -> Option<&mut AnyTween> {
		self.tweens.get_mut(&id)
	}
	
	pub fn claim_tween(&mut self, id: ID) -> Option<AnyTween> {
		self.tweens.remove(&id)
	}
	
	pub fn kill_tween(&mut self, id: ID) {
		self.tweens.remove(&id);
	}
	
	pub unsafe fn complete_tween(&mut self, id: ID) {
		self.tweens
			.remove(&id)
			.map(|tween| {
				tween.force_finish();
			});
	}
	
	#[allow(unused)]
	pub(crate) unsafe fn complete_sequence(&mut self, id: ID) {
		self.sequences
		    .remove(&id)
		    .map(|sequence| {
			    sequence.force_finish();
		    });
	}
	
	pub fn register_tween<T: Tick + FromTween>(&mut self, tween: impl Into<AnyTween>) -> TweenID<T> {
		loop {
			let rc = Rc::new(());
			let id = ID(Rc::clone(&rc));
			self.tweens.insert(id, tween.into());
			
			let weak_id = WeakID(Rc::downgrade(&rc));
			return TweenID::new(weak_id);
		}
	} 
	
	pub(crate) fn register_sequence(&mut self, sequence: Sequence) -> SequenceID {
		loop {
			let rc = Rc::new(());
			let id = ID(Rc::clone(&rc));
			self.sequences.insert(id, sequence);
			
			let weak_id = WeakID(Rc::downgrade(&rc));
			return SequenceID(weak_id);
		}
	}
}