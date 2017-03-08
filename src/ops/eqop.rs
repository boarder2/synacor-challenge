
use ops::operation::Operation;
use vm::state;
pub struct EqOp;

impl Operation for EqOp {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val1 = vm_state.get_mem_or_register_value(ci + 2);
		let val2 = vm_state.get_mem_or_register_value(ci + 3);
		let new_val = if val1 == val2 { 1 } else { 0 };
		let reg = vm_state.get_mem_raw(ci + 1);
		vm_state.set_register(reg, new_val);
		vm_state.set_current_instruction(ci + 4);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn eq_mem_not_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 1111, 1112];
		let expected = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let reg = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn eq_mem_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 1111, 1111];
		let expected = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let reg = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn eq_reg_not_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 32769, 32770];
		let expected = vec![0, 12, 14, 0, 0, 0, 0, 0];
		let reg = vec![1, 12, 14, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn eq_reg_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 32769, 32770];
		let expected = vec![1, 12, 12, 0, 0, 0, 0, 0];
		let reg = vec![0, 12, 12, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, reg);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}
}