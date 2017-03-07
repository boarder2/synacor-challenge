use vm::state;

pub trait Operation {
	fn run(&self, &mut state::VMState);
}