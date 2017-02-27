use ops;
use ops::operation::Operation;
pub struct Push;

impl Operation for Push {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, stack: &mut Vec<u16>) -> usize {
		let val = ops::get_mem_or_register_value(mem[ci as usize + 1], reg);
		stack.push(val);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn push_mem() {
		let op = Push;
		let mem = vec![2, 1234];
		let expected = vec![1234];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut stack = Vec::new();
		op.run(0, &mem, &mut registers, &mut stack);
		assert_eq!(expected, stack);
	}

	#[test]
	fn push_reg() {
		let op = Push;
		let mem = vec![2, 32768];
		let expected = vec![1234];
		let mut registers = vec![1234, 0, 0, 0, 0, 0, 0, 0];
		let mut stack = Vec::new();
		op.run(0, &mem, &mut registers, &mut stack);
		assert_eq!(expected, stack);
	}
}