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
		writeln!(f, "{:.2}:", self.time)?;

		if let Some(err) = &self.x {
			writeln!(f, "\tX: {}", err)?;
		}
		
		if let Some(err) = &self.y {
			writeln!(f, "\tY: {}", err)?;
		}
		
		if let Some(err) = &self.color {
			writeln!(f, "\tColor: {}", err)?;
		}
		
		Ok(())
	}
}

impl Report {
	fn generate(owner: &Node2D, time: f64, expected_x: f64, expected_y: f64, expected_r: f64) -> Option<Report> {
		let owner_x = owner.position().x as f64;
		let x =
			if f64::abs(owner_x - expected_x) > (owner_x.abs() + expected_x.abs()) / 1000. {
				Some(format!("Expected x: {}, actual x: {}", expected_x, owner_x))
			} else {
				None
			};
		
		let owner_y = owner.position().y as f64;
		let y =
			if f64::abs(owner_y - expected_y) > (owner_y.abs() + expected_y.abs()) / 1000. {
				Some(format!("Expected y: {}, actual y: {}", expected_y, owner_y))
			} else {
				None
			};
		
		let owner_r = owner.modulate().r as f64;
		let color =
			if f64::abs(owner_r - expected_r) > (owner_r.abs() + expected_r.abs()) / 1000. {
				Some(format!("Expected color r: {}, actual color r: {}", expected_r, owner_r))
			} else {
				None
			};

		if x.is_some() || y.is_some() || color.is_some() {
			Some(Report { x, y, color, time })
		} else {
			None
		}
	}
}

enum State {
	Running { test: Test, last_time: Option<f64> },
	PrintingReports,
	Finished,
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Tester {
	sequence: Option<SequenceID>,
	reports: Vec<Report>,
	frame_count: i64,
	state: State,
}

const D_1: f64 = 4.;
const D_15: f64 = D_1 * 1.5;
const D_2: f64 = D_1 * 2.0;

#[derive(Copy, Clone, FromVariant, ToVariant)]
enum Test {
	Absolute,
	SpeedBased,
	Relative,
}

impl Test {
	fn report(&self, tester: &mut Tester, owner: &Node2D, time: f64) {
		match self {
			Test::Absolute => {
				let expected_x = 
					match time {
						..=D_1 => {
							f64::_lerp(&0., &1000., time / D_1)
						}
						..=D_2 => {
							f64::_lerp(&1000., &-500., (time - D_1) / D_1)
						}
						_ => -500.
					};
				
				let expected_y =
					match time {
						..=D_1 => {
							f64::_lerp(&0., &-1000., time / D_1)
						}
						..=D_15 => {
							-1000.
						}
						..=D_2 => {
							f64::_lerp(&-1000., &0., (time - D_15) / (D_2 - D_15))
						}
						_ => 0.
					};
				
				let expected_color_r =
					match time {
						..=D_2 => {
							f64::_lerp(&0., &0.5, time / D_2)
						}
						_ => 0.5
					};
				
				Report::generate(owner, time, expected_x, expected_y, expected_color_r)
					.map(|report| tester.reports.push(report));
			}
			Test::SpeedBased => {
				let expected_x = 
					match time {
						..=10.0 => {
							f64::step(&0., &1000., 100., time).0
						}
						..=19.0 => {
							f64::step(&1000., &-800., 200., time - 10.).0
						}
						_ => -800.
					};
				
				let expected_y = 
					match time {
						..=10.0 => {
							f64::step(&0., &-1000., 100., time).0
						}
						..=17.0 => {
							f64::step(&-1000., &400., 200., time - 10.).0
						}
						_ => 400.
					};
				
				let expected_color_r =
					match time {
						..=10.0 => {
							f64::step(&0., &0.5, 0.05, time).0
						}
						..=12.0 => {
							0.5
						}
						..=12.5 => {
							f64::step(&0.5, &0., 1., time - 12.).0
						}
						_ => 0.
					};
				
				Report::generate(owner, time, expected_x, expected_y, expected_color_r)
					.map(|report| tester.reports.push(report));
			}
			Test::Relative => {
				let expected_x = 
					match time {
						..=D_1 => {
							f64::_lerp(&0., &1000., time / D_1)
						}
						..=D_2 => {
							1000. - 500. * (time - D_1) / D_1
						}
						_ => 500.
					};
				
				let expected_y = 
					match time {
						..=2.0 => {
							f64::_lerp(&0., &-1000., time / D_1)
						}
						..=D_1 => {
							f64::_lerp(&0., &-1000., time / D_1)
								+ f64::_lerp(&0., &1500., (time - 2.0) / D_1)
						}
						..=const { D_1 + 2.0 } => {
							-1000. + f64::_lerp(&0., &1500., (time - 2.0) / D_1)
						}
						_ => 500.
					};
				
				let expected_color_r =
					match time {
						..=D_2 => {
							f64::_lerp(&0., &1., time / D_2)
						}
						_ => 1.
					};
				
				Report::generate(owner, time, expected_x, expected_y, expected_color_r)
					.map(|report| tester.reports.push(report));
			}
		}
	}
	
	#[must_use]
	fn start(&self, owner: &Node2D) -> SequenceID {
		match self {
			Test::Absolute => {
				owner.do_color_r(0.5, D_2)
				     .register()
				     .unwrap();

				let mut seq = Sequence::new().bound_to(owner);
				seq.append(owner.do_move_x(1000.0, D_1));
				seq.join(owner.do_move_y(-1000.0, D_1));
				seq.append(owner.do_move_x(-500.0, D_1));
				seq.insert(D_15, owner.do_move_y(0., D_1).with_speed_scale(2.));
				seq.register()
				   .unwrap()
			}
			Test::SpeedBased => {
				owner.do_color_r(0.5, 0.)
				     .as_speed_based(0.05)
				     .register()
				     .unwrap();

				let mut seq = Sequence::new().bound_to(owner);
				seq.append(owner.do_move_x(1000.0, D_1).as_speed_based(100.));
				seq.join(owner.do_move_y(-1000.0, D_1).as_speed_based(100.));
				seq.append(owner.do_move(Vector2::new(-800., 400.), D_1).as_speed_based(200.));
				seq.insert(12.0, owner.do_color_r(0., 0.).as_speed_based(1.));
				seq.register()
				   .unwrap()
			}
			Test::Relative => {
				owner.do_color_r(1., D_2)
				     .register()
				     .unwrap();

				let mut seq = Sequence::new().bound_to(owner);
				seq.append(owner.do_move_x(1000.0, D_1));
				seq.join(owner.do_move_y(-1000.0, D_1));
				seq.append(owner.do_move_x(-500.0, D_1).as_relative(0.));
				seq.insert(2.0, owner.do_move_y(1500., D_1).as_relative(0.));
				seq.register()
				   .unwrap()
			}
		}
	}
}

#[methods]
impl Tester {
	fn new(_owner: &Node2D) -> Self {
		Self {
			sequence: None,
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
		       .connect("pressed", owner_ref, fn_name(&Self::_start_test), Test::Absolute.to_shared_array(), 0)
		       .log_if_err();

		buttons.get_node_as::<Button>("speed_based")
		       .unwrap()
		       .connect("pressed", owner_ref, fn_name(&Self::_start_test), Test::SpeedBased.to_shared_array(), 0)
		       .log_if_err();
		
		buttons.get_node_as::<Button>("relative")
		       .unwrap()
		       .connect("pressed", owner_ref, fn_name(&Self::_start_test), Test::Relative.to_shared_array(), 0)
		       .log_if_err();
	}
	
	fn reset(&mut self, owner: &Node2D) {
		self.reports.clear();
		owner.kill_bound_tweens()
		     .log_if_err();
		
		owner.set_position(Vector2::new(0., 0.));
		owner.set_modulate(Color::from_rgba(0., 1., 1., 1.));
	}

	#[method]
	fn _start_test(&mut self, #[base] owner: &Node2D, test: Test) {
		self.reset(owner);
		self.state = State::Running { test, last_time: None };
		self.sequence = Some(test.start(owner));
	}
	
	#[method]
	fn _process(&mut self, #[base] owner: &Node2D, _delta: f64) {
		match &mut self.state {
			State::Running { test, last_time } => {
				if let Some(time) = last_time.take() {
					let test = *test;
					test.report(self, owner, time);
				} else if let Some(time) =
					self.sequence.as_ref()
						.map(|id| { 
							id.map(|seq| seq.total_elapsed_time).ok() 
						}).flatten() {
					*last_time = Some(time);
				} else {
					godot_print!("Sequence ended, printing reports!");
					self.state = State::PrintingReports;
				}
			},
			State::PrintingReports => {
				self.frame_count -= 1;
				if self.frame_count > 0 {
					return;
				}
				
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