#[allow(unused_imports)]
use crate::*;

pub trait DoProperty<Val: _Lerp + FromVariant + ToVariant>: Sized {
	type Tween;

	fn do_property(&self,
	               property: impl Into<String>,
	               end_val: Val,
	               duration: f64)
	               -> Result<Self::Tween>;
}

macro_rules! do_property_impl {
    ($val: ty, $tween: ty) => {
	    impl<T: Inherits<Object>> DoProperty<$val> for T {
			type Tween = $tween;
		
			fn do_property(&self,
			               property: impl Into<String>,
			               end_val: $val,
			               duration: f64)
			               -> Result<Self::Tween> {
				let property = property.into();
				
				let obj_ref = unsafe { self.base() };
				let obj = unsafe { obj_ref.assume_safe() };
				let variant = obj.get_indexed(&property);
				let start_val = 
					variant.try_to::<$val>()
						   .map_err(|err| anyhow!(
								"Object `{obj:?}` returned invalid value for property `{property}` \n\
								 Value: `{variant:?}` \n\
								 Expected: `{}` \n\
								 Error: {err:?}", type_name::<$val>()))?;
				
				let mut tween = <$tween>::new(property, self, start_val, end_val, duration, AutoPlay(true));
				
				if let Some(node) = obj.cast::<Node>() {
					tween.bound_to(&node);
				} 
				
				Ok(tween)
			}
		}
	};
}

do_property_impl!(i64, TweenProperty_i64);
do_property_impl!(f64, TweenProperty_f64);
do_property_impl!(GodotString, TweenProperty_String);
do_property_impl!(Color, TweenProperty_Color);
do_property_impl!(Vector2, TweenProperty_Vector2);
do_property_impl!(Vector3, TweenProperty_Vector3);

pub trait DoPropertyVariant {
	fn do_property_var<Val: _Lerp + FromVariant + ToVariant + Clone + Copy>(
		&self,
		property: impl Into<String>,
		end_val: Val,
		duration: f64)
		-> Result<TweenProperty_Variant>;
}

impl<T: Inherits<Object>> DoPropertyVariant for T  {
	fn do_property_var<Val: _Lerp + FromVariant + ToVariant + Clone + Copy>(
		&self,
		property: impl Into<String>,
		end_val: Val, 
		duration: f64)
		-> Result<TweenProperty_Variant> {
		let lerp_fn = |from: &Variant, to: &Variant, t: f64| -> Variant {
			let from = from.to::<Val>().unwrap();
			let to = to.to::<Val>().unwrap();
			Val::_lerp(&from, &to, t).to_variant()
		};
		
		let relative_fn = |value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant| -> Variant {
			let value_at_obj = value_at_obj.to::<Val>().unwrap();
			let previous_calc = previous_calc.to::<Val>().unwrap();
			let next_calc = next_calc.to::<Val>().unwrap();
			Val::add_relative(&value_at_obj, &previous_calc, &next_calc).to_variant()
		};
		
		let property = property.into();

		let obj_ref = unsafe { self.base() };
		let obj = unsafe { obj_ref.assume_safe() };
		let variant = obj.get_indexed(&property);
		
		let start_val = 
			variant.try_to::<Val>()
				   .map_err(|err| anyhow!(
					   "Object `{obj:?}` returned invalid value for property `{property}` \n\
					    Value: `{variant:?}` \n\
						Expected: `{}` \n\
					    Error: {err:?}", type_name::<Val>()))?;

		let mut tween = TweenProperty_Variant::new(
			property, self, start_val, end_val, duration, AutoPlay(true), lerp_fn, relative_fn);

		if let Some(node) = obj.cast::<Node>() {
			tween.bound_to(&node);
		}

		Ok(tween)
	}
}
