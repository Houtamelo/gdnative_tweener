#![allow(unused)]

#[allow(unused_imports)]
use crate::*;

macro_rules! do_trait_def {
    (pub trait $trait_ty: ident {
	    fn_name: $fn_name: ident,
	    val: $val_ty: ty,
    }) => {
	    pub trait $trait_ty {
			fn $fn_name(self, val: $val_ty, duration: f64) -> Result<NodeBoundTween>;
		}
    };
}

macro_rules! do_trait_impl {
    (impl $trait_ty: ident for $sub_ty: ty {
	    fn_name: $fn_name: ident,
	    val: $val_ty: ty,
	    property: $property: literal,
    }) => {
	    impl<T, TSelf> $trait_ty for TSelf
			where TSelf: DoPropertyDeprecated<Inner = T>,
			      T: SubClass<$sub_ty> {
			fn $fn_name(self, val: $val_ty, duration: f64) -> Result<NodeBoundTween> {
				self.do_property_deprecated($property, val, duration)
			}
		}
    };
}

macro_rules! do_trait_impl_into_val {
    (impl $trait_ty: ident for $sub_ty: ty {
	    fn_name: $fn_name: ident,
	    val: $val_ty: ty,
	    property: $property: literal,
    }) => {
	    impl<T, TSelf> $trait_ty for TSelf
			where TSelf: DoPropertyDeprecated<Inner = T>,
			      T: SubClass<$sub_ty> {
			fn $fn_name(self, val: $val_ty, duration: f64) -> Result<NodeBoundTween> {
				self.do_property_deprecated($property, val.into(), duration)
			}
		}
    };
}

macro_rules! do_full_trait {
    (pub trait $trait_ty: ident : $sub_ty: ty {
	    fn_name: $fn_name: ident,
	    val: $val: ty,
	    property: $property: literal,
	    tween: $tween: ty  $(,)?
    }) => {
	    pub trait $trait_ty {
			fn $fn_name(self, start_val: $val, end_val: $val, duration: f64) -> Result<$tween>;
		}
		
		impl<TSelf> $trait_ty for TSelf where TSelf: Inherits<$sub_ty> + Inherits<Object> {
			fn $fn_name(self, start_val: $val, end_val: $val, duration: f64) -> Result<$tween> {
				self.do_property($property, start_val, end_val, duration)
			}
		}
    };
	(pub trait $trait_ty: ident : $sub_ty: ty {
	    fn_name: $fn_name: ident,
	    impl_val: $val: ty,
	    property: $property: literal,
		tween: $tween: ty  $(,)?
    }) => {
	    pub trait $trait_ty {
			fn $fn_name(self, start_val: impl Into<$val>, end_val: impl Into<$val>, duration: f64) -> Result<$tween>;
		}
		
		impl<TSelf> $trait_ty for TSelf where TSelf: Inherits<$sub_ty> + Inherits<Object> {
			fn $fn_name(self, start_val: impl Into<$val>, end_val: impl Into<$val>, duration: f64) -> Result<$tween> {
				self.do_property($property, start_val.into(), end_val.into(), duration)
			}
		}
    };
}

#[allow(unused)]
pub(crate) use {
	do_trait_def,
	do_trait_impl,
	do_trait_impl_into_val,
	do_full_trait,
};