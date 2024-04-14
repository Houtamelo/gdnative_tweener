#[allow(unused_imports)]
use crate::*;
use std::fmt::{Display, Formatter};
use crate::id::{ID, WeakID};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct SequenceID(pub WeakID);

impl SequenceID {
	pub fn is_valid(&self) -> bool {
		TweensController::singleton()
			.try_borrow()
			.is_ok_and(|brain| {
				let weak = self.0.clone();
				weak.0.upgrade()
				    .is_some_and(|id| {
					    brain.get_sequence(ID(id)).is_some()
				    })
			})
	}

	pub fn kill(&self) -> Result<()> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Sequence with id `{}` no longer exists.", self))?;

		let brain =
			&mut TweensController::singleton().try_borrow_mut()?;

		brain.kill_sequence(ID(id));
		Ok(())
	}

	pub fn complete(&self) -> Result<()> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Sequence with id `{}` no longer exists.", self))?;

		let brain =
			&mut TweensController::singleton().try_borrow_mut()?;

		unsafe { brain.complete_tween(ID(id)) };
		Ok(())
	}

	pub fn play(&self) -> Result<()> {
		self.map_mut(|sequence| { sequence.play(); })
	}

	pub fn pause(&self) -> Result<()> {
		self.map_mut(|sequence| { sequence.pause(); })
	}

	pub fn stop(&self) -> Result<()> {
		self.map_mut(|sequence| { sequence.stop(); })
	}
	
	pub fn map<TMap>(&self, f: impl FnOnce(&Sequence) -> TMap) -> Result<TMap> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Tween with id `{}` no longer exists.", self))?;

		let brain =
			TweensController::singleton().try_borrow()?;

		brain.get_sequence(ID(id))
		     .ok_or_else(|| anyhow!("Tween with id `{}` no longer exists.", * self ))
		     .map(f)
	}

	pub fn map_mut<TMap>(&self, f: impl FnOnce(&mut Sequence) -> TMap) -> Result<TMap> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Tween with id `{}` no longer exists.", self))?;

		let brain =
			&mut TweensController::singleton().try_borrow_mut()?;

		brain.get_sequence_mut(ID(id))
		     .ok_or_else(|| anyhow!("Tween with id `{}` no longer exists.", * self ))
		     .map(f)
	}
}

impl Display for SequenceID {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "SequenceID({:?})", self.0)
	}
}