#[allow(unused_imports)]
use crate::*;

#[allow(unused)]
macro_rules! value_impl {
    ($value_ty: ty, $struct_ty: ident) => {
	    impl $struct_ty {
		    pub fn with_ease(self, ease: Ease) -> Self { 
				Self { ease, ..self }
			}
		    
		    pub fn ending_at(self, value: $value_ty) -> Self {
				Self { end: value, ..self }
			}
	    }
    };
}

pub(crate) use value_impl;