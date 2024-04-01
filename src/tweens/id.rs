#[allow(unused_imports)]
use crate::*;

use std::fmt::{Display, Formatter};
use crate::id::{ID, WeakID};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct TweenID<T: Tick + FromTween>(pub WeakID, PhantomData<T>);

#[derive(Debug, Clone)]
pub struct TweenID_Variant<TTween: Tick + FromTween,
                           TVar: _Lerp + ToVariant + FromVariant + Clone + Copy> {
	pub id: TweenID<TTween>,
	type_hint: PhantomData<TVar>,
}

impl<T: Tick + FromTween> Display for TweenID<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "TweenID({:?})", self.0)
	}
}

impl<T: Tick + FromTween> TweenID<T> {
	pub fn new(id: WeakID) -> Self { Self(id, PhantomData) }

	pub fn is_valid(&self) -> bool {
		TweensBrain::singleton()
			.try_borrow()
			.is_ok()
	}

	pub fn kill(&self) -> Result<()> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Tween with id `{}` no longer exists.", self))?;
		
		let brain =
			&mut TweensBrain::singleton().try_borrow_mut()?;

		brain.kill_tween(ID(id));
		Ok(())
	}

	pub fn complete(&self) -> Result<()> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Tween with id `{}` no longer exists.", self))?;
		
		let brain =
			&mut TweensBrain::singleton().try_borrow_mut()?;

		unsafe { brain.complete_tween(ID(id)) };
		Ok(())
	}

	pub fn map<TMap>(&self, f: impl FnOnce(&T) -> TMap) -> Result<TMap> {
		let id = 
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Tween with id `{}` no longer exists.", self))?;
		
		let brain = 
			TweensBrain::singleton().try_borrow()?;

		let tween =
			brain.get_tween(ID(id))
			     .ok_or_else(|| anyhow!("Tween with id `{}` no longer exists.", * self ))?;
	
		if let Some(self_tween) = T::from_tween(tween) {
			Ok(f(self_tween))
		} else {
			Err(anyhow!("Expected a tween of type `{}`.\n\
						 Got: {:?}", type_name::<T>(), tween))
		}
	}

	pub fn map_mut<TMap>(&self, f: impl FnOnce(&mut T) -> TMap) -> Result<TMap> {
		let id =
			Weak::upgrade(&self.0.0)
				.ok_or_else(|| anyhow!(
					"Tween with id `{}` no longer exists.", self))?;
		
		let brain =
			&mut TweensBrain::singleton().try_borrow_mut()?;

		let tween =
			brain.get_tween_mut(ID(id))
			     .ok_or_else(|| anyhow!("Tween with id `{}` no longer exists.", * self ))?;
		
		if let Some(self_tween) = T::from_tween_mut(tween) {
			Ok(f(self_tween))
		} else {
			Err(anyhow!("Expected a tween of type `{}`.\n\
						 Got: {:?}", type_name::<T>(), tween))
		}
	}

	pub fn play(&self) -> Result<()> {
		self.map_mut(|tween| { tween.play(); })
	}

	pub fn pause(&self) -> Result<()> {
		self.map_mut(|tween| { tween.pause(); })
	}

	pub fn stop(&self) -> Result<()> {
		self.map_mut(|tween| { tween.stop(); })
	}

	/*
	pub fn do_absolute_step(&self, delta: f64) -> Result<()> {
		self.map_mut(|tween| { tween.do_absolute_step(delta) })
	}

	pub fn do_scaled_step(&self, delta: f64) -> Result<()> {
		self.map_mut(|tween| { tween.do_scaled_step(delta) })
	}

	pub fn seek(&self, time: f64) -> Result<()> {
		self.map_mut(|tween| { tween.seek(time) })
	}
	*/
}