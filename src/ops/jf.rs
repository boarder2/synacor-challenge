use ops;
use ops::operation::Operation;
pub struct Jf;

impl Operation for Jf {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		true
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>) -> usize {
		let val = ops::get_mem_or_register_value(mem[ci as usize + 1], reg);
		if val == 0 {
			return ops::get_mem_or_register_value(mem[ci as usize + 2], reg) as usize;
		}
		ci as usize + self.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn jf_mem_zero() {
		let j = Jf;
		let expected = 1234;
		let mem = vec![8, 0, expected];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers);
		assert_eq!(expected as usize, new_loc);
	}

	#[test]
	fn jf_mem_notzero() {
		let j = Jf;
		let mem = vec![8, 2, 1234];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers);
		assert_eq!(3, new_loc);
	}

	#[test]
	fn jf_reg_zero() {
		let j = Jf;
		let expected = 1234;
		let mem = vec![8, 32768, expected];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers);
		assert_eq!(expected as usize, new_loc);
	}

	#[test]
	fn jf_reg_notzero() {
		let j = Jf;
		let mem = vec![8, 32768, 1234];
		let mut registers = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers);
		assert_eq!(3, new_loc);
	}
}