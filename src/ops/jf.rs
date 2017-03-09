use ops::operation::Operation;
use vm::state;
pub struct Jf;

impl Operation for Jf {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val = vm_state.get_mem_or_register_value(ci + 1);
		let next_instr;
		if val == 0 {
			next_instr = vm_state.get_mem_or_register_value(ci + 2);
		} else {
			next_instr = ci + 3;
		}
		vm_state.set_current_instruction(next_instr);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn jf_mem_zero() {
		let j = Jf;
		let expected = 1234;
		let mem = vec![8, 0, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
	}

	#[test]
	fn jf_mem_notzero() {
		let j = Jf;
		let mem = vec![8, 2, 1234];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(3, state.get_current_instruction());
	}

	#[test]
	fn jf_reg_zero() {
		let j = Jf;
		let expected = 1234;
		let mem = vec![8, 32768, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
	}

	#[test]
	fn jf_reg_notzero() {
		let j = Jf;
		let mem = vec![8, 32768, 1234];
		let registers = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(3, state.get_current_instruction());
	}
}