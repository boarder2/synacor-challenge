use std::collections::VecDeque;
use std::io;
mod operation;
mod set;
mod push;
mod pop;
mod eqop;
mod gt;
mod jmp;
mod jt;
mod jf;
mod add;
mod mult;
mod modop;
mod and;
mod or;
mod not;
mod rmem;
mod wmem;
mod call;
mod ret;
mod out;
use vm::state;

pub fn run_op(vm_state: &mut state::VMState, input_buffer: &mut VecDeque<u8>) -> bool {

	if !vm_state.is_valid_memory_address(vm_state.get_current_instruction()) {
		return false;
	}

	// println!("Running op {:?}", memory[current_instruction as usize]);
	match vm_state.get_mem_or_register_value(vm_state.get_current_instruction()) {
		0 => return false,
		1 => run_op_local(set::Set, vm_state),
		2 => run_op_local(push::Push, vm_state),
		3 => run_op_local(pop::Pop, vm_state),
		4 => run_op_local(eqop::EqOp, vm_state),
		5 => run_op_local(gt::Gt, vm_state),
		6 => run_op_local(jmp::Jmp, vm_state),
		7 => run_op_local(jt::Jt, vm_state),
		8 => run_op_local(jf::Jf, vm_state),
		9 => run_op_local(add::Add, vm_state),
		10 => run_op_local(mult::Mult, vm_state),
		11 => run_op_local(modop::ModOp, vm_state),
		12 => run_op_local(and::And, vm_state),
		13 => run_op_local(or::Or, vm_state),
		14 => run_op_local(not::Not, vm_state),
		15 => run_op_local(rmem::Rmem, vm_state),
		16 => run_op_local(wmem::Wmem, vm_state),
		17 => run_op_local(call::Call, vm_state),
		18 => run_op_local(ret::Ret, vm_state),
		19 => run_op_local(out::Out, vm_state),
		20 => {
			let ci = vm_state.get_current_instruction();
			if input_buffer.is_empty() {
				loop {
					let mut buf = String::new();
					println!(">");
					io::stdin().read_line(&mut buf).unwrap();
					let is_custom_command = process_custom_command(buf.replace("\r", "")
						                                               .replace("\n", "")
						                                               .as_str(),
					                                               vm_state);
					if !is_custom_command {
						for ch in buf.chars() {
							if ch as u8 != 13 {
								input_buffer.push_back(ch as u8);
							}
						}
						break;
					}
				}
			}
			if let Some(ch) = input_buffer.pop_front() {
				let reg_raw = vm_state.get_mem_raw(ci + 1);
				vm_state.set_register(reg_raw, ch as u16);
			}
			vm_state.set_current_instruction(ci + 2);
		}
		21 => {
			let ci = vm_state.get_current_instruction();
			vm_state.set_current_instruction(ci + 1)
		}
		x => {
			println!("{:?} not implemented", x);
			return false;
		}
	}
	return true;
}

fn run_op_local<T: operation::Operation>(op: T, vm_state: &mut state::VMState) {
	op.run(vm_state);
}

fn process_custom_command(cmd: &str, vm_state: &mut state::VMState) -> bool {
	let args = cmd.trim().split(' ').collect::<Vec<&str>>();

	if args[0] == "save" {
		if args.len() < 2 || args[1].trim().is_empty() {
			println!("invalid save syntax");
		} else {
			vm_state.save_state(args[1]);
		}
		return true;
	} else if args[0] == "load" {
		if args.len() < 2 || args[1].trim().is_empty() {
			println!("invalid load syntax");
		} else {
			vm_state.load_state(args[1]);
		}
		return true;
	}
	return false;
}