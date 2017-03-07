
use ops::operation::Operation;
use vm::state;
pub struct Rmem;

impl Operation for Rmem {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let val = vm_state.get_mem_raw(vm_state.get_mem_or_register_value(ci + 2));
		let addr = vm_state.get_mem_raw(ci + 1);
		vm_state.set_register(addr, val);
		vm_state.set_current_instruction(ci + 3);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rmem_mem() {
		let op = Rmem;
		let mut mem = vec![0, 32768, 3, 3];
		let expected = vec![3, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn rmem_reg() {
		let op = Rmem;
		let mut mem = vec![0, 32768, 32769, 3];
		let expected = vec![3, 3, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 3, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}