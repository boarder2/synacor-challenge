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
			set(registers,
			    memory[current_instruction as usize + 1],
			    memory[current_instruction as usize + 2]);
			new_instruction_index += 2;
		}
		6 => return Some(jmp(current_instruction, memory, registers)),
		19 => {
			out(memory[current_instruction as usize + 1]);
			new_instruction_index += 1;
		}
		21 => {} // 21 is NOOP, so just move to the next instruction.
		i => println!("Instruction not implemented {:?} at offset {:?}", i, current_instruction),
		//_ => {}
	}

	// return new instruction index
	Some(new_instruction_index)
}

fn out(code: u16) {
	print!("{}", code as u8 as char);
}

fn set(registers: &mut Vec<u16>, register: u16, value: u16) {
	println!("Register set {:?} to {:?} - Registers {:?}",
	         register,
	         value,
	         registers);
	if let Some(r) = registers.get_mut(register as usize) {
		*r = value;
	}
}

fn jmp(current_instruction: u16, memory: &Vec<u16>, registers: &Vec<u16>) -> u16 {
	get_mem_or_register_value(memory[current_instruction as usize + 1], registers)
}

fn get_mem_or_register_value(memory_value: u16, registers: &Vec<u16>) -> u16 {
	if memory_value >= 32768 {
		return registers[memory_value as usize - 32768];
	}
	memory_value
}