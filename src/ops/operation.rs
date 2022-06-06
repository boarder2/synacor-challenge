use vm::state;

pub trait Operation {
	fn run(&self, vm_state: &mut state::VMState);
}