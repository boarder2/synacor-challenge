use ops;
use ops::operation::Operation;
pub struct Out;

impl Operation for Out {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &mut Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		print!("{}", ops::get_mem_or_register_value(mem[ci as usize + 1], reg) as u8 as char);
		0
	}
}
