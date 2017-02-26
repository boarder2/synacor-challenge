use ops::operation::Operation;
pub struct Push;

impl Operation for Push {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, _: u16, _: &Vec<u16>, _: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		0
	}
}