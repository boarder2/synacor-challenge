use ops;
use ops::operation::Operation;
pub struct Call;

impl Operation for Call {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		true
	}
	fn run(&self, ci: u16, mem: &mut Vec<u16>, reg: &mut Vec<u16>, stack: &mut Vec<u16>) -> usize {
		stack.push(ci + 2);
		ops::get_mem_or_register_value(mem[ci as usize + 1], reg) as usize
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn call_mem() {
		let op = Call;
		let expected = 1234;
		let mut mem = vec![0, expected];
		let mut stack = vec![];
		let result = op.run(0, &mut mem, &mut Vec::new(), &mut stack);
		assert_eq!(expected as usize, result);
		assert_eq!(stack[0], 2);
	}

	#[test]
	fn call_reg() {
		let op = Call;
		let expected = 1234;
		let mut mem = vec![0, 32768];
		let mut stack = vec![];
		let mut reg = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let result = op.run(0, &mut mem, &mut reg, &mut stack);
		assert_eq!(expected as usize, result);
		assert_eq!(stack[0], 2);
	}
}