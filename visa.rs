#[crate_id = "visa#0.1"];
#[crate_type = "lib"];

#[feature(globs)];
#[allow(dead_code)];

pub use visatype::*;
pub use visadef::*;
pub use visafn::*;
mod visatype;
mod visadef;
mod visafn;

pub fn vi_open_default_RM() -> (ViStatus, ViSession) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut resource_manager: ViSession = ViSession(0);
	unsafe { *status = viOpenDefaultRM(&mut *resource_manager) };
	(status, resource_manager)
}
pub fn vi_open(rsrc_mngr: ViSession, name: &str, mode: ViAccessMode, timeout: uint) -> (ViStatus, ViSession) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut instr: ViSession = ViSession(0);
	unsafe { *status = viOpen(*rsrc_mngr, name.to_c_str().unwrap(), *mode, timeout as u32, &mut *instr) };
	(status, instr)
}
pub fn vi_set_attribute(vi: ViObject, attrName: ViAttr, attrValue: ViAttrState) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe { *status = viSetAttribute(*vi, *attrName, *attrValue) };
	status
}
pub fn vi_write(vi: ViSession, buf: &[u8]) -> (ViStatus, uint) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut retCnt: u32 = 0;
	unsafe { *status = viWrite(*vi, buf.as_ptr(), buf.iter().len() as u32, &mut retCnt) };
	(status, retCnt as uint)
}
pub fn vi_write_str(vi: ViSession, buf: &str) -> (ViStatus, uint) {
	vi_write(vi, buf.as_bytes())
}
pub fn vi_read(vi: ViSession, cnt: uint) -> (ViStatus, ~[u8], uint) {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	let mut buf = ::std::vec::with_capacity(cnt);
	let mut retCnt: u32 = 0;
	unsafe {
		*status = viRead(*vi, buf.as_ptr(), cnt as u32, &mut retCnt);
		buf.set_len(retCnt as uint);
	};
	(status, buf, retCnt as uint)
}
pub fn vi_read_str(vi: ViSession, cnt: uint) -> (ViStatus, ~str, uint) {
	match vi_read(vi, cnt) {
		(status, buf, retCnt) => {
			(status, buf.into_ascii().into_str(), retCnt)
		}
	}
}
pub fn vi_close(vi: ViSession) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe {
		*status = viClose(*vi);
	};
	status
}
pub fn vi_clear(vi: ViSession) -> ViStatus {
	let mut status: ViStatus = ViStatus(_VI_ERROR);
	unsafe {
		*status = viClear(*vi);
	}
	status
}
