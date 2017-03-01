use debug_state;
use std::io;

pub fn step(debug_state: &mut debug_state::DebugState,
            output: &str,
            ci: u16,
            mem: &mut Vec<u16>,
            reg: &Vec<u16>,
            stack: &Vec<u16>) {
	if !debug_state.is_stepping() && !debug_state.is_instruction_break(ci) {
		return;
	}
	clear_term();
	println!("Output:\n{}", output);
	print_summary(ci, mem, reg, stack);
	loop {
		println!("debug ci:{} - stepping:{}>", ci, debug_state.is_stepping());
		let mut buf = String::new();
		match io::stdin().read_line(&mut buf) {
			Ok(_) => {
				if buf.trim().is_empty() {
					break;
				}
				let args: Vec<&str> = buf.trim().split(' ').collect::<Vec<&str>>();
				if args[0] == "aib" {
					if args.len() == 2 {
						match args[1].parse::<u16>() {
							Ok(inst) => debug_state.add_instruciton_break(inst),
							Err(_) => {}
						}
					}
				}
				if args[0] == "summary" {
					print_summary(ci, mem, reg, stack);
				}
				if args[0] == "step" {
					debug_state.set_stepping(true);
					break;
				}
				if args[0] == "cont" {
					debug_state.set_stepping(false);
					break;
				}
				if args[0] == "dumpmem" {
					println!("Memory {:?}", mem);
				}
				if args[0] == "mem" {
					set_mem(ci, mem, &args);
				}
				if args[0] == "help" {
					print_help();
				}
			}
			Err(_) => break,
		}
	}
}

fn clear_term() {
	print!("{}[2J", 27 as char);
}

fn print_summary(ci: u16, mem: &mut Vec<u16>, reg: &Vec<u16>, stack: &Vec<u16>) {
	println!("NextMem {:?}",
	         mem.into_iter().skip(ci as usize).take(4).collect::<Vec<&mut u16>>());
	println!("Registers {:?}", reg);
	println!("Stack {:?}", stack);
}

fn set_mem(ci: u16, mem: &mut Vec<u16>, args: &Vec<&str>) {
	if args.len() == 3 {
		let start = args[1].parse::<i32>().unwrap_or(0) + ci as i32;
		let len = (ci as i32 + args[2].parse().unwrap_or(0)) - ci as i32;
		println!("Memory start {:?} end {:?}", start, len + start);
		println!("{:?}",
		         mem.into_iter()
			         .skip(start as usize)
			         .take(len as usize)
			         .collect::<Vec<&mut u16>>());
	} else {
		println!("Invalid instruction");
	}
}

fn print_help() {
	// clear_term();
	println!("Available commands:");
	println!("\tsummary");
	println!("\taib");
	println!("\tstep");
	println!("\tcont");
	println!("\tdumpmem");
	println!("\tmem <begin> <end> (relative to current instruction position)");
}