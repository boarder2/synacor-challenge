
use ops::operation::Operation;
use vm::state;
pub struct ModOp;

impl Operation for ModOp {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val1 = vm_state.get_mem_or_register_value(ci + 2);
		let val2 = vm_state.get_mem_or_register_value(ci + 3);
		let reg = vm_state.get_mem_raw(ci + 1);
		let new_val = val1 % val2;
		vm_state.set_register(reg, new_val);
		vm_state.set_current_instruction(ci + 4);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn modop_mem() {
		let op = ModOp;
		let mut mem = vec![9, 32768, 3, 2];
		let expected = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn modop_reg() {
		let op = ModOp;
		let mut mem = vec![9, 32768, 32769, 32770];
		let expected = vec![1, 3, 2, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 3, 2, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}