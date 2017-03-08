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
	use test::state_helper;

	#[test]
	fn call_mem() {
		let op = Call;
		let expected = 1234;
		let mut state = state_helper::generate_vm_state_mem(vec![0, expected]);
		op.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
		assert_eq!(2, state.pop_stack().unwrap());
	}

	#[test]
	fn call_reg() {
		let op = Call;
		let expected = 1234;
		let mut state = state_helper::generate_vm_state_mem_reg(vec![0, 32768],
		                                                        vec![expected, 0, 0, 0, 0, 0, 0, 0]);
		op.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
		assert_eq!(2, state.pop_stack().unwrap());
	}
}