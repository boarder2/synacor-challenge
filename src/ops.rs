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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_mem_value() {
		let expected = vec![0, 1, 0, 0, 0, 0, 0, 0];
		let mem = vec![1, 32769, 1];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		set(0, &mem, &mut registers);
		assert_eq!(expected, registers);
	}

	#[test]
	fn set_reg_value() {
		let expected = vec![1, 1, 0, 0, 0, 0, 0, 0];
		let mem = vec![1, 32769, 32768];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		set(0, &mem, &mut registers);
		assert_eq!(expected, registers);
	}

	#[test]
	fn get_mem_or_register_value_mem() {
		let expected = 1234;
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let result = get_mem_or_register_value(expected, &registers);
		assert_eq!(expected, result);
	}

	#[test]
	fn get_mem_or_register_value_reg_0() {
		let expected = 1234;
		let registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let result = get_mem_or_register_value(32768, &registers);
		assert_eq!(expected, result);
	}
	
	#[test]
	fn get_mem_or_register_value_reg_8() {
		let expected = 1234;
		let registers = vec![0, 0, 0, 0, 0, 0, 0, expected];
		let result = get_mem_or_register_value(32775, &registers);
		assert_eq!(expected, result);
	}

	#[test]
	fn jmp_mem() {
		let expected = 2;
		let mem = vec![6, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jmp(0, &mem, &registers);
		assert_eq!(expected, new_loc);
	}

	#[test]
	fn jmp_reg() {
		let expected = 19;
		let mem = vec![6, 32768];
		let registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jmp(0, &mem, &registers);
		assert_eq!(expected, new_loc);
	}

	#[test]
	fn jt_mem_zero() {
		let mem = vec![7, 0, 1234];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jt(0, &mem, &registers);
		assert_eq!(3, new_loc);
	}

	#[test]
	fn jt_mem_notzero() {
		let expected = 1234;
		let mem = vec![7, 2, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jt(0, &mem, &registers);
		assert_eq!(expected, new_loc);
	}

	#[test]
	fn jt_reg_zero() {
		let mem = vec![7, 32768, 1234];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jt(0, &mem, &registers);
		assert_eq!(3, new_loc);
	}

	#[test]
	fn jt_reg_notzero() {
		let expected = 1234;
		let mem = vec![7, 32768, expected];
		let registers = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jt(0, &mem, &registers);
		assert_eq!(expected, new_loc);
	}

	#[test]
	fn jf_mem_zero() {
		let expected = 1234;
		let mem = vec![8, 0, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jf(0, &mem, &registers);
		assert_eq!(expected, new_loc);
	}

	#[test]
	fn jf_mem_notzero() {
		let mem = vec![8, 2, 1234];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jf(0, &mem, &registers);
		assert_eq!(3, new_loc);
	}

	#[test]
	fn jf_reg_zero() {
		let expected = 1234;
		let mem = vec![8, 32768, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jf(0, &mem, &registers);
		assert_eq!(expected, new_loc);
	}

	#[test]
	fn jf_reg_notzero() {
		let mem = vec![8, 32768, 1234];
		let registers = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = jf(0, &mem, &registers);
		assert_eq!(3, new_loc);
	}
}