use ops::operation::Operation;
use vm::state;
pub struct Wmem;

impl Operation for Wmem {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let addr = vm_state.get_mem_or_register_value(ci + 1);
		let val = vm_state.get_mem_or_register_value(ci + 2);
		vm_state.set_memory(addr, val);
		vm_state.set_current_instruction(ci + 3);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn wmem_mem_value_mem_loc() {
		let op = Wmem;
		let expected = 1234;
		let mem = vec![1, 3, expected, 0];
		let mut state = state_helper::generate_vm_state_mem(mem);
		op.run(&mut state);
		assert_eq!(expected, state.get_mem_raw(3));
	}

	#[test]
	fn wmem_reg_value_mem_loc() {
		let op = Wmem;
		let expected = 1234;
		let mem = vec![1, 3, 32768, 0];
		let reg = vec![expected];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_mem_raw(3));
	}

	#[test]
	fn wmem_mem_value_reg_loc() {
		let op = Wmem;
		let expected = 1234;
		let mem = vec![1, 32768, expected, 0];
		let reg = vec![3];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_mem_raw(3));
	}

	#[test]
	fn wmem_reg_value_reg_loc() {
		let op = Wmem;
		let expected = 1234;
		let mem = vec![1, 32769, 32768, 0];
		let reg = vec![expected, 3];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_mem_raw(3));
	}
}