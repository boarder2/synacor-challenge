use ops;
use ops::operation::Operation;
pub struct Jt;

impl Operation for Jt {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		true
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let val = ops::get_mem_or_register_value(mem[ci as usize + 1], reg);
		if val != 0 {
			return ops::get_mem_or_register_value(mem[ci as usize + 2], reg) as usize;
		}
		ci as usize + self.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn jt_mem_zero() {
		let j = Jt;
		let mem = vec![7, 0, 1234];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(3, new_loc);
	}

	#[test]
	fn jt_mem_notzero() {
		let expected = 1234;
		let j = Jt;
		let mem = vec![7, 2, expected];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected as usize, new_loc);
	}

	#[test]
	fn jt_reg_zero() {
		let j = Jt;
		let mem = vec![7, 32768, 1234];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(3, new_loc);
	}

	#[test]
	fn jt_reg_notzero() {
		let expected = 1234;
		let j = Jt;
		let mem = vec![7, 32768, expected];
		let mut registers = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let new_loc = j.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected as usize, new_loc);
	}
}