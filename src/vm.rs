use ops;

pub fn run(mut memory: &mut Vec<u16>) {
	let mut current_op = 0;
	let mut registers = Vec::with_capacity(8);
	let mut stack = Vec::new();
	for _ in 0..8 {
		 registers.push(0);
	}
	let foo = memory.clone();
	println!("{:?}", foo);
	while let Some(new_op) = ops::run_op(current_op, &mut memory, &mut registers, &mut stack) {
		current_op = new_op;
		//println!("New op addr {:?}", new_op);
	}
}