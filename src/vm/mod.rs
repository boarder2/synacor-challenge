pub mod state;
use ops;
use debug_state;
use debugger;
use loader;

use std::collections::VecDeque;


pub fn run(bin_path: &str, debug: bool) {
	match loader::load_bin(bin_path) {
		Ok(memory) => {
			let mut vm_state = state::VMState::new(memory);

			let mut input_buffer = VecDeque::new();
			let mut debug_state = debug_state::DebugState::new();

			loop {
				if debug {
					debugger::step(&mut debug_state, &mut vm_state);
				}
				if !ops::run_op(&mut vm_state, &mut input_buffer) {
					break;
				}
			}
		}
		Err(ex) => println!("{:?}", ex),
	}
}