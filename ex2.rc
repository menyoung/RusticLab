// use std::libc::{c_int, size_t};

#[link_args = "-framework VISA"]
extern {
	fn viOpenDefaultRM(vi: *mut u32) -> i32;
	fn viOpen(sesn: u32, name: *i8, mode: u32, timeout: u32, vi: *mut u32) -> i32; 
	fn viSetAttribute(vi: u32, attrName: u32, attrValue: u64) -> i32;
	fn viWrite(vi: u32, buf: *u8, cnt: u32, retCnt: *mut u32) -> i32;
	fn viRead(vi: u32, buf: *u8, cnt: u32, retCnt: *mut u32) -> i32;
	fn viClose(vi: u32) -> i32;
}

struct ViStatus(i32);
struct ViSession(u32);
struct ViUInt32(u32);
struct ViChar(u8);
struct ViRsrc(*i8);
struct ViAccessMode(u32);
struct ViObject(u32);
struct ViAttr(u32);
struct ViAttrState(u64);
struct ViBuf(*u8);

#[fixed_stack_segment]
// #[nolink]
fn main() {
	// constants. not mutable!
	let VI_SUCCESS: i32 = 0;
	let VI_NULL: u32 = 0;
	let VI_ATTR_TMO_VALUE = 0x3FFF001Au32;
	
	println ("Hello.");
	let mut defaultRM: u32 = 25813;
	
	// initialize the system.
	let mut x = unsafe { viOpenDefaultRM(&mut defaultRM) };
	println(fmt!("Default Resource Manager is %d; Status = %d.", defaultRM as int, x as int));
	if (x < VI_SUCCESS) {
		// error initializing; exit.
		return;
	}
	
	// open communication with gpib devvice at primary address 12.
	// no error checking!
	let mut instr: u32 = 25814;
	let name = "GPIB0::12::INSTR";
	println(name);
	let c_name = name.to_c_str(); // TODO: learn pointers and strings.
	let mut x = unsafe { viOpen(defaultRM, c_name.unwrap(), VI_NULL, VI_NULL, &mut instr) };
	println(fmt!("VI at address 12 is %d; Status = %d.", instr as int, x as int));
	
	// set the timeout for messages
	let mut x = unsafe { viSetAttribute(instr, VI_ATTR_TMO_VALUE, 5000) as u64};
	
	// ask the device for identification
	let mut retCount: u32 = 25815;
	let message = "*IDN?\n";
	let c_message = message.to_c_str();
	let mut x = unsafe { viWrite(instr, c_message.unwrap() as *u8, 6, &mut retCount) }; 
	
	// read the response
	let MAX_CNT: u32 = 200;
	let mut buffer: ~[u8] = std::vec::from_elem(MAX_CNT as uint, 0u8);
	let mut x = unsafe { viRead(instr, std::vec::raw::to_ptr(buffer), MAX_CNT, &mut retCount) };
	println(buffer.slice(0,retCount as uint).to_ascii().to_str_ascii());
	
	let mut x = unsafe { viClose(instr) };
	let mut x = unsafe { viClose(defaultRM) };
}
