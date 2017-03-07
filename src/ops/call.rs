
use ops::operation::Operation;
use vm::state;
pub struct Call;

impl Operation for Call {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let new_instruction = vm_state.get_mem_or_register_value(ci + 1);
		vm_state.push_stack(ci + 2);
		vm_state.set_current_instruction(new_instruction);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn call_mem() {
		let op = Call;
		let expected = 1234;
		let mut mem = vec![0, expected];
		let mut stack = vec![];
		let result = op.run(0, &mut mem, &mut Vec::new(), &mut stack);
		assert_eq!(expected as usize, result);
		assert_eq!(stack[0], 2);
	}

	#[test]
	fn call_reg() {
		let op = Call;
		let expected = 1234;
		let mut mem = vec![0, 32768];
		let mut stack = vec![];
		let mut reg = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let result = op.run(0, &mut mem, &mut reg, &mut stack);
		assert_eq!(expected as usize, result);
		assert_eq!(stack[0], 2);
	}
}