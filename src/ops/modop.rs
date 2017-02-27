use ops;
use ops::operation::Operation;
pub struct ModOp;

impl Operation for ModOp {
	fn len(&self) -> usize {
		4
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let val1 = ops::get_mem_or_register_value(mem[ci as usize + 2], reg);
		let val2 = ops::get_mem_or_register_value(mem[ci as usize + 3], reg);
		let new_val = val1 % val2;
		ops::set_register(mem[ci as usize + 1], reg, new_val);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn modop_mem() {
		let op = ModOp;
		let mem = vec![9, 32768, 3, 2];
		let expected = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn modop_reg() {
		let op = ModOp;
		let mem = vec![9, 32768, 32769, 32770];
		let expected = vec![1, 3, 2, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 3, 2, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}