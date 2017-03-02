use debug_state;
use debugger;
use ops;
use std::collections::VecDeque;

pub fn run(mut memory: &mut Vec<u16>, debug: bool) {
	let mut current_op = 0;
	let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
	let mut stack = Vec::new();
	let mut output = String::new();
	let mut input_buffer = VecDeque::new();
	let mut debug_state = debug_state::DebugState::new();

	loop {
		if debug {
			debugger::step(&mut debug_state,
			               &output,
			               current_op,
			               memory,
			               &registers,
			               &stack);
		}
		if let Some(new_op) = ops::run_op(current_op,
		                                  &mut memory,
		                                  &mut registers,
		                                  &mut stack,
		                                  &mut output,
		                                  &mut input_buffer) {
			current_op = new_op;
		} else {
			break;
		}
	}
}