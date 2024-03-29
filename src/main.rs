extern crate time;
extern crate byteorder;
extern crate clap;
extern crate serde;

mod loader;
mod vm;
mod ops;
mod debugger;
mod test;
mod debug_state;
use clap::{Arg, App};
use time::Instant;

fn main() {
	let start = Instant::now();
	let matches = App::new("Synacor Challenge VM")
		.version("1.0")
		.author("Willie Zutz")
		.about("Runs Synacor Challenge compatible binaries")
		.arg(Arg::with_name("debug")
			.short("d")
			.long("debug")
			.help("Runs application in interactive debugger"))
		.arg(Arg::with_name("binary")
			.short("b")
			.long("binary")
			.value_name("FILE")
			.help("Specify path to binary to run"))
		.get_matches();

	let bin_path;
	if let Some(path) = matches.value_of("binary") {
		bin_path = path;
	} else {
		bin_path = "challenge/challenge.bin";
	}

	let debug = matches.is_present("debug");

	vm::run(bin_path, debug);

	let end = Instant::now();
	let elapsed = start - end;
	println!("Took {}:{}:{}", elapsed.whole_minutes(), elapsed.whole_seconds(), elapsed.whole_milliseconds());
}