use ops::operation::Operation;
use vm::state;
pub struct Push;

impl Operation for Push {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val = vm_state.get_mem_or_register_value(ci + 1);
		vm_state.push_stack(val);
		vm_state.set_current_instruction(ci + 2);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn push_mem() {
		let op = Push;
		let expected = 1234;
		let mem = vec![2, expected];
		let registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(Some(expected), state.pop_stack());
	}

	#[test]
	fn push_reg() {
		let op = Push;
		let expected = 1234;
		let mem = vec![2, 32768];
		let registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(Some(expected), state.pop_stack());
	}
}