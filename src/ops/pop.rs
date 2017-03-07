
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

	#[test]
	fn pop() {
		let op = Pop;
		let mut mem = vec![2, 32768];
		let expected = vec![1234, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut stack = vec![1234];
		op.run(0, &mut mem, &mut registers, &mut stack);
		assert_eq!(expected, registers);
		assert_eq!(stack, vec![]);
	}
}