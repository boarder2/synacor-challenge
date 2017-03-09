use ops::operation::Operation;
use vm::state;
pub struct Out;

impl Operation for Out {
	fn run(&self, vm_state: &mut state::VMState) {
		let ci = vm_state.get_current_instruction();
		let ch = vm_state.get_mem_or_register_value(ci + 1) as u8 as char;
		vm_state.add_to_console_output(ch);
		print!("{}", ch);
		vm_state.set_current_instruction(ci + 2);
	}
}
