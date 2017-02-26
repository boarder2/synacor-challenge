use ops::operation::Operation;
use ops;
pub struct Set;

impl Operation for Set {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, current_instruction: u16, memory: &Vec<u16>, registers: &mut Vec<u16>) -> usize {
		let register = memory[current_instruction as usize + 1] - 32768;
		let value = ops::get_mem_or_register_value(memory[current_instruction as usize + 2], registers);
		if let Some(r) = registers.get_mut(register as usize) {
			println!("Setting register {:?} to {} previous value {}",
			         register,
			         value,
			         r);
			*r = value;
		}
		println!("New registers {:?}", registers);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_mem_value() {
		let s = Set;
		let expected = vec![0, 1, 0, 0, 0, 0, 0, 0];
		let mem = vec![1, 32769, 1];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		s.run(0, &mem, &mut registers);
		assert_eq!(expected, registers);
	}

	#[test]
	fn set_reg_value() {
		let s = Set;
		let expected = vec![1, 1, 0, 0, 0, 0, 0, 0];
		let mem = vec![1, 32769, 32768];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		s.run(0, &mem, &mut registers);
		assert_eq!(expected, registers);
	}
}