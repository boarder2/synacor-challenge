use ops::operation::Operation;
pub struct Ret;

impl Operation for Ret {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		false//true
	}
	fn run(&self, _: u16, _: &mut Vec<u16>, _: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		0//stack.pop().unwrap() as usize
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[ignore]
	#[test]
	fn ret() {
		let op = Ret;
		let expected = 1234;
		let mut stack = vec![3333,expected];
		let result = op.run(0, &mut Vec::new(), &mut Vec::new(), &mut stack);
		assert_eq!(expected as usize, result);
	}
}