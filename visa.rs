// use std::libc::{c_int, size_t};
use visatype::*;
use visadef::*;
use visafn::*;
mod visafn;
mod visadef;
mod visatype;

#[fixed_stack_segment]
pub fn open_default_RM() -> (ViStatus, ViSession) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut resource_manager: ViSession = ViSession(0);
	unsafe { *status = viOpenDefaultRM(&mut *resource_manager) };
	(status, resource_manager)
}
#[fixed_stack_segment]
pub fn open(rsrc_mngr: ViSession, name: &str, mode: ViAccessMode, timeout: ViUInt32) -> (ViStatus, ViSession) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut instr: ViSession = ViSession(0);
	unsafe { *status = viOpen(*rsrc_mngr, name.to_c_str().unwrap(), *mode, *timeout, &mut *instr) };
	(status, instr)
}
#[fixed_stack_segment]
pub fn set_attribute(vi: ViObject, attrName: ViAttr, attrValue: ViAttrState) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe { *status = viSetAttribute(*vi, *attrName, *attrValue) };
	status
}
