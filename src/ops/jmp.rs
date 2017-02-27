use ops::operation::Operation;
use ops;
pub struct Jmp;

impl Operation for Jmp {
	fn len(&self) -> usize {
		2
	}
	fn is_jump(&self) -> bool {
		true
	}
	fn run(&self, ci: u16, mem: &mut Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		ops::get_mem_or_register_value(mem[ci as usize + 1], reg) as usize
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn jmp_mem() {
		let j = Jmp;
		let expected = 2;
		let mut mem = vec![6, expected];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected as usize, new_loc);
	}

	#[test]
	fn jmp_reg() {
		let j = Jmp;
		let expected = 19;
		let mut mem = vec![6, 32768];
		let mut registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected as usize, new_loc);
	}
}