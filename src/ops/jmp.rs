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
	#[test]
	fn jmp_mem() {
		let j = Jmp;
		let expected = 2;
		let mut mem = vec![6, expected];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected as usize, new_loc);
	}

	#[test]
	fn jmp_reg() {
		let j = Jmp;
		let expected = 19;
		let mut mem = vec![6, 32768];
		let mut registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected as usize, new_loc);
	}
}