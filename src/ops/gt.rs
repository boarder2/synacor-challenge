
use ops::operation::Operation;
use vm::state;
pub struct Gt;

impl Operation for Gt {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val1 = vm_state.get_mem_or_register_value(ci + 2);
		let val2 = vm_state.get_mem_or_register_value(ci + 3);
		let new_val = if val1 > val2 { 1 } else { 0 };
		let reg = vm_state.get_mem_raw(ci + 1);
		vm_state.set_register(reg, new_val);
		vm_state.set_current_instruction(ci + 4);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn gt_mem_gt() {
		let op = Gt;
		let mut mem = vec![4, 32768, 1112, 1111];
		let expected = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn gt_mem_equal() {
		let op = Gt;
		let mut mem = vec![4, 32768, 1111, 1111];
		let expected = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn gt_mem_lt() {
		let op = Gt;
		let mut mem = vec![4, 32768, 1111, 1111];
		let expected = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn gt_reg_gt() {
		let op = Gt;
		let mut mem = vec![4, 32768, 32769, 32770];
		let expected = vec![1, 123, 12, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 123, 12, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn gt_reg_equal() {
		let op = Gt;
		let mut mem = vec![4, 32768, 32769, 32770];
		let expected = vec![0, 12, 12, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 12, 12, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn gt_reg_lt() {
		let op = Gt;
		let mut mem = vec![4, 32768, 32769, 32770];
		let expected = vec![0, 11, 12, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 11, 12, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}