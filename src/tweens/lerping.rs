#[allow(unused_imports)]
use crate::*;

pub trait _Lerp {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self;
	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self;
}

impl _Lerp for i64 {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self {
		(*from as f64 + (to - from) as f64 * t).round() as i64
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self { 
		present_at_obj + previous_calc - new_calc
	}
}

impl _Lerp for f64 {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self {
		from + (to - from) * t
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		present_at_obj + previous_calc - new_calc
	}
}

impl _Lerp for GodotString {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = from.to_string();
		let to = to.to_string();
		
		let t = t.clamp01();
		
		let from_len = from.chars().count() as i64;
		let to_len = to.chars().count() as i64;
		let new_len_raw= from_len + ((to_len - from_len) as f64 * t).round() as i64;
		let new_len = i64::abs(new_len_raw) as usize;
		
		let mut result = from.chars().collect::<Vec<_>>();
		let chars_to_take = usize::min((to_len as f64 * t).round() as usize, to_len as usize);
		let taken_chars = to.chars().take(chars_to_take).enumerate();
		for (index, char) in taken_chars {
			if result.len() > index {
				result[index] = char;
			} else {
				result.push(char);
			}
		}

		result.into_iter().take(new_len).collect::<String>().into()
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, next_calc: &Self) -> Self {
		let previous_calc = previous_calc.to_string();
		let next_calc = next_calc.to_string();
		
		let delta = {
			let new_count = next_calc.chars().count();
			let old_count = previous_calc.chars().count();

			if old_count >= new_count {
				&previous_calc
			} else {
				let new_index =
					previous_calc
						.char_indices()
						.nth(old_count)
						.map(|(index, _)| index)
						.unwrap_or(0);

				&next_calc[new_index..]
			}
		};
		
		(present_at_obj.to_string() + delta).into()
	}
}

impl _Lerp for Color {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = from;
		let to = to;
		
		let t = t as f32;
		Color::from_rgba(
			from.r + (to.r - from.r) * t,
			from.g + (to.g - from.g) * t,
			from.b + (to.b - from.b) * t,
			from.a + (to.a - from.a) * t)
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		let present_at_obj = present_at_obj;
		
		Color::from_rgba(
			present_at_obj.r + (new_calc.r - previous_calc.r),
			present_at_obj.g + (new_calc.g - previous_calc.g),
			present_at_obj.b + (new_calc.b - previous_calc.b),
			present_at_obj.a + (new_calc.a - previous_calc.a))
	}
}

impl _Lerp for Vector2 {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = from;
		let to = to;
		
		Vector2::new(
			from.x + (to.x - from.x) * t as f32,
			from.y + (to.y - from.y) * t as f32)
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		let present_at_obj = present_at_obj;
		
		Vector2::new(
			present_at_obj.x + (new_calc.x - previous_calc.x),
			present_at_obj.y + (new_calc.y - previous_calc.y))
	}
}

impl _Lerp for Vector3 {
	fn _lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = from;
		let to = to;

		Vector3::new(
			from.x + (to.x - from.x) * t as f32,
			from.y + (to.y - from.y) * t as f32,
			from.z + (to.z - from.z) * t as f32)
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		let present_at_obj = present_at_obj;

		Vector3::new(
			present_at_obj.x + (new_calc.x - previous_calc.x),
			present_at_obj.y + (new_calc.y - previous_calc.y),
			present_at_obj.z + (new_calc.z - previous_calc.z))
	}
}
