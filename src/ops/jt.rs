
use ops::operation::Operation;
use vm::state;
pub struct Jt;

impl Operation for Jt {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val = vm_state.get_mem_or_register_value(ci + 1);
		let next_instr;
		if val != 0 {
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
	fn jt_mem_zero() {
		let j = Jt;
		let mem = vec![7, 0, 1234];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(3, state.get_current_instruction());
	}

	#[test]
	fn jt_mem_notzero() {
		let expected = 1234;
		let j = Jt;
		let mem = vec![7, 2, expected];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
	}

	#[test]
	fn jt_reg_zero() {
		let j = Jt;
		let mem = vec![7, 32768, 1234];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(3, state.get_current_instruction());
	}

	#[test]
	fn jt_reg_notzero() {
		let expected = 1234;
		let j = Jt;
		let mem = vec![7, 32768, expected];
		let registers = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		j.run(&mut state);
		assert_eq!(expected, state.get_current_instruction());
	}
}