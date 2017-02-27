use ops;
use ops::operation::Operation;
pub struct Pop;

impl Operation for Pop {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, stack: &mut Vec<u16>) -> usize {
		ops::set_register(mem[ci as usize + 1], reg, stack.pop().unwrap());
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pop() {
		let op = Pop;
		let mem = vec![2, 32768];
		let expected = vec![1234, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut stack = vec![1234];
		op.run(0, &mem, &mut registers, &mut stack);
		assert_eq!(expected, registers);
		assert_eq!(stack, vec![]);
	}
}