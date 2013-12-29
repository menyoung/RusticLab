use std::libc::c_char;

/*- Resource Manager Functions and Operations -------------------------------*/

fn viOpenDefaultRM (vi: *mut u32) -> i32;
fn viFindRsrc      (u32 sesn: u32, ViString expr, ViPFindList vi,
                                    ViPUInt32 retCnt, ViChar  desc[]) -> i32;
fn viFindNext      (ViFindList vi, ViChar  desc[]) -> i32;
fn viParseRsrc     (u32 rmSesn, ViRsrc rsrcName,
                                    ViPUInt16 intfType, ViPUInt16 intfNum) -> i32;
fn viParseRsrcEx   (u32 rmSesn, ViRsrc rsrcName, ViPUInt16 intfType,
                                    ViPUInt16 intfNum, ViChar  rsrcClass[],
                                    ViChar  expandedUnaliasedName[],
                                    ViChar  aliasIfExists[]) -> i32;
fn viOpen          (u32 sesn, ViRsrc name, ViAccessMode mode,
                                    ViUInt32 timeout, *mut u32 vi) -> i32;

/*- Resource Template Operations --------------------------------------------*/

fn viClose         (ViObject vi) -> i32;
fn viSetAttribute  (ViObject vi, ViAttr attrName, ViAttrState attrValue) -> i32;
fn viGetAttribute  (ViObject vi, ViAttr attrName, void _VI_PTR attrValue) -> i32;
fn viStatusDesc    (ViObject vi, ViStatus status, ViChar  desc[]) -> i32;
fn viTerminate     (ViObject vi, ViUInt16 degree, ViJobId jobId) -> i32;

fn viLock          (u32 vi, ViAccessMode lockType, ViUInt32 timeout,
                                    ViKeyId requestedKey, ViChar  accessKey[]) -> i32;
fn viUnlock        (u32 vi) -> i32;
fn viEnableEvent   (u32 vi, ViEventType eventType, ViUInt16 mechanism,
                                    ViEventFilter context) -> i32;
fn viDisableEvent  (u32 vi, ViEventType eventType, ViUInt16 mechanism) -> i32;
fn viDiscardEvents (u32 vi, ViEventType eventType, ViUInt16 mechanism) -> i32;
fn viWaitOnEvent   (u32 vi, ViEventType inEventType, ViUInt32 timeout,
                                    ViPEventType outEventType, ViPEvent outContext) -> i32;
fn viInstallHandler(u32 vi, ViEventType eventType, ViHndlr handler,
                                    ViAddr userHandle) -> i32;
fn viUninstallHandler(u32 vi, ViEventType eventType, ViHndlr handler,
                                      ViAddr userHandle) -> i32;

/*- Basic I/O Operations ----------------------------------------------------*/

fn viRead          (u32 vi, ViPBuf buf, ViUInt32 cnt, ViPUInt32 retCnt) -> i32;
fn viReadAsync     (u32 vi, ViPBuf buf, ViUInt32 cnt, ViPJobId  jobId) -> i32;
fn viReadToFile    (u32 vi, ViConstString filename, ViUInt32 cnt,
                                    ViPUInt32 retCnt) -> i32;
fn viWrite         (u32 vi, ViBuf  buf, ViUInt32 cnt, ViPUInt32 retCnt) -> i32;
fn viWriteAsync    (u32 vi, ViBuf  buf, ViUInt32 cnt, ViPJobId  jobId) -> i32;
fn viWriteFromFile (u32 vi, ViConstString filename, ViUInt32 cnt,
                                    ViPUInt32 retCnt) -> i32;
fn viAssertTrigger (u32 vi, ViUInt16 protocol) -> i32;
fn viReadSTB       (u32 vi, ViPUInt16 status) -> i32;
fn viClear         (u32 vi) -> i32;

/*- Formatted and Buffered I/O Operations -----------------------------------*/

fn viSetBuf        (u32 vi, ViUInt16 mask, ViUInt32 size) -> i32;
fn viFlush         (u32 vi, ViUInt16 mask) -> i32;

fn viBufWrite      (u32 vi, ViBuf  buf, ViUInt32 cnt, ViPUInt32 retCnt) -> i32;
fn viBufRead       (u32 vi, ViPBuf buf, ViUInt32 cnt, ViPUInt32 retCnt) -> i32;

fn viPrintf        (u32 vi, ViString writeFmt, ...) -> i32;
fn viVPrintf       (u32 vi, ViString writeFmt, ViVAList params) -> i32;
fn viSPrintf       (u32 vi, ViPBuf buf, ViString writeFmt, ...) -> i32;
fn viVSPrintf      (u32 vi, ViPBuf buf, ViString writeFmt,
                                    ViVAList parms) -> i32;

fn viScanf         (u32 vi, ViString readFmt, ...) -> i32;
fn viVScanf        (u32 vi, ViString readFmt, ViVAList params) -> i32;
fn viSScanf        (u32 vi, ViBuf buf, ViString readFmt, ...) -> i32;
fn viVSScanf       (u32 vi, ViBuf buf, ViString readFmt,
                                    ViVAList parms) -> i32;

fn viQueryf        (u32 vi, ViString writeFmt, ViString readFmt, ...) -> i32;
fn viVQueryf       (u32 vi, ViString writeFmt, ViString readFmt, 
                                    ViVAList params) -> i32;

/*- Memory I/O Operations ---------------------------------------------------*/

fn viIn8           (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt8  val8) -> i32;
fn viOut8          (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt8   val8) -> i32;
fn viIn16          (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt16 val16) -> i32;
fn viOut16         (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt16  val16) -> i32;
fn viIn32          (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt32 val32) -> i32;
fn viOut32         (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt32  val32) -> i32;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viIn64          (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt64 val64) -> i32;
fn viOut64         (u32 vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt64  val64) -> i32;

fn viIn8Ex         (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt8  val8) -> i32;
fn viOut8Ex        (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt8   val8) -> i32;
fn viIn16Ex        (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt16 val16) -> i32;
fn viOut16Ex       (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt16  val16) -> i32;
fn viIn32Ex        (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt32 val32) -> i32;
fn viOut32Ex       (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt32  val32) -> i32;
fn viIn64Ex        (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt64 val64) -> i32;
fn viOut64Ex       (u32 vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt64  val64) -> i32;
#endif

fn viMoveIn8       (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt8  buf8) -> i32;
fn viMoveOut8      (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt8  buf8) -> i32;
fn viMoveIn16      (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt16 buf16) -> i32;
fn viMoveOut16     (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt16 buf16) -> i32;
fn viMoveIn32      (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt32 buf32) -> i32;
fn viMoveOut32     (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt32 buf32) -> i32;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMoveIn64      (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt64 buf64) -> i32;
fn viMoveOut64     (u32 vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt64 buf64) -> i32;

fn viMoveIn8Ex     (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt8  buf8) -> i32;
fn viMoveOut8Ex    (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt8  buf8) -> i32;
fn viMoveIn16Ex    (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt16 buf16) -> i32;
fn viMoveOut16Ex   (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt16 buf16) -> i32;
fn viMoveIn32Ex    (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt32 buf32) -> i32;
fn viMoveOut32Ex   (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt32 buf32) -> i32;
fn viMoveIn64Ex    (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt64 buf64) -> i32;
fn viMoveOut64Ex   (u32 vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt64 buf64) -> i32;
#endif

fn viMove          (u32 vi, ViUInt16 srcSpace, ViBusAddress srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength) -> i32; 
fn viMoveAsync     (u32 vi, ViUInt16 srcSpace, ViBusAddress srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength, ViPJobId jobId) -> i32;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMoveEx        (u32 vi, ViUInt16 srcSpace, ViBusAddress64 srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress64 destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength) -> i32; 
fn viMoveAsyncEx   (u32 vi, ViUInt16 srcSpace, ViBusAddress64 srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress64 destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength, ViPJobId jobId) -> i32;
#endif

fn viMapAddress    (u32 vi, ViUInt16 mapSpace, ViBusAddress mapOffset,
                                    ViBusSize mapSize, ViBoolean access,
                                    ViAddr suggested, ViPAddr address) -> i32;
fn viUnmapAddress  (u32 vi) -> i32;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMapAddressEx  (u32 vi, ViUInt16 mapSpace, ViBusAddress64 mapOffset,
                                    ViBusSize mapSize, ViBoolean access,
                                    ViAddr suggested, ViPAddr address) -> i32;
#endif

fn viPeek8         (u32 vi, ViAddr address, ViPUInt8  val8) -> ();
fn viPoke8         (u32 vi, ViAddr address, ViUInt8   val8) -> ();
fn viPeek16        (u32 vi, ViAddr address, ViPUInt16 val16) -> ();
fn viPoke16        (u32 vi, ViAddr address, ViUInt16  val16) -> ();
fn viPeek32        (u32 vi, ViAddr address, ViPUInt32 val32) -> ();
fn viPoke32        (u32 vi, ViAddr address, ViUInt32  val32) -> ();

#if defined(_VI_INT64_UINT64_DEFINED)
fn viPeek64        (u32 vi, ViAddr address, ViPUInt64 val64) -> ();
fn viPoke64        (u32 vi, ViAddr address, ViUInt64  val64) -> ();
#endif

/*- Shared Memory Operations ------------------------------------------------*/

fn viMemAlloc      (u32 vi, ViBusSize size, ViPBusAddress offset) -> i32;
fn viMemFree       (u32 vi, ViBusAddress offset) -> i32;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMemAllocEx    (u32 vi, ViBusSize size, ViPBusAddress64 offset) -> i32;
fn viMemFreeEx     (u32 vi, ViBusAddress64 offset) -> i32;
#endif

/*- Interface Specific Operations -------------------------------------------*/

fn viGpibControlREN(u32 vi, ViUInt16 mode) -> i32;
fn viGpibControlATN(u32 vi, ViUInt16 mode) -> i32;
fn viGpibSendIFC   (u32 vi) -> i32;
fn viGpibCommand   (u32 vi, ViBuf cmd, ViUInt32 cnt, ViPUInt32 retCnt) -> i32;
fn viGpibPassControl(u32 vi, ViUInt16 primAddr, ViUInt16 secAddr) -> i32;

fn viVxiCommandQuery(u32 vi, ViUInt16 mode, ViUInt32 cmd,
                                     ViPUInt32 response) -> i32;
fn viAssertUtilSignal(u32 vi, ViUInt16 line) -> i32;
fn viAssertIntrSignal(u32 vi, ViInt16 mode, ViUInt32 statusID) -> i32;
fn viMapTrigger    (u32 vi, ViInt16 trigSrc, ViInt16 trigDest, 
                                    ViUInt16 mode) -> i32;
fn viUnmapTrigger  (u32 vi, ViInt16 trigSrc, ViInt16 trigDest) -> i32;
fn viUsbControlOut (u32 vi, ViInt16 bmRequestType, ViInt16 bRequest,
                                    ViUInt16 wValue, ViUInt16 wIndex, ViUInt16 wLength,
                                    ViBuf buf) -> i32;
fn viUsbControlIn  (u32 vi, ViInt16 bmRequestType, ViInt16 bRequest,
                                    ViUInt16 wValue, ViUInt16 wIndex, ViUInt16 wLength,
                                    ViPBuf buf, ViPUInt16 retCnt) -> i32;
fn viPxiReserveTriggers(u32 vi, ViInt16 cnt, ViAInt16 trigBuses,
                                    ViAInt16 trigLines, ViPInt16 failureIndex) -> i32;

/*- The End -----------------------------------------------------------------*/
