use ops::operation::Operation;
use vm::state;
pub struct Ret;

impl Operation for Ret {
	fn run(&self, vm_state: &mut state::VMState) {
		let next_instr = vm_state.pop_stack().unwrap();
		vm_state.set_current_instruction(next_instr);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn ret() {
		let op = Ret;
		let expected = 1234;
		let stack = vec![3333, expected];
		let mut state = state_helper::generate_vm_state_mem_reg_stack(vec![], vec![], stack);
		op.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
		assert_eq!(Some(3333), state.pop_stack());
	}
}
