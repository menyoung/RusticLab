//! RusticLab's visa Library
//!
//! provides functions that perform VISA operations by making the appropriate C calls through the linked VISA library in the system.
//! Tested with NI-VISA 5.4 for i686-apple-darwin Mac OS X 10.9.
//!
//! Compile: `rustc --target i686-apple-darwin visa.rs`

//#[feature(globs)]
//#[allow(dead_code)]

pub use visadef::*;
pub use visafn::*;
pub use visatype::*;
/// static constants for attributes, error codes, etc.
mod visadef;
/// the extern-defined foreign function interface to VISA library.
mod visafn;
/// special VISA types that represent status, sessions, etc.
mod visatype;

/// open the default resource manager.
pub fn vi_open_default_RM() -> (ViStatus, ViSession) {
	let mut status: i32;
	let mut resource_manager: u32 = 0;
	unsafe { status = viOpenDefaultRM(&mut resource_manager); }
	(ViStatus(status), ViSession(resource_manager))
}
pub fn vi_open(ViSession(RM): ViSession, name: &str, ViAccessMode(mode): ViAccessMode, timeout: uint) -> (ViStatus, ViSession) {
	let mut status: i32; // ViStatus = ViStatus(_VI_ERROR);
	let mut instr: u32 = 0; // ViSession = ViSession(0);
	unsafe { status = viOpen(RM, name.to_c_str().unwrap(), mode, timeout as u32, &mut instr); }
	(ViStatus(status), ViSession(instr))
}
pub fn vi_set_attribute(ViObject(vi): ViObject, ViAttr(attrName): ViAttr, ViAttrState(attrValue): ViAttrState) -> ViStatus {
	let mut status: i32;
	unsafe { status = viSetAttribute(vi, attrName, attrValue); }
	ViStatus(status)
}
pub fn vi_write(ViSession(vi): ViSession, buf: &[u8]) -> (ViStatus, uint) {
	let mut status: i32;
	let mut retCnt: u32 = 0;
	unsafe { status = viWrite(vi, buf.as_ptr(), buf.iter().len() as u32, &mut retCnt); }
	(ViStatus(status), retCnt as uint)
}
pub fn vi_write_str(vi: ViSession, buf: &str) -> (ViStatus, uint) {
	vi_write(vi, buf.as_bytes())
}
pub fn vi_read(ViSession(vi): ViSession, cnt: uint) -> (ViStatus, [u8], uint) {
	let mut status: i32;
	let mut buf = ::std::vec::with_capacity(cnt);
	let mut retCnt: u32 = 0;
	unsafe {
		status = viRead(vi, buf.as_ptr(), cnt as u32, &mut retCnt);
		buf.set_len(retCnt as uint);
	}
	(ViStatus(status), buf, retCnt as uint)
}
pub fn vi_read_str(vi: ViSession, cnt: uint) -> (ViStatus, str, uint) {
	match vi_read(vi, cnt) {
		(status, buf, retCnt) => {
			(status, buf.into_ascii().into_str(), retCnt)
		}
	}
}
pub fn vi_close(ViSession(vi): ViSession) -> ViStatus {
	let mut status: i32;
	unsafe { status = viClose(vi); }
	ViStatus(status)
}
pub fn vi_clear(ViSession(vi): ViSession) -> ViStatus {
	let mut status: i32;
	unsafe { status = viClear(vi); }
	ViStatus(status)
}
pub fn vi_status_desc(ViSession(vi): ViSession, ViStatus(status): ViStatus) -> (ViStatus, str) {
	let mut retStat: i32;
	let mut buf = ::std::vec::with_capacity(256); // "the size of the desc parameter should be at least 256 bytes."
	unsafe {
		buf.set_len(256);
		retStat = viStatusDesc(vi, status, buf.as_ptr());
		for i in range(0, 256) {
			if (buf[i] == 0) {
				buf.set_len(i);
				break;
			}
		}
	}
	(ViStatus(retStat), buf.into_ascii().into_str())
}
