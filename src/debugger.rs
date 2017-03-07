use debug_state;
use std::io;
use vm::state;

pub fn step(debug_state: &mut debug_state::DebugState, vm_state: &mut state::VMState) {

	let ci = vm_state.get_current_instruction();

	clear_term();
	println!("Output:\n{}", vm_state.get_console_output());
	print_summary(vm_state, debug_state);

	if !debug_state.is_stepping() && !debug_state.is_instruction_break(ci) &&
	   !debug_state.is_instruction_type_break(vm_state.get_mem_raw(ci)) {
		return;
	}
	
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
					"aitb" => add_instruction_type_breakpoint(&args, debug_state),
					"ritb" => remove_instruction_type_breakpoint(&args, debug_state),
					"pitb" => print_instruction_type_breakpoints(debug_state),
					"aib" => add_instruction_breakpoint(&args, debug_state),
					"rib" => remove_instruction_breakpoint(&args, debug_state),
					"pib" => print_instruction_breakpoints(debug_state),
					"amw" => add_memory_watch(&args, debug_state),
					"rmw" => remove_memory_watch(&args, debug_state),
					"pmw" => print_memory_watches(debug_state, vm_state),
					"summary" => print_summary(vm_state, debug_state),
					"step" => debug_state.set_stepping(true),
					"cont" => debug_state.set_stepping(false),
					//"dumpmem" => println!("Memory {:?}", mem),
					"mem" => show_mem(vm_state, &args),
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

fn add_instruction_type_breakpoint(args: &Vec<&str>, debug_state: &mut debug_state::DebugState) {
	if args.len() == 2 {
		match args[1].parse::<u16>() {
			Ok(inst) => debug_state.add_instruciton_type_break(inst),
			Err(_) => {}
		}
	}
}

fn remove_instruction_type_breakpoint(args: &Vec<&str>,
                                      debug_state: &mut debug_state::DebugState) {
	if args.len() == 2 {
		match args[1].parse::<u16>() {
			Ok(inst) => debug_state.remove_instruction_type_break(inst),
			Err(_) => {}
		}
	}
}

fn print_instruction_type_breakpoints(debug_state: &debug_state::DebugState) {
	println!("Instruction Type Breakpoints {:?}",
	         debug_state.get_instruction_type_breaks());
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

fn print_memory_watches(debug_state: &debug_state::DebugState, vms: &state::VMState) {
	let watches = debug_state.get_memory_watches();
	println!("Memory Watches");
	for watch in watches {
		println!("\t{} - {}", watch, vms.get_mem_raw(watch));
	}
}

fn clear_term() {
	print!("{}[2J", 27 as char);
}

fn print_summary(vms: &mut state::VMState, ds: &debug_state::DebugState) {
	let ci = vms.get_current_instruction();
	println!("Instr [{}] {:?}",
	         instr_name(vms.get_mem_raw(ci)),
	         vms.get_mem_segment(ci, 4));
	//TODO: Add register and stack retrieval
	// println!("Registers {:?}", reg);
	// println!("Stack {:?}", stack);
	print_memory_watches(ds, &vms);
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

fn show_mem(vms: &mut state::VMState, args: &Vec<&str>) {
	if args.len() == 3 {
		let ci = vms.get_current_instruction() as i32;
		let start = args[1].parse::<i32>().unwrap_or(0) + ci;
		let len = (ci + args[2].parse().unwrap_or(0)) - ci;
		println!("Memory start {:?} end {:?}", start, len + start);
		println!("{:?}", vms.get_mem_segment(start as u16, len as u16));
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
	println!("\taitb <instr>\tAdd instruction type breakpoint");
	println!("\tritb <instr>\tRemove instruction type breakpoint");
	println!("\tpitb        \tPrint instruction type breakpoints");
	println!("\taib <instr>\tAdd instruction breakpoint");
	println!("\trib <instr>\tRemove instruction breakpoint");
	println!("\tpib        \tPrint instruction breakpoints");
	println!("");
	println!("Execution");
	println!("\tstep\tSet execution mode to stepping (stop at each new instruction)");
	println!("\tcont\tSet execution mode to continuous (only breakpoints will stop)");
	println!("");
	println!("Memory");
	//println!("\tdumpmem          \tDump entire contents of memory");
	println!("\tmem <begin> <end>\tPrint section of memory (relative to current instruction \
	          position)");
	println!("\tamw <offset>     \tAdd memory watch");
	println!("\trmw <offset>     \tRemove memory watch");
	println!("\tpmw <offset>     \tPrint memory watch locations");
}