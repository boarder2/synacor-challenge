extern crate time;
extern crate byteorder;
mod loader;
mod vm;
mod ops;
use time::PreciseTime;

fn main() {
	let start = PreciseTime::now();
	match loader::load_bin("challenge\\challenge.bin".to_string()) {
		Ok(memory) => vm::run(memory),
		Err(ex) => println!("{:?}", ex),
	}
	let end = PreciseTime::now();
	println!("Took {}", start.to(end));
}