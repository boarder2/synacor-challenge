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

#[cfg(test)]
mod tests {
	use std::fs;
	use std::fs::File;
	use std::io::Write;

	#[test]
	fn test_run() {
		let file_path = "./test_run.bin";
		let mut f = File::create(file_path).unwrap();
		let mut buf = [19, 65, 0, 0];
		f.write_all(&mut buf).unwrap();
		f.flush().unwrap();
		super::run(file_path, false);
		fs::remove_file(file_path).unwrap();
	}
}