#[derive(Debug)]
pub struct DebugState {
	instruction_breaks: Vec<u16>,
	instruction_type_breaks: Vec<u16>,
	memory_watches: Vec<u16>,
	stepping: bool,
}

impl DebugState {
	pub fn new() -> DebugState {
		DebugState {
			instruction_breaks: Vec::new(),
			instruction_type_breaks: Vec::new(),
			memory_watches: Vec::new(),
			stepping: true,
		}
	}

	pub fn add_instruciton_break(&mut self, instr: u16) {
		self.instruction_breaks.push(instr);
	}

	pub fn is_instruction_break(&self, instr: u16) -> bool {
		self.instruction_breaks.as_slice().contains(&instr)
	}

	pub fn remove_instruction_break(&mut self, instr: u16) {
		let pos = self.instruction_breaks.iter().position(|&x| x == instr);
		if let Some(index) = pos {
			self.instruction_breaks.remove(index);
		}
	}

	pub fn get_instruction_breaks(&self) -> Vec<u16> {
		self.instruction_breaks.clone()
	}

	pub fn add_instruciton_type_break(&mut self, instr: u16) {
		self.instruction_type_breaks.push(instr);
	}

	pub fn is_instruction_type_break(&self, instr: u16) -> bool {
		self.instruction_type_breaks.as_slice().contains(&instr)
	}

	pub fn remove_instruction_type_break(&mut self, instr: u16) {
		let pos = self.instruction_type_breaks.iter().position(|&x| x == instr);
		if let Some(index) = pos {
			self.instruction_type_breaks.remove(index);
		}
	}

	pub fn get_instruction_type_breaks(&self) -> Vec<u16> {
		self.instruction_type_breaks.clone()
	}

	pub fn add_memory_watch(&mut self, instr: u16) {
		self.memory_watches.push(instr);
	}

	// pub fn is_memory_watch(&self, instr: u16) -> bool {
	// 	self.memory_watches.as_slice().contains(&instr)
	// }

	pub fn remove_memory_watch(&mut self, instr: u16) {
		let pos = self.memory_watches.iter().position(|&x| x == instr);
		if let Some(index) = pos {
			self.memory_watches.remove(index);
		}
	}

	pub fn get_memory_watches(&self) -> Vec<u16> {
		self.memory_watches.clone()
	}

	pub fn is_stepping(&self) -> bool {
		self.stepping
	}

	pub fn set_stepping(&mut self, val: bool) {
		self.stepping = val;
	}
}