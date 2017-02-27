use ops::operation::Operation;
pub struct Noop;

impl Operation for Noop {
	fn len(&self) -> usize {
		1
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, _: u16, _: &mut Vec<u16>, _: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		0
	}
}