use ops;
use ops::operation::Operation;
pub struct Wmem;

impl Operation for Wmem {
	fn len(&self) -> usize {
		3
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, ci: u16, mem: &mut Vec<u16>, reg: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		let addr = ops::get_mem_or_register_value(mem[ci as usize + 1], reg);
		let val = ops::get_mem_or_register_value(mem[ci as usize + 2], reg);
		if let Some(m) = mem.get_mut(addr as usize) {
			//println!("Setting memory {:?} to {:?}", addr, val);
			*m = val;
		}
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn wmem_mem() {
	// 	let op = Wmem;
	// 	let mut mem = vec![0, 3, 3, 0];
	// 	let expected = vec![3, 0, 0, 0, 0, 0, 0, 0];
	// 	let mut registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
	// 	op.run(0, &mut mem, &mut registers, &mut Vec::new());
	// 	assert_eq!(expected, registers);
	// }

	// #[test]
	// fn rmem_reg() {
	// 	let op = Wmem;
	// 	let mut mem = vec![0, 32768, 32769, 3];
	// 	let expected = vec![3, 3, 0, 0, 0, 0, 0, 0];
	// 	let mut registers = vec![0, 3, 0, 0, 0, 0, 0, 0];
	// 	op.run(0, &mut mem, &mut registers, &mut Vec::new());
	// 	assert_eq!(expected, registers);
	// }
}