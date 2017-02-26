pub trait Operation {
	fn len(&self) -> usize;
	fn is_jump(&self) -> bool;
	fn run(&self, u16, &Vec<u16>, &mut Vec<u16>) -> usize;
}