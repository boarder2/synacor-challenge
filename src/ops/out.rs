use ops::operation::Operation;
pub struct Out;

impl Operation for Out {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, current_instruction: u16, memory: &Vec<u16>, _: &mut Vec<u16>) -> usize {
		print!("{}", memory[current_instruction as usize + 1] as u8 as char);
		0
	}
}
