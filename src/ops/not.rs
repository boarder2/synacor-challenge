use ops;
use ops::operation::Operation;
pub struct Not;

impl Operation for Not {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &mut Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let val = ops::get_mem_or_register_value(mem[ci as usize + 2], reg);
		let new_val = (!val) % 32768;
		ops::set_register(mem[ci as usize + 1], reg, new_val);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn not_mem() {
		let op = Not;
		let mut mem = vec![0, 32768, 0];
		let expected = vec![32767, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn not_reg() {
		let op = Not;
		let mut mem = vec![0, 32768, 32769];
		let expected = vec![32767, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}