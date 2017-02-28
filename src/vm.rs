use ops;
use debugger;
use debug_state;

pub fn run(mut memory: &mut Vec<u16>) {
	let mut current_op = 0;
	let mut registers = Vec::with_capacity(8);
	let mut stack = Vec::new();
	let mut output = String::new();
	let mut debug_state = debug_state::DebugState::new();

	for _ in 0..8 {
		 registers.push(0);
	}
	let foo = memory.clone();
	println!("{:?}", foo);
	loop {
		debugger::step(&mut debug_state, &output, current_op, memory, &registers, &stack);

		if let Some(new_op) = ops::run_op(current_op, &mut memory, &mut registers, &mut stack, &mut output) {
			 current_op = new_op;
		} else {
			break;
		}
	}
}