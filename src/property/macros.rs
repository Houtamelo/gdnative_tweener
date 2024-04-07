#![allow(unused)]

#[allow(unused_imports)]
use crate::*;

macro_rules! do_full_trait {
    (pub trait $trait_ty: ident : $sub_ty: ty {
	    fn_name: $fn_name: ident,
	    val: $val: ty,
	    property: $property: literal,
	    tween: $tween: ty  $(,)?
    }) => {
	    pub trait $trait_ty {
			fn $fn_name(&self, end_val: $val, duration: f64) -> $tween;
		}
		
		impl<TSelf> $trait_ty for TSelf where TSelf: Inherits<$sub_ty> + Inherits<Object> {
			fn $fn_name(&self, end_val: $val, duration: f64) -> $tween {
				self.do_property($property, end_val, duration)
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
			fn $fn_name(&self, end_val: impl Into<$val>, duration: f64) -> $tween;
		}
		
		impl<TSelf> $trait_ty for TSelf where TSelf: Inherits<$sub_ty> + Inherits<Object> {
			fn $fn_name(&self, end_val: impl Into<$val>, duration: f64) -> $tween {
				self.do_property($property, end_val.into(), duration)
			}
		}
    };
}

#[allow(unused)]
pub(crate) use do_full_trait;
