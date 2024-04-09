#[allow(unused_imports)]
use crate::*;
use enum_dispatch::enum_dispatch;

mod tween_typed;
mod tween_variant;
mod tween_macros;

#[allow(unused_imports)] pub use tween_typed::*;
#[allow(unused_imports)] pub use tween_variant::*;

#[enum_dispatch(Tick)]
#[derive(Debug)]
pub enum TweenProperty {
	i64(TweenProperty_i64),
	f64(TweenProperty_f64),
	String(TweenProperty_String),
	Color(TweenProperty_Color),
	Vector2(TweenProperty_Vector2),
	Vector3(TweenProperty_Vector3),
	Variant(TweenProperty_Variant),
}

pub(crate) fn eval_property<T: FromVariant>(obj: &impl Inherits<Object>, property: &GodotString) -> Result<T> {
	let Some(target) = (unsafe { obj.base().assume_safe_if_sane() })
		else { bail!("Target is not sane, cannot evaluate property `{property}`'s value.") };

	let variant = target.get_indexed(property.new_ref());

	variant.try_to::<T>()
	       .map_err(|err| anyhow!(
			   "Target property `{property}` is not of type `{}`, got: `{variant:?}`. \n\
			    Error: {}", type_name::<T>(), err))
}
