extern crate time;
extern crate byteorder;
mod loader;
mod vm;
mod ops;
mod debugger;
mod debug_state;
use time::PreciseTime;

fn main() {
	let start = PreciseTime::now();
	match loader::load_bin("challenge\\challenge.bin".to_string()) {
		Ok(mut memory) => vm::run(&mut memory),
		Err(ex) => println!("{:?}", ex),
	}
	let end = PreciseTime::now();
	println!("Took {}", start.to(end));
}