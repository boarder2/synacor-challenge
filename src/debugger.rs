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
	print_summary(ci, mem, reg, stack, debug_state);
	loop {
		println!("debug ci:{} - stepping:{}>", ci, debug_state.is_stepping());
		let mut buf = String::new();
		match io::stdin().read_line(&mut buf) {
			Ok(_) => {
				if buf.trim().is_empty() {
					break;
				}
				let args: Vec<&str> = buf.trim().split(' ').collect::<Vec<&str>>();
				match args[0] {
					// TODO: Add break on instruction type - always break on a particular instruction type.
					"aib" => add_instruction_breakpoint(&args, debug_state),
					"rib" => remove_instruction_breakpoint(&args, debug_state),
					"pib" => print_instruction_breakpoints(debug_state),
					"amw" => add_memory_watch(&args, debug_state),
					"rmw" => remove_memory_watch(&args, debug_state),
					"pmw" => print_memory_watches(debug_state, mem),
					"summary" => print_summary(ci, mem, reg, stack, debug_state),
					"step" => debug_state.set_stepping(true),
					"cont" => debug_state.set_stepping(false),
					"dumpmem" => println!("Memory {:?}", mem),
					"mem" => set_mem(ci, mem, &args),
					_ => print_help(),
				}
			}
			Err(_) => break,
		}
	}
}

fn add_instruction_breakpoint(args: &Vec<&str>, debug_state: &mut debug_state::DebugState) {
	if args.len() == 2 {
		match args[1].parse::<u16>() {
			Ok(inst) => debug_state.add_instruciton_break(inst),
			Err(_) => {}
		}
	}
}

fn remove_instruction_breakpoint(args: &Vec<&str>, debug_state: &mut debug_state::DebugState) {
	if args.len() == 2 {
		match args[1].parse::<u16>() {
			Ok(inst) => debug_state.remove_instruction_break(inst),
			Err(_) => {}
		}
	}
}

fn print_instruction_breakpoints(debug_state: &debug_state::DebugState) {
	println!("Instruction Breakpoints {:?}",
	         debug_state.get_instruction_breaks());
}

fn add_memory_watch(args: &Vec<&str>, debug_state: &mut debug_state::DebugState) {
	if args.len() == 2 {
		match args[1].parse::<u16>() {
			Ok(inst) => debug_state.add_memory_watch(inst),
			Err(_) => {}
		}
	}
}

fn remove_memory_watch(args: &Vec<&str>, debug_state: &mut debug_state::DebugState) {
	if args.len() == 2 {
		match args[1].parse::<u16>() {
			Ok(inst) => debug_state.remove_memory_watch(inst),
			Err(_) => {}
		}
	}
}

fn print_memory_watches(debug_state: &debug_state::DebugState, mem: &Vec<u16>) {
	let watches = debug_state.get_memory_watches();
	println!("Memory Watches");
	for watch in watches {
		println!("\t{} - {}", watch, mem[watch as usize]);
	}
}

fn clear_term() {
	print!("{}[2J", 27 as char);
}

fn print_summary(ci: u16,
                 mem: &mut Vec<u16>,
                 reg: &Vec<u16>,
                 stack: &Vec<u16>,
                 ds: &debug_state::DebugState) {
	println!("Instr [{}] {:?}",
	         instr_name(mem[ci as usize]),
	         mem.into_iter().skip(ci as usize).take(4).collect::<Vec<&mut u16>>());
	println!("Registers {:?}", reg);
	println!("Stack {:?}", stack);
	print_memory_watches(ds, mem);
}

fn instr_name(instr: u16) -> String {
	match instr {
		 0 => "halt".to_string(),
		 1 => "set".to_string(),
		 2 => "push".to_string(),
		 3 => "pop".to_string(),
		 4 => "eq".to_string(),
		 5 => "gt".to_string(),
		 6 => "jmp".to_string(),
		 7 => "jt".to_string(),
		 8 => "jf".to_string(),
		 9 => "add".to_string(),
		 10 => "mult".to_string(),
		 11 => "mod".to_string(),
		 12 => "and".to_string(),
		 13 => "or".to_string(),
		 14 => "not".to_string(),
		 15 => "rmem".to_string(),
		 16 => "wmem".to_string(),
		 17 => "call".to_string(),
		 18 => "ret".to_string(),
		 19 => "out".to_string(),
		 20 => "in".to_string(),
		 21 => "noop".to_string(),
		 _ => "???".to_string(),
	}
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
	println!("\tsummary\tPrint summary with stack and registers");
	println!("");
	println!("Breakpoints");
	println!("\taib <instr>\tAdd instruction breakpoint");
	println!("\trib <instr>\tRemove instruction breakpoint");
	println!("\tpib        \tPrint instruction breakpoints");
	println!("");
	println!("Execution");
	println!("\tstep\tSet execution mode to stepping (stop at each new instruction)");
	println!("\tcont\tSet execution mode to continuous (only breakpoints will stop)");
	println!("");
	println!("Memory");
	println!("\tdumpmem          \tDump entire contents of memory");
	println!("\tmem <begin> <end>\tPrint section of memory (relative to current instruction \
	          position)");
	println!("\tamw <offset>     \tAdd memory watch");
	println!("\trmw <offset>     \tRemove memory watch");
	println!("\tpmw <offset>     \tPrint memory watch locations");
}