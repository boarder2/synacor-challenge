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
	use test::state_helper;

	#[test]
	fn set_mem_value() {
		let op = Set;
		let expected = vec![0, 1, 0, 0, 0, 0, 0, 0];
		let mem = vec![1, 32769, 1];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn set_reg_value() {
		let op = Set;
		let expected = vec![1, 1, 0, 0, 0, 0, 0, 0];
		let mem = vec![1, 32769, 32768];
		let registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}
}