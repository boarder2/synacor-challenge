use ops;
use ops::operation::Operation;
pub struct Add;

impl Operation for Add {
	fn len(&self) -> usize {
		4
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &mut Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let val1 = ops::get_mem_or_register_value(mem[ci as usize + 2], reg);
		let val2 = ops::get_mem_or_register_value(mem[ci as usize + 3], reg);
		let new_val = ((val1 + val2 ) % 32768) as u16;
		ops::set_register(mem[ci as usize + 1], reg, new_val);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_mem() {
		let a = Add;
		let mut mem = vec![9, 32768, 1111, 1111];
		let expected = vec![2222, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		a.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn add_mem_rollover() {
		let a = Add;
		let mut mem = vec![9, 32768, 32767, 32767];
		let expected = vec![32766, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		a.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn add_reg() {
		let a = Add;
		let mut mem = vec![9, 32768, 32769, 32770];
		let expected = vec![2222, 1111, 1111, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 1111, 1111, 0, 0, 0, 0, 0];
		a.run(0, &mut mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}