use std::libc::{c_char, c_void};

// from visatype.h
type ViUInt64 = u64;
type ViInt64 = i64;
type ViUInt32 = u32;
type ViInt32 = i32;
type ViUInt16 = u16;
type ViInt16 = i16;
type ViUInt8 = u8;
type ViInt8 = i8;
type ViChar = c_char;
type ViByte = u8;

type ViAddr = &c_void;

type ViReal32 = f32;
type ViReal64 = f64;

type ViBuf = u8;
type ViString = &c_char;
type ViRsrc = &c_char;

type ViBoolean = u16;
type ViStatus = i32;
type ViVersion = u32;
type ViObject = u32;
type ViSession = u32;

type ViAttr = u32;
type ViConstString = &c_char;
//

// from visa.h
type ViEvent = ViObject;
// type ViEvent      _VI_PTR ViPEvent;
type ViFindList = ViObject;
// type ViFindList   _VI_PTR ViPFindList;

#[cfg(target_pointer_width = "64")]
type ViBusAddress = ViUInt64;
#[cfg(target_pointer_width = "64")]
type ViBusSize = ViUInt64;
#[cfg(target_pointer_width = "64")]
type ViAttrState = ViUInt64;

#[cfg(target_pointer_width = "32")]
type ViBusAddress = ViUInt32;
#[cfg(target_pointer_width = "32")]
type ViBusSize = ViUInt32;
#[cfg(target_pointer_width = "32")]
type ViAttrState = ViUInt32;

#[cfg(target_pointer_width = "64")]
type ViBusAddress64 = ViUInt64;
// type ViBusAddress64 _VI_PTR ViPBusAddress64;

type ViEventType = ViUInt32;
// type ViEventType  _VI_PTR ViPEventType;
// type ViEventType  _VI_PTR ViAEventType;
type ViPAttrState = &c_void;
// type ViAttr       _VI_PTR ViPAttr;
// type ViAttr       _VI_PTR ViAAttr;

type ViKeyId = ViString;
type ViPKeyId = ViPString;
type ViJobId = ViUInt32;
// type ViJobId      _VI_PTR ViPJobId;
type ViAccessMode = ViUInt32;
//type ViAccessMode _VI_PTR ViPAccessMode;
//type ViBusAddress _VI_PTR ViPBusAddress;
type ViEventFilter = ViUInt32;

// typedef va_list              ViVAList;
//
// typedef ViStatus (_VI_FUNCH _VI_PTR ViHndlr)
//    (ViSession vi, ViEventType eventType, ViEvent event, ViAddr userHandle);
// C functions with variable arguments not supported in 0.8  = but in 0.9pre).
