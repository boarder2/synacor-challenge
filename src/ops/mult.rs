use ops::operation::Operation;
use vm::state;
pub struct Mult;

impl Operation for Mult {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val1 = vm_state.get_mem_or_register_value(ci + 2);
		let val2 = vm_state.get_mem_or_register_value(ci + 3);
		let reg = vm_state.get_mem_raw(ci + 1);
		let new_val = ((val1 as u32 * val2 as u32) % 32768) as u16;
		vm_state.set_register(reg, new_val);
		vm_state.set_current_instruction(ci + 4);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::state_helper;

	#[test]
	fn mult_mem() {
		let op = Mult;
		let mem = vec![9, 32768, 2, 6];
		let expected = vec![12, 0, 0, 0, 0, 0, 0, 0];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn mult_mem_overflow() {
		let op = Mult;
		let mem = vec![9, 32768, 32767, 2];
		let expected = vec![32766, 0, 0, 0, 0, 0, 0, 0];
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}

	#[test]
	fn mult_reg() {
		let op = Mult;
		let mem = vec![9, 32768, 32769, 32770];
		let expected = vec![12, 2, 6, 0, 0, 0, 0, 0];
		let registers = vec![0, 2, 6, 0, 0, 0, 0, 0];
		let mut state = state_helper::generate_vm_state_mem_reg(mem, registers);
		op.run(&mut state);
		assert_eq!(expected, state.get_registers());
	}
}