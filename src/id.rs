use std::fmt::{Display, Formatter};
#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ID(pub Rc<()>);

impl PartialEq for ID {
	fn eq(&self, other: &Self) -> bool {
		Rc::ptr_eq(&self.0, &other.0)
	}
}

impl Eq for ID {}

impl Hash for ID {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		Rc::as_ptr(&self.0).hash(state)
	}
}

impl Display for ID {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "ID({:?})", self.0)
	}
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WeakID(pub Weak<()>);
