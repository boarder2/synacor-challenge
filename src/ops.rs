pub fn run_op(current_instruction: u16,
              memory: &Vec<u16>,
              registers: &mut Vec<u16>)
              -> Option<u16> {
	let mut new_instruction_index = current_instruction;
	new_instruction_index += 1;
	if current_instruction as usize > memory.len() {
		return None;
	}

	match memory[current_instruction as usize] {
		0 => return None,
		1 => {
			set(current_instruction, memory, registers);
			new_instruction_index += 2;
		}
		6 => return Some(jmp(current_instruction, memory, registers)),
		7 => return Some(jt(current_instruction, memory, registers)),
		8 => return Some(jf(current_instruction, memory, registers)),
		19 => {
			out(memory[current_instruction as usize + 1]);
			new_instruction_index += 1;
		}
		21 => {} // 21 is NOOP, so just move to the next instruction.
		i => {
			println!("Instruction not implemented {:?} at offset {:?}",
			         i,
			         current_instruction)
		}
		//_ => {}
	}

	// return new instruction index
	Some(new_instruction_index)
}

fn out(code: u16) {
	print!("{}", code as u8 as char);
}

fn set(current_instruction: u16, memory: &Vec<u16>, registers: &mut Vec<u16>) {
	let register = memory[current_instruction as usize + 1] - 32768;
	let value = get_mem_or_register_value(memory[current_instruction as usize + 2], registers);
	if let Some(r) = registers.get_mut(register as usize) {
		println!("Setting register {:?} to {} previous value {}",
		         register,
		         value,
		         r);
		*r = value;
	}
	println!("New registers {:?}", registers);
}

fn jmp(current_instruction: u16, memory: &Vec<u16>, registers: &Vec<u16>) -> u16 {
	get_mem_or_register_value(memory[current_instruction as usize + 1], registers)
}

fn jt(current_instruction: u16, memory: &Vec<u16>, registers: &Vec<u16>) -> u16 {
	let val = get_mem_or_register_value(memory[current_instruction as usize + 1], registers);
	if val != 0 {
		return get_mem_or_register_value(memory[current_instruction as usize + 2], registers);
	}
	current_instruction + 3
}

fn jf(current_instruction: u16, memory: &Vec<u16>, registers: &Vec<u16>) -> u16 {
	let val = get_mem_or_register_value(memory[current_instruction as usize + 1], registers);
	if val == 0 {
		return get_mem_or_register_value(memory[current_instruction as usize + 2], registers);
	}
	current_instruction + 3
}

fn get_mem_or_register_value(memory_value: u16, registers: &Vec<u16>) -> u16 {
	if memory_value >= 32768 {
		return registers[memory_value as usize - 32768];
	}
	memory_value
}