use visatype::*;
use visadef::*;
mod visatype;
mod visadef;
mod visafn;
mod visa;

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
	match visa::open(defaultRM, "GPIB0::12::INSTR", ViAccessMode(0), 0) {
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
	visa::set_attribute(ViObject(*instr), ViAttr(VI_ATTR_TMO_VALUE), ViAttrState(5000));
	
	// ask the device for identification
	match visa::write_str(instr, "*IDN?\n") {
		(status, retCnt) => {
			println(fmt!("Upon write operation, RetCount = %u; Status = %d.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error writing to instrument; exit.
				return;
			}
		}
	}
	
	// read the response
	let MAX_CNT = 200;
	match visa::read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(fmt!("That was %u bytes; Status = %d.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}

	visa::write_str(instr, "*TST?");
	match visa::read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
		}
	}

	visa::close(instr);
	visa::close(defaultRM);
}
