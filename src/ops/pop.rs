
use ops::operation::Operation;
use vm::state;
pub struct Pop;

impl Operation for Pop {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let a1 = vm_state.get_mem_raw(ci + 1);
		let stack_val = vm_state.pop_stack().unwrap();
		vm_state.set_register(a1, stack_val);
		vm_state.set_current_instruction(ci + 2);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn pop() {
		let op = Pop;
		let mem = vec![2, 32768];
		let expected = vec![1234, 0, 0, 0, 0, 0, 0, 0];
		let registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let stack = vec![1234];
		let mut state = state_helper::generate_vm_state_mem_reg_stack(mem, registers, stack);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
		assert_eq!(None, state.pop_stack());
	}
}