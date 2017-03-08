
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

	// #[test]
	// fn wmem_mem() {
	// 	let op = Wmem;
	// 	let mem = vec![0, 3, 3, 0];
	// 	let expected = vec![3, 0, 0, 0, 0, 0, 0, 0];
	// 	let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
	// 	op.run(0, &mut mem, &mut registers, &mut Vec::new());
	// 	assert_eq!(expected, registers);
	// }

	// #[test]
	// fn rmem_reg() {
	// 	let op = Wmem;
	// 	let mem = vec![0, 32768, 32769, 3];
	// 	let expected = vec![3, 3, 0, 0, 0, 0, 0, 0];
	// 	let registers = vec![0, 3, 0, 0, 0, 0, 0, 0];
	// 	op.run(0, &mut mem, &mut registers, &mut Vec::new());
	// 	assert_eq!(expected, registers);
	// }
}