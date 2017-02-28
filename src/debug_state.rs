#[derive(Debug)]
pub struct DebugState {
	 instruction_breaks: Vec<u16>,
	 stepping: bool,
}

impl DebugState {
	pub fn new() -> DebugState {
		DebugState {
			instruction_breaks: Vec::new(),
			stepping: true,
		}
	}

	pub fn add_instruciton_break(&mut self, instr: u16) {
		 self.instruction_breaks.push(instr);
	}

	pub fn is_instruction_break(&self, instr: u16) -> bool {
		 self.instruction_breaks.as_slice().contains(&instr)
	}

	pub fn is_stepping(&self) -> bool {
		self.stepping
	}

	pub fn set_stepping(&mut self, val: bool) {
		self.stepping = val;
	}
}