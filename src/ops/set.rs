
use ops::operation::Operation;
use vm::state;
pub struct Set;

impl Operation for Set {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val = vm_state.get_mem_or_register_value(ci + 2);
		let reg = vm_state.get_mem_raw(ci + 1);
		vm_state.set_register(reg, val);
		vm_state.set_current_instruction(ci + 3);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_mem_value() {
		let s = Set;
		let expected = vec![0, 1, 0, 0, 0, 0, 0, 0];
		let mut mem = vec![1, 32769, 1];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		s.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn set_reg_value() {
		let s = Set;
		let expected = vec![1, 1, 0, 0, 0, 0, 0, 0];
		let mut mem = vec![1, 32769, 32768];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		s.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}