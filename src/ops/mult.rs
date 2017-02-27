use ops;
use ops::operation::Operation;
pub struct Mult;

impl Operation for Mult {
	fn len(&self) -> usize {
		4
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let val1 = ops::get_mem_or_register_value(mem[ci as usize + 2], reg);
		let val2 = ops::get_mem_or_register_value(mem[ci as usize + 3], reg);
		let new_val = ((val1 as u32 * val2 as u32) % 32768) as u16;
		ops::set_register(mem[ci as usize + 1], reg, new_val);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mult_mem() {
		let op = Mult;
		let mem = vec![9, 32768, 2, 6];
		let expected = vec![12, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn mult_mem_overflow() {
		let op = Mult;
		let mem = vec![9, 32768, 32767, 2];
		let expected = vec![32766, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn mult_reg() {
		let op = Mult;
		let mem = vec![9, 32768, 32769, 32770];
		let expected = vec![12, 2, 6, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 2, 6, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}