use std::libc::{c_char, c_void};
 
pub struct ViUInt64(u64);
pub struct ViInt64(i64);
pub struct ViUInt32(u32);
pub struct ViInt32(i32);
pub struct ViUInt16(u16);
pub struct ViInt16(i16);
pub struct ViUInt8(u8);
pub struct ViInt8(i8);
pub struct ViChar(c_char);
pub struct ViByte(u8);
pub struct ViAddr(~c_void);
pub struct ViReal32(f32);
pub struct ViReal64(f64);
pub struct ViBuf(u8);
pub struct ViString(~c_char);
pub struct ViRsrc(~c_char);
pub struct ViBoolean(u16);
pub struct ViStatus(i32);
pub struct ViVersion(u32);
pub struct ViObject(u32);
pub struct ViSession(u32);
pub struct ViAttr(u32);
pub struct ViEvent(u32);
pub struct ViFindList(u32);
pub struct ViBusAddress(u32);
pub struct ViBusSize(u32);
pub struct ViAttrState(u32);
pub struct ViBusAddress64(u64);
pub struct ViEventType(u32);
pub struct ViKeyId(~c_char);
pub struct ViJobId(u32);
pub struct ViAccessMode(u32);
pub struct ViEventFilter(u32);
// struct ViVAList(????);
// C functions with variable arguments not supported in 0.8 (but in 0.9pre).
