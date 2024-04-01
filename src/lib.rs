#![feature(let_chains)]
#![feature(result_flattening)]
#![feature(associated_type_bounds)]
#![feature(hash_extract_if)]
#![allow(non_camel_case_types)]
#![feature(trivial_bounds)]

mod property;
mod sequence;
mod singleton;
mod ease;
mod tweens;
mod id;

pub mod prelude {
	pub use crate::property::*;
	pub(crate) use crate::sequence::*;
	pub use crate::tweens::*;
	pub use crate::singleton::*;
	pub use crate::ease::*;
	pub use crate::lerping::*;
}

#[allow(unused_imports)]
pub(crate) use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use crate::prelude::*;
	pub use gdnative::prelude::*;
	pub use gdnative::api::*;
	pub use gdnative::derive::*;
	pub use gdnative::api::tween::TweenProcessMode;
	pub use gdnative::api::scene_tree_tween::TweenPauseMode;
	pub use util::prelude::*;
	pub use util_gdnative::prelude::*;
	pub use anyhow::{Result, anyhow, bail};
	pub use gdnative_export_node_as_path::extends;
	pub use inline_format::format as iformat;
	pub use inline_format::write as iwrite;
	pub use inline_format::format_args as iformat_args;
	pub use std::rc::{Rc, Weak};
	pub use std::marker::PhantomData;
	pub use std::mem;
	pub use std::collections::{VecDeque, HashMap, HashSet};
	pub use std::hash::Hash;
	pub use std::any::{type_name, type_name_of_val};
	pub use gdnative::object::memory::Memory;
	
	pub trait Inherits<T: GodotObject> {
		unsafe fn base<Base: GodotObject>(&self) -> Ref<Base> where T: SubClass<Base>;
	}

	impl<TSelf, Origin, Inherited> Inherits<Inherited> for TSelf
		where TSelf: ChangeRef<Inner = Origin>,
		      Origin: GodotObject + SubClass<Inherited>,
		      Inherited: GodotObject {
		unsafe fn base<Base: GodotObject>(&self) -> Ref<Base> where Inherited: SubClass<Base> {
			self.change_ref::<Inherited>().change_ref::<Base>()
		}
	}

	pub(crate) trait ChangeRef {
		type Inner: GodotObject;
		
		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where Self::Inner: SubClass<Base>;
	}
	
	#[allow(unused_qualifications)]
	impl<T, Own> ChangeRef for Ref<T, Own> 
		where T: GodotObject,
		      Own: gdnative::object::ownership::Ownership {
		type Inner = T;
		
		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where T: SubClass<Base> {
			Ref::<Base>::from_sys(std::ptr::NonNull::new_unchecked(self.as_ptr()))
		}
	}

	#[allow(unused_qualifications)]
	impl<T, Own> ChangeRef for TRef<'_, T, Own>
		where T: GodotObject,
		      Own: gdnative::object::ownership::Ownership {
		type Inner = T;
		
		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where Self::Inner: SubClass<Base> {
			self.upcast().assume_shared()
		}
	}

	impl<T: GodotObject> ChangeRef for &T {
		type Inner = T;
		
		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where Self::Inner: SubClass<Base> {
			self.upcast().assume_shared()
		}
	}
	
	#[allow(unused)]
	unsafe fn test(obj: &impl Inherits<Node>) {
		let obj_ref: Ref<Object> = obj.base();
	}

	#[allow(unused)]
	unsafe fn test_1(obj: &PathFollow2D, obj_ref: Ref<Node2D>, obj_tref: TRef<'_, Sprite>) {
		test(&obj);
		test(&obj_ref);
		test(&obj_tref);
	}

	#[allow(unused)]
	unsafe fn test_2(obj: &impl Inherits<Resource>) {
		let obj_ref: Ref<Object> = obj.base();
	}

	#[allow(unused)]
	unsafe fn test_3(obj: &PackedScene, obj_ref: Ref<Texture>, obj_tref: TRef<'_, DynamicFontData>) {
		test_2(&obj);
		test_2(&obj_ref);
		test_2(&obj_tref);
	}

}
