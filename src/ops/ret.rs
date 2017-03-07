use ops::operation::Operation;
use vm::state;
pub struct Ret;

impl Operation for Ret {
	fn run(&self, vm_state: &mut state::VMState) {
		let next_instr = vm_state.pop_stack().unwrap();
		vm_state.set_current_instruction(next_instr);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ret() {
		let op = Ret;
		let expected = 1234;
		let mut stack = vec![3333, expected];
		let result = op.run(0, &mut Vec::new(), &mut Vec::new(), &mut stack);
		assert_eq!(expected as usize, result);
		assert_eq!(vec![3333], stack);
	}
}