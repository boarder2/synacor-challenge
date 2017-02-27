use ops;
use ops::operation::Operation;
pub struct EqOp;

impl Operation for EqOp {
	fn len(&self) -> usize {
		4
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let v1 = ops::get_mem_or_register_value(mem[ci as usize + 2], reg);
		let v2 = ops::get_mem_or_register_value(mem[ci as usize + 3], reg);
		if v1 == v2 {
			ops::set_register(mem[ci as usize + 1], reg, 1);
		} else {
			ops::set_register(mem[ci as usize + 1], reg, 0);
		}
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eq_mem_not_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 1111, 1112];
		let expected = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn eq_mem_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 1111, 1111];
		let expected = vec![1, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn eq_reg_not_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 32769, 32770];
		let expected = vec![0, 12, 14, 0, 0, 0, 0, 0];
		let mut registers = vec![1, 12, 14, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn eq_reg_equal() {
		let op = EqOp;
		let mem = vec![4, 32768, 32769, 32770];
		let expected = vec![1, 12, 12, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 12, 12, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}