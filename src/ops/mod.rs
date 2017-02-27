mod operation;
mod set;
mod push;
mod pop;
mod eqop;
mod gt;
mod jmp;
mod jt;
mod jf;
mod add;
mod mult;
mod modop;
mod and;
mod or;
mod not;
mod rmem;
mod wmem;
mod call;
mod ret;
mod out;
mod inop;
mod noop;

pub fn run_op(current_instruction: u16,
              memory: &mut Vec<u16>,
              registers: &mut Vec<u16>,
              stack: &mut Vec<u16>)
              -> Option<u16> {
	if current_instruction as usize > memory.len() {
		return None;
	}

	//println!("Running op {:?}", memory[current_instruction as usize]);
	match memory[current_instruction as usize] {
		0 => return None,
		1 => return run_op_local(set::Set, current_instruction, memory, registers, stack),
		2 => return run_op_local(push::Push, current_instruction, memory, registers, stack),
		3 => return run_op_local(pop::Pop, current_instruction, memory, registers, stack),
		4 => return run_op_local(eqop::EqOp, current_instruction, memory, registers, stack),
		5 => return run_op_local(gt::Gt, current_instruction, memory, registers, stack),
		6 => return run_op_local(jmp::Jmp, current_instruction, memory, registers, stack),
		7 => return run_op_local(jt::Jt, current_instruction, memory, registers, stack),
		8 => return run_op_local(jf::Jf, current_instruction, memory, registers, stack),
		9 => return run_op_local(add::Add, current_instruction, memory, registers, stack),
		10 => return run_op_local(mult::Mult, current_instruction, memory, registers, stack),
		11 => return run_op_local(modop::ModOp, current_instruction, memory, registers, stack),
		12 => return run_op_local(and::And, current_instruction, memory, registers, stack),
		13 => return run_op_local(or::Or, current_instruction, memory, registers, stack),
		14 => return run_op_local(not::Not, current_instruction, memory, registers, stack),
		15 => return run_op_local(rmem::Rmem, current_instruction, memory, registers, stack),
		16 => return run_op_local(wmem::Wmem, current_instruction, memory, registers, stack),
		17 => return run_op_local(call::Call, current_instruction, memory, registers, stack),
		18 => return run_op_local(ret::Ret, current_instruction, memory, registers, stack),
		19 => return run_op_local(out::Out, current_instruction, memory, registers, stack),
		20 => return run_op_local(inop::InOp, current_instruction, memory, registers, stack),
		21 => return run_op_local(noop::Noop, current_instruction, memory, registers, stack),
		x => {
			println!("{:?} not implemented\nRegisters {:?}\nStack {:?}", x, registers, stack);
			//println!("Memory {:?}", memory);
			None
		},
		//_ => unimplemented!(),
	}
}

pub fn get_mem_or_register_value(memory_value: u16, registers: &Vec<u16>) -> u16 {
	if memory_value >= 32768 {
		return registers[memory_value as usize - 32768];
	}
	memory_value
}

pub fn set_register(register_raw: u16, registers: &mut Vec<u16>, value: u16) {
	let register = register_raw - 32768;
	if let Some(r) = registers.get_mut(register as usize) {
		println!("Setting register {:?} to {:?}", register, value);
		*r = value;
	}
}

fn run_op_local<T: operation::Operation>(op: T,
                                         current_instruction: u16,
                                         memory: &mut Vec<u16>,
                                         registers: &mut Vec<u16>,
                                         stack: &mut Vec<u16>)
                                         -> Option<u16> {
	let new_loc = op.run(current_instruction, memory, registers, stack);
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