#[feature(globs)];
use visatype::*;
use visadef::*;
use visafn::*;
mod visatype;
mod visadef;
mod visafn;

pub fn open_default_RM() -> (ViStatus, ViSession) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut resource_manager: ViSession = ViSession(0);
	unsafe { *status = viOpenDefaultRM(&mut *resource_manager) };
	(status, resource_manager)
}
pub fn open(rsrc_mngr: ViSession, name: &str, mode: ViAccessMode, timeout: uint) -> (ViStatus, ViSession) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut instr: ViSession = ViSession(0);
	unsafe { *status = viOpen(*rsrc_mngr, name.to_c_str().unwrap(), *mode, timeout as u32, &mut *instr) };
	(status, instr)
}
pub fn set_attribute(vi: ViObject, attrName: ViAttr, attrValue: ViAttrState) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe { *status = viSetAttribute(*vi, *attrName, *attrValue) };
	status
}
pub fn write(vi: ViSession, buf: &[u8]) -> (ViStatus, uint) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut retCnt: u32 = 0;
	unsafe { *status = viWrite(*vi, buf.as_ptr(), buf.iter().len() as u32, &mut retCnt) };
	(status, retCnt as uint)
}
pub fn write_str(vi: ViSession, buf: &str) -> (ViStatus, uint) {
	let c_str_buf = buf.to_c_str();
	let c_buf = c_str_buf.as_bytes();
	write(vi, c_buf.slice_to(c_buf.iter().len()-1))
}
pub fn read(vi: ViSession, cnt: uint) -> (ViStatus, ~[u8], uint) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut buf = ::std::vec::with_capacity(cnt);
	let mut retCnt: u32 = 0;
	unsafe {
		*status = viRead(*vi, buf.as_ptr(), cnt as u32, &mut retCnt);
		buf.set_len(retCnt as uint);
	};
	(status, buf, retCnt as uint)
}
pub fn read_str(vi: ViSession, cnt: uint) -> (ViStatus, ~str, uint) {
	match read(vi, cnt) {
		(status, buf, retCnt) => {
			(status, buf.into_ascii().into_str(), retCnt)
		}
	}
}
pub fn close(vi: ViSession) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe {
		*status = viClose(*vi);
	};
	status
}
pub fn clear(vi: ViSession) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe {
		*status = viClear(*vi);
	}
	status
}
