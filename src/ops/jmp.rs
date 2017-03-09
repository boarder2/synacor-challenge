use ops::operation::Operation;

use vm::state;
pub struct Jmp;

impl Operation for Jmp {
	fn run(&self, vm_state: &mut state::VMState) {
		let next_instr = vm_state.get_mem_or_register_value(vm_state.get_current_instruction() + 1);
		vm_state.set_current_instruction(next_instr);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn jmp_mem() {
		let j = Jmp;
		let expected = 2;
		let mem = vec![6, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
	}

	#[test]
	fn jmp_reg() {
		let j = Jmp;
		let expected = 19;
		let mem = vec![6, 32768];
		let registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
	}
}