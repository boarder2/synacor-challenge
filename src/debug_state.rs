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

	pub fn add_instruction_break(&mut self, instr: u16) {
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

	pub fn add_instruction_type_break(&mut self, instr: u16) {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn stepping() {
		let mut ds = DebugState::new();
		ds.set_stepping(false);
		assert_eq!(false, ds.is_stepping());
	}

	#[test]
	fn memory_watches() {
		let mut ds = DebugState::new();
		let mut watches = vec![1, 2, 3];
		for w in watches.clone() {
			ds.add_memory_watch(w);
		}
		assert_eq!(watches, ds.get_memory_watches());
		ds.remove_memory_watch(watches.pop().unwrap());
		assert_eq!(watches, ds.get_memory_watches());
	}

	#[test]
	fn instruction_type_breaks() {
		let mut ds = DebugState::new();
		let mut arr = vec![1, 2, 3];
		for a in arr.clone() {
			ds.add_instruction_type_break(a);
		}
		assert_eq!(arr, ds.get_instruction_type_breaks());
		ds.remove_instruction_type_break(arr.pop().unwrap());
		assert_eq!(arr, ds.get_instruction_type_breaks());
		assert_eq!(true, ds.is_instruction_type_break(arr[0]));
		assert_eq!(false, ds.is_instruction_type_break(1234));
	}

	#[test]
	fn instruction_breaks() {
		let mut ds = DebugState::new();
		let mut arr = vec![1, 2, 3];
		for a in arr.clone() {
			ds.add_instruction_break(a);
		}
		assert_eq!(arr, ds.get_instruction_breaks());
		ds.remove_instruction_break(arr.pop().unwrap());
		assert_eq!(arr, ds.get_instruction_breaks());
		assert_eq!(true, ds.is_instruction_break(arr[0]));
		assert_eq!(false, ds.is_instruction_break(1234));
	}
}