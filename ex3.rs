// use std::libc::{c_int, size_t};
use visatype::*;
use visadef::*;
use visafn::*;
mod visa;
mod visafn;
mod visadef;
mod visatype;

#[fixed_stack_segment]
fn main() {
	println(fmt!("Hello. VI Version is %d.%d.%d.", VI_VERSION_MAJOR(VI_SPEC_VERSION) as int, VI_VERSION_MINOR(VI_SPEC_VERSION) as int, VI_VERSION_SUBMINOR(VI_SPEC_VERSION) as int));
	
	// initialize the system.
	let mut defaultRM: ViSession = ViSession(25813);
	match visa::open_default_RM() {
		(status, resource_manager) => {
			*defaultRM = *resource_manager;
			println(fmt!("Default Resource Manager is %d; Status = %d.", *defaultRM as int, *status as int));
			if (*status < VI_SUCCESS) {
				// error initializing; exit.
				return;
			}
		}
	}
	
	// open communication with gpib device at primary address 12.
	// no error checking!
	let mut instr: ViSession = ViSession(25814);
	match visa::open(defaultRM, "GPIB0::12::INSTR", ViAccessMode(0), ViUInt32(0)) {
		(status, vi) => {
			*instr = *vi;
			println(fmt!("Instrument at address 12 is %d; Status = %d.", *instr as int, *status as int));
			if (*status < VI_SUCCESS) {
				// error opening instrument; exit.
				return;
			}
		}
	}
	
	// set the timeout for messages
	unsafe { viSetAttribute(*instr, VI_ATTR_TMO_VALUE, 5000) as u64};
	
	// ask the device for identification
	let mut retCount: u32 = 25815;
	let message = "*IDN?\n";
	let c_message = message.to_c_str();
	unsafe { viWrite(*instr, c_message.unwrap() as *u8, 6, &mut retCount) }; 
	
	// read the response
	let MAX_CNT: u32 = 200;
	let buffer: ~[u8] = std::vec::from_elem(MAX_CNT as uint, 0u8);
	unsafe { viRead(*instr, std::vec::raw::to_ptr(buffer), MAX_CNT, &mut retCount) };
	println(buffer.slice(0,retCount as uint).to_ascii().to_str_ascii());

	unsafe { viClose(*instr) };
	unsafe { viClose(*defaultRM) };
}
