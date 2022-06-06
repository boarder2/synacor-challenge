#[cfg(test)]
pub mod state_helper {
	use vm::state::VMState;

	pub fn generate_vm_state_mem_reg_stack(mem: Vec<u16>,
	                                       reg: Vec<u16>,
	                                       stack: Vec<u16>)
	                                       -> VMState {
		let mut state = VMState::new(mem);
		for (i, r) in reg.into_iter().enumerate() {
			state.set_register((i + 32768) as u16, r);
		}
		for s in stack {
			state.push_stack(s);
		}
		state
	}

	pub fn generate_vm_state_mem_reg(mem: Vec<u16>, reg: Vec<u16>) -> VMState {
		generate_vm_state_mem_reg_stack(mem, reg, Vec::new())
	}

	pub fn generate_vm_state_mem(mem: Vec<u16>) -> VMState {
		generate_vm_state_mem_reg(mem, vec![0, 0, 0, 0, 0, 0, 0, 0])
	}
}