use ops;

pub fn run(memory: Vec<u16>) {
	let mut current_op = 0;
	let mut registers = Vec::with_capacity(8);
	while let Some(new_op) = ops::run_op(current_op, &memory, &mut registers) {
		current_op = new_op;
	}
}