use ops::operation::Operation;
pub struct Wmem;

impl Operation for Wmem {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, _: u16, _: &Vec<u16>, _: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		0
	}
}