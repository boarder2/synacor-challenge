use ops;
use ops::operation::Operation;
pub struct Rmem;

impl Operation for Rmem {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let val = mem[ops::get_mem_or_register_value(mem[ci as usize + 2], reg) as usize];
		let addr = mem[ci as usize + 1];
		ops::set_register(addr, reg, val);
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rmem_mem() {
		let op = Rmem;
		let mem = vec![0, 32768, 3, 3];
		let expected = vec![3, 0, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}

	#[test]
	fn rmem_reg() {
		let op = Rmem;
		let mem = vec![0, 32768, 32769, 3];
		let expected = vec![3, 3, 0, 0, 0, 0, 0, 0];
		let mut registers = vec![0, 3, 0, 0, 0, 0, 0, 0];
		op.run(0, &mem, &mut registers, &mut Vec::new());
		assert_eq!(expected, registers);
	}
}