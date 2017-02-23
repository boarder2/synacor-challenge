extern crate time;
extern crate byteorder;
mod loader;
use time::PreciseTime;

fn main() {
	let start = PreciseTime::now();
	let memory = loader::load_bin("challenge\\challenge.bin".to_string());
	println!("{:?}", memory);
	let end = PreciseTime::now();
	println!("Took {}", start.to(end));
}