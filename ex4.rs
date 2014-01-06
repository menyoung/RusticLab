#[feature(globs)];

extern mod visa;

use visa::*;

fn main() {
	println(format!("Hello. VISA Version is {}.{}.{}.", VI_VERSION_MAJOR(VI_SPEC_VERSION) as int, VI_VERSION_MINOR(VI_SPEC_VERSION) as int, VI_VERSION_SUBMINOR(VI_SPEC_VERSION) as int));
	
	// initialize the system.
	let mut defaultRM: ViSession = ViSession(25813);
	match vi_open_default_RM() {
		(status, resource_manager) => {
			*defaultRM = *resource_manager;
			println(format!("Default Resource Manager is {}; Status = {}.", *defaultRM as int, *status as int));
			if (*status < VI_SUCCESS) {
				// error initializing; exit.
				return;
			}
		}
	}
	
	// open communication with gpib device at primary address 12.
	// no error checking!
	let mut instr: ViSession = ViSession(25814);
	match vi_open(defaultRM, "GPIB0::12::INSTR", ViAccessMode(0), 0) {
		(status, vi) => {
			*instr = *vi;
			println(format!("Instrument at address 12 is {}; Status = {}.", *instr as int, *status as int));
			if (*status < VI_SUCCESS) {
				// error opening instrument; exit.
				return;
			}
		}
	}
	
	// set the timeout for messages
	vi_set_attribute(ViObject(*instr), ViAttr(VI_ATTR_TMO_VALUE), ViAttrState(5000));
	// clear the instrument
	vi_clear(instr);

	// ask the device for identification
	match vi_write_str(instr, "*IDN?\n") {
		(status, retCnt) => {
			println(format!("Upon write operation, RetCount = {}; Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error writing to instrument; exit.
				return;
			}
		}
	}
	
	// read the response
	let MAX_CNT = 200;
	match vi_read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(format!("Message was {} bytes long. Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}

	vi_write_str(instr, "MEAS:VOLT:DC?");
	match vi_read_str(instr, MAX_CNT) {
		(status, msg, retCnt) => {
			println(msg);
			println(format!("Message was {} bytes long. Status = {}.", retCnt, *status as int));
			if (*status < VI_SUCCESS) {
				// error reading from instrument; exit.
				return;
			}
		}
	}

	vi_close(instr);
	vi_close(defaultRM);
}
