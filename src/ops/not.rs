use ops::operation::Operation;
use vm::state;
pub struct Not;

impl Operation for Not {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val = vm_state.get_mem_or_register_value(ci + 2);
		let reg = vm_state.get_mem_raw(ci + 1);
		let new_val = (!val) % 32768;
		vm_state.set_register(reg, new_val);
		vm_state.set_current_instruction(ci + 3);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn not_mem() {
		let op = Not;
		let mem = vec![0, 32768, 0];
		let expected = vec![32767, 0, 0, 0, 0, 0, 0, 0];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn not_reg() {
		let op = Not;
		let mem = vec![0, 32768, 32769];
		let expected = vec![32767, 0, 0, 0, 0, 0, 0, 0];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}
}