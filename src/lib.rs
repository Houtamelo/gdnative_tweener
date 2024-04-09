#![feature(let_chains)]
#![feature(result_flattening)]
#![feature(associated_type_bounds)]
#![feature(hash_extract_if)]
#![feature(trivial_bounds)]
#![feature(slice_take)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(inline_const_pat)]
#![feature(inline_const)]
#![allow(non_camel_case_types)]
#![allow(suspicious_auto_trait_impls)]

mod property;
mod sequence;
mod singleton;
mod ease;
mod tweens;
mod id;

pub mod prelude {
	pub use crate::property::*;
	pub use crate::sequence::*;
	pub use crate::tweens::*;
	pub use crate::singleton::*;
	pub use crate::ease::*;
	pub use crate::lerping::*;
	pub use crate::method::*;
	pub use crate::callback::*;
}

pub trait KillBoundTweens {
	fn kill_bound_tweens(&self) -> Result<()> where Self: Inherits<Node> {
		let node = unsafe { self.base::<Node>() };
		let singleton =
			&mut TweensController::singleton().try_borrow_mut()?;

		singleton.kill_boundeds(node);
		Ok(())
	}
}

impl<T> KillBoundTweens for T { }

pub trait CompleteBoundTweens {
	fn complete_bound_tweens(&self) -> Result<()> where Self: Inherits<Node> {
		let node = unsafe { self.base::<Node>() };
		let singleton =
			&mut TweensController::singleton().try_borrow_mut()?;

		singleton.kill_boundeds(node);
		Ok(())
	}
}

impl<T> CompleteBoundTweens for T { }

#[allow(unused_imports)]
pub(crate) use internal_prelude::*;

#[cfg(feature = "integration_tests")]
mod integration_tests;
mod callback;
mod method;

#[cfg(feature = "integration_tests")]
fn init(handle: InitHandle) {
	handle.add_class::<TweensController>();
	handle.add_class::<integration_tests::Tester>();
}

#[cfg(feature = "integration_tests")]
godot_init!(init);

#[allow(unused_imports)]
mod internal_prelude {
	pub use util_gdnative::prelude::*;
	pub use util::prelude::*;
	pub use crate::prelude::*;
}
