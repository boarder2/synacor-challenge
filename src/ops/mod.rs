mod operation;
mod out;
mod set;
mod noop;
mod jmp;
mod jt;
mod jf;

pub fn run_op(current_instruction: u16,
              memory: &Vec<u16>,
              registers: &mut Vec<u16>)
              -> Option<u16> {
	if current_instruction as usize > memory.len() {
		return None;
	}
	
	match memory[current_instruction as usize] {
		0 => return None,
		1 => return run_op_local(set::Set, current_instruction, memory, registers),
		6 => return run_op_local(jmp::Jmp, current_instruction, memory, registers),
		7 => return run_op_local(jt::Jt, current_instruction, memory, registers),
		8 => return run_op_local(jf::Jf, current_instruction, memory, registers),
		19 => return run_op_local(out::Out, current_instruction, memory, registers),
		21 => return run_op_local(noop::Noop, current_instruction, memory, registers),
		i => {
			println!("Instruction not implemented {:?} at offset {:?}",
			         i,
			         current_instruction);
			return Some(current_instruction + 1);
		}
		//_ => {}
	}
}

pub fn get_mem_or_register_value(memory_value: u16, registers: &Vec<u16>) -> u16 {
	if memory_value >= 32768 {
		return registers[memory_value as usize - 32768];
	}
	memory_value
}

fn run_op_local<T: operation::Operation>(op: T,
                                         current_instruction: u16,
                                         memory: &Vec<u16>,
                                         registers: &mut Vec<u16>)
                                         -> Option<u16> {
	let new_loc = op.run(current_instruction, memory, registers);
	if op.is_jump() {
		return Some(new_loc as u16);
	}
	Some(current_instruction + op.len() as u16)
}

#[cfg(test)]
mod tests {
	use super::*;

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
}