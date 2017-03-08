use ops::operation::Operation;
use vm::state;
pub struct Add;

impl Operation for Add {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val1 = vm_state.get_mem_or_register_value(ci + 2);
		let val2 = vm_state.get_mem_or_register_value(ci + 3);
		let reg = vm_state.get_mem_raw(ci + 1);
		let new_val = ((val1 + val2) % 32768) as u16;
		vm_state.set_register(reg, new_val);
		vm_state.set_current_instruction(ci + 4);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn add_mem() {
		let a = Add;
		let expected = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem(vec![9, 32768, 1111, 1111]);
		a.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn add_mem_rollover() {
		let a = Add;
		let mut state = state_helper::generate_vm_state_mem(vec![9, 32768, 32767, 32767]);
		let expected = vec![32766, 0, 0, 0, 0, 0, 0, 0];
		a.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn add_reg() {
		let a = Add;
		let mut state = state_helper::generate_vm_state_mem_reg(vec![9, 32768, 32769, 32770],
		                                                        vec![0, 1111, 1111, 0, 0, 0, 0, 0]);
		let expected = vec![2222, 1111, 1111, 0, 0, 0, 0, 0];
		a.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}
}