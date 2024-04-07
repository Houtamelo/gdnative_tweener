use std::fmt::{Display, Formatter};
use crate::*;

struct Report {
	time: f64,
	x: Option<String>,
	y: Option<String>,
	color: Option<String>,
}

impl Display for Report {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let x = match &self.x {
			None => "Ok",
			Some(err) => &err,
		};
		
		let y = match &self.y {
			None => "Ok",
			Some(err) => &err,
		};
		
		let color = match &self.color {
			None => "Ok",
			Some(err) => &err,
		};
		
		write!(f, "{:.2}: \n\tX: {}\n\tY: {}\n\tColor: {}", self.time, x, y, color)
	}
}

impl Report {
	fn generate(owner: &Node2D, time: f64, expected_x: f64, expected_y: f64, expected_color_r: f64) -> Report {
		let owner_x = owner.position().x as f64;
		let x = if f64::abs(owner_x - expected_x) > 0.01 {
			Some(format!("Expected x: {}, actual x: {}", expected_x, owner_x))
		} else {
			None
		};
		
		let owner_y = owner.position().y as f64;
		let y = if f64::abs(owner_y - expected_y) > 0.01 {
			Some(format!("Expected y: {}, actual y: {}", expected_y, owner_y))
		} else {
			None
		};
		
		let owner_r = owner.modulate().r as f64;
		let color = if f64::abs(owner_r - expected_color_r) > 0.01 {
			Some(format!("Expected color r: {}, actual color r: {}", expected_color_r, owner_r))
		} else {
			None
		};

		Report { x, y, color, time }
	}
}

enum State {
	Running(Test),
	PrintingReports,
	Finished,
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Tester {
	time: f64,
	reports: Vec<Report>,
	frame_count: i64,
	state: State,
}

const D_1: f64 = 4.;
const D_2: f64 = 8.;

#[derive(Copy, Clone)]
enum Test {
	Std,
	Relative,
}

impl Test {
	fn report(&self, tester: &mut Tester, owner: &Node2D, time: f64) {
		match self {
			Test::Std => {
				match time {
					..=D_1 => {
						let expected_x = 1000. * time / D_1;
						let expected_y = -1000. * time / D_1;
						let expected_color_r = time * 0.5 / D_2;
						tester.reports.push(Report::generate(owner, time, expected_x, expected_y, expected_color_r));
					}
					..=D_2 => {
						let expected_x = f64::_lerp(&1000., &-500., (time - D_1) / D_1);
						let expected_y = -1000.;
						let expected_color_r = time * 0.5 / D_2;
						tester.reports.push(Report::generate(owner, time, expected_x, expected_y, expected_color_r));
					}
					_ => {
						let expected_x = -500.;
						let expected_y = -1000.;
						let expected_color_r = 0.5;
						tester.reports.push(Report::generate(owner, time, expected_x, expected_y, expected_color_r));
						tester.state = State::PrintingReports;
					}
				}
			}
			Test::Relative => {
				match time {
					..=D_1 => {
						let expected_x = 1000. * time / D_1;
						let expected_y = -1000. * time / D_1;
						let expected_color_r = time / D_2;
						tester.reports.push(Report::generate(owner, time, expected_x, expected_y, expected_color_r));
					}
					..=D_2 => {
						let expected_x = 1000. - 500. * (time - D_1) / D_1;
						let expected_y = -1000.;
						let expected_color_r = time / D_2;
						tester.reports.push(Report::generate(owner, time, expected_x, expected_y, expected_color_r));
					}
					_ => {
						let expected_x = 500.;
						let expected_y = -1000.;
						let expected_color_r = 1.;
						tester.reports.push(Report::generate(owner, time, expected_x, expected_y, expected_color_r));
						tester.state = State::PrintingReports;
					}
				}
			}
		}
	}
	
	fn start(&self, owner: &Node2D) {
		match self {
			Test::Std => {
				owner.do_color_r(0.5, D_2)
				     .register()
				     .unwrap();

				let mut seq = Sequence::new().bound_to(owner);
				seq.append(owner.do_move_x(1000.0, D_1));
				seq.join(owner.do_move_y(-1000.0, D_1));
				seq.append(owner.do_move_x(-500.0, D_1).lerp_flexible());
				seq.register()
				   .unwrap();
			}
			Test::Relative => {
				owner.do_color_r(1., D_2)
				     .register()
				     .unwrap();

				let mut seq = Sequence::new().bound_to(owner);
				seq.append(owner.do_move_x(1000.0, D_1));
				seq.join(owner.do_move_y(-1000.0, D_1));
				seq.append(owner.do_move_x(-500.0, D_1).lerp_relative());
				seq.register()
				   .unwrap();
			}
		}
	}
}

#[methods]
impl Tester {
	fn new(_owner: &Node2D) -> Self {
		Self {
			time: 0.,
			reports: Vec::new(),
			frame_count: 1,
			state: State::Finished,
		}
	}
	
	#[method]
	unsafe fn _ready(&self, #[base] owner: &Node2D) {
		let owner_ref = owner.assume_shared();
		
		let buttons = 
			owner.get_node_as::<Control>("../test_buttons")
				 .unwrap();
		
		buttons.get_node_as::<Button>("std")
		       .unwrap()
		       .connect("pressed", owner_ref, fn_name(&Self::_test_std), VariantArray::new_shared(), 0)
		       .log_if_err();
		
		buttons.get_node_as::<Button>("relative")
		       .unwrap()
		       .connect("pressed", owner_ref, fn_name(&Self::_test_relative), VariantArray::new_shared(), 0)
		       .log_if_err();
	}
	
	fn reset(&mut self, owner: &Node2D) {
		self.time = 0.;
		self.reports.clear();
		owner.kill_bound_tweens()
		     .log_if_err();
		
		owner.set_position(Vector2::new(0., 0.));
		owner.set_modulate(Color::from_rgba(0., 1., 1., 1.));
	}

	#[method]
	fn _test_std(&mut self, #[base] owner: &Node2D) {
		self.reset(owner);
		self.state = State::Running(Test::Std);
		Test::Std.start(owner);
	}

	#[method]
	fn _test_relative(&mut self, #[base] owner: &Node2D) {
		self.reset(owner);
		owner.kill_bound_tweens()
		     .log_if_err();
		
		self.state = State::Running(Test::Relative);
		Test::Relative.start(owner);
	}
	
	#[method]
	fn _process(&mut self, #[base] owner: &Node2D, delta: f64) {
		let time = self.time;
		self.time += delta;
		
		self.frame_count -= 1;
		if self.frame_count > 0 {
			return;
		}

		self.frame_count = 30;
		
		match self.state {
			State::Running(test) => {
				test.report(self, owner, time);
			},
			State::PrintingReports => {
				let count = usize::clamp(self.reports.len(), 0, 5);
				if count <= 0 {
					self.state = State::Finished;
					return;
				}
				
				self.frame_count = 120;

				let to_print =
					self.reports
					    .drain(..count)
					    .map(|r| r.to_string())
					    .collect::<Vec<_>>()
					    .join("\n");

				godot_print!("{to_print}");
			},
			State::Finished => {}
		}
	}
}