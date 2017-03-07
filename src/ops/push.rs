
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

	#[test]
	fn push_mem() {
		let op = Push;
		let mut mem = vec![2, 1234];
		let expected = vec![1234];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut stack = Vec::new();
		op.run(0, &mut mem, &mut registers, &mut stack);
		assert_eq!(expected, stack);
	}

	#[test]
	fn push_reg() {
		let op = Push;
		let mut mem = vec![2, 32768];
		let expected = vec![1234];
		let mut registers = vec![1234, 0, 0, 0, 0, 0, 0, 0];
		let mut stack = Vec::new();
		op.run(0, &mut mem, &mut registers, &mut stack);
		assert_eq!(expected, stack);
	}
}