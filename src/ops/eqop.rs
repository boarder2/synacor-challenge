
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

	#[test]
	fn eq_mem_not_equal() {
		let op = EqOp;
		let mut mem = vec![4, 32768, 1111, 1112];
		let expected = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn eq_mem_equal() {
		let op = EqOp;
		let mut mem = vec![4, 32768, 1111, 1111];
		let expected = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn eq_reg_not_equal() {
		let op = EqOp;
		let mut mem = vec![4, 32768, 32769, 32770];
		let expected = vec![0, 12, 14, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 12, 14, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn eq_reg_equal() {
		let op = EqOp;
		let mut mem = vec![4, 32768, 32769, 32770];
		let expected = vec![1, 12, 12, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 12, 12, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}