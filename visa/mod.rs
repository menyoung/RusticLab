use visatype::*;
mod visatype;

/*- Resource Manager Functions and Operations -------------------------------*/

fn viOpenDefaultRM (ViPSession vi) -> ViStatus;
fn viFindRsrc      (ViSession sesn, ViString expr, ViPFindList vi,
                                    ViPUInt32 retCnt, ViChar _VI_FAR desc[]) -> ViStatus;
fn viFindNext      (ViFindList vi, ViChar _VI_FAR desc[]) -> ViStatus;
fn viParseRsrc     (ViSession rmSesn, ViRsrc rsrcName,
                                    ViPUInt16 intfType, ViPUInt16 intfNum) -> ViStatus;
fn viParseRsrcEx   (ViSession rmSesn, ViRsrc rsrcName, ViPUInt16 intfType,
                                    ViPUInt16 intfNum, ViChar _VI_FAR rsrcClass[],
                                    ViChar _VI_FAR expandedUnaliasedName[],
                                    ViChar _VI_FAR aliasIfExists[]) -> ViStatus;
fn viOpen          (ViSession sesn, ViRsrc name, ViAccessMode mode,
                                    ViUInt32 timeout, ViPSession vi) -> ViStatus;

/*- Resource Template Operations --------------------------------------------*/

fn viClose         (ViObject vi) -> ViStatus;
fn viSetAttribute  (ViObject vi, ViAttr attrName, ViAttrState attrValue) -> ViStatus;
fn viGetAttribute  (ViObject vi, ViAttr attrName, void _VI_PTR attrValue) -> ViStatus;
fn viStatusDesc    (ViObject vi, ViStatus status, ViChar _VI_FAR desc[]) -> ViStatus;
fn viTerminate     (ViObject vi, ViUInt16 degree, ViJobId jobId) -> ViStatus;

fn viLock          (ViSession vi, ViAccessMode lockType, ViUInt32 timeout,
                                    ViKeyId requestedKey, ViChar _VI_FAR accessKey[]) -> ViStatus;
fn viUnlock        (ViSession vi) -> ViStatus;
fn viEnableEvent   (ViSession vi, ViEventType eventType, ViUInt16 mechanism,
                                    ViEventFilter context) -> ViStatus;
fn viDisableEvent  (ViSession vi, ViEventType eventType, ViUInt16 mechanism) -> ViStatus;
fn viDiscardEvents (ViSession vi, ViEventType eventType, ViUInt16 mechanism) -> ViStatus;
fn viWaitOnEvent   (ViSession vi, ViEventType inEventType, ViUInt32 timeout,
                                    ViPEventType outEventType, ViPEvent outContext) -> ViStatus;
fn viInstallHandler(ViSession vi, ViEventType eventType, ViHndlr handler,
                                    ViAddr userHandle) -> ViStatus;
fn viUninstallHandler(ViSession vi, ViEventType eventType, ViHndlr handler,
                                      ViAddr userHandle) -> ViStatus;

/*- Basic I/O Operations ----------------------------------------------------*/

fn viRead          (ViSession vi, ViPBuf buf, ViUInt32 cnt, ViPUInt32 retCnt) -> ViStatus;
fn viReadAsync     (ViSession vi, ViPBuf buf, ViUInt32 cnt, ViPJobId  jobId) -> ViStatus;
fn viReadToFile    (ViSession vi, ViConstString filename, ViUInt32 cnt,
                                    ViPUInt32 retCnt) -> ViStatus;
fn viWrite         (ViSession vi, ViBuf  buf, ViUInt32 cnt, ViPUInt32 retCnt) -> ViStatus;
fn viWriteAsync    (ViSession vi, ViBuf  buf, ViUInt32 cnt, ViPJobId  jobId) -> ViStatus;
fn viWriteFromFile (ViSession vi, ViConstString filename, ViUInt32 cnt,
                                    ViPUInt32 retCnt) -> ViStatus;
fn viAssertTrigger (ViSession vi, ViUInt16 protocol) -> ViStatus;
fn viReadSTB       (ViSession vi, ViPUInt16 status) -> ViStatus;
fn viClear         (ViSession vi) -> ViStatus;

/*- Formatted and Buffered I/O Operations -----------------------------------*/

fn viSetBuf        (ViSession vi, ViUInt16 mask, ViUInt32 size) -> ViStatus;
fn viFlush         (ViSession vi, ViUInt16 mask) -> ViStatus;

fn viBufWrite      (ViSession vi, ViBuf  buf, ViUInt32 cnt, ViPUInt32 retCnt) -> ViStatus;
fn viBufRead       (ViSession vi, ViPBuf buf, ViUInt32 cnt, ViPUInt32 retCnt) -> ViStatus;

fn _VI_FUNCC viPrintf        (ViSession vi, ViString writeFmt, ...) -> ViStatus;
fn viVPrintf       (ViSession vi, ViString writeFmt, ViVAList params) -> ViStatus;
fn _VI_FUNCC viSPrintf       (ViSession vi, ViPBuf buf, ViString writeFmt, ...) -> ViStatus;
fn viVSPrintf      (ViSession vi, ViPBuf buf, ViString writeFmt,
                                    ViVAList parms) -> ViStatus;

fn _VI_FUNCC viScanf         (ViSession vi, ViString readFmt, ...) -> ViStatus;
fn viVScanf        (ViSession vi, ViString readFmt, ViVAList params) -> ViStatus;
fn _VI_FUNCC viSScanf        (ViSession vi, ViBuf buf, ViString readFmt, ...) -> ViStatus;
fn viVSScanf       (ViSession vi, ViBuf buf, ViString readFmt,
                                    ViVAList parms) -> ViStatus;

fn _VI_FUNCC viQueryf        (ViSession vi, ViString writeFmt, ViString readFmt, ...) -> ViStatus;
fn viVQueryf       (ViSession vi, ViString writeFmt, ViString readFmt, 
                                    ViVAList params) -> ViStatus;

/*- Memory I/O Operations ---------------------------------------------------*/

fn viIn8           (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt8  val8) -> ViStatus;
fn viOut8          (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt8   val8) -> ViStatus;
fn viIn16          (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt16 val16) -> ViStatus;
fn viOut16         (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt16  val16) -> ViStatus;
fn viIn32          (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt32 val32) -> ViStatus;
fn viOut32         (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt32  val32) -> ViStatus;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viIn64          (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViPUInt64 val64) -> ViStatus;
fn viOut64         (ViSession vi, ViUInt16 space,
                                    ViBusAddress offset, ViUInt64  val64) -> ViStatus;

fn viIn8Ex         (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt8  val8) -> ViStatus;
fn viOut8Ex        (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt8   val8) -> ViStatus;
fn viIn16Ex        (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt16 val16) -> ViStatus;
fn viOut16Ex       (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt16  val16) -> ViStatus;
fn viIn32Ex        (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt32 val32) -> ViStatus;
fn viOut32Ex       (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt32  val32) -> ViStatus;
fn viIn64Ex        (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViPUInt64 val64) -> ViStatus;
fn viOut64Ex       (ViSession vi, ViUInt16 space,
                                    ViBusAddress64 offset, ViUInt64  val64) -> ViStatus;
#endif

fn viMoveIn8       (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt8  buf8) -> ViStatus;
fn viMoveOut8      (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt8  buf8) -> ViStatus;
fn viMoveIn16      (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt16 buf16) -> ViStatus;
fn viMoveOut16     (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt16 buf16) -> ViStatus;
fn viMoveIn32      (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt32 buf32) -> ViStatus;
fn viMoveOut32     (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt32 buf32) -> ViStatus;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMoveIn64      (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt64 buf64) -> ViStatus;
fn viMoveOut64     (ViSession vi, ViUInt16 space, ViBusAddress offset,
                                    ViBusSize length, ViAUInt64 buf64) -> ViStatus;

fn viMoveIn8Ex     (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt8  buf8) -> ViStatus;
fn viMoveOut8Ex    (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt8  buf8) -> ViStatus;
fn viMoveIn16Ex    (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt16 buf16) -> ViStatus;
fn viMoveOut16Ex   (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt16 buf16) -> ViStatus;
fn viMoveIn32Ex    (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt32 buf32) -> ViStatus;
fn viMoveOut32Ex   (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt32 buf32) -> ViStatus;
fn viMoveIn64Ex    (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt64 buf64) -> ViStatus;
fn viMoveOut64Ex   (ViSession vi, ViUInt16 space, ViBusAddress64 offset,
                                    ViBusSize length, ViAUInt64 buf64) -> ViStatus;
#endif

fn viMove          (ViSession vi, ViUInt16 srcSpace, ViBusAddress srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength) -> ViStatus; 
fn viMoveAsync     (ViSession vi, ViUInt16 srcSpace, ViBusAddress srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength, ViPJobId jobId) -> ViStatus;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMoveEx        (ViSession vi, ViUInt16 srcSpace, ViBusAddress64 srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress64 destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength) -> ViStatus; 
fn viMoveAsyncEx   (ViSession vi, ViUInt16 srcSpace, ViBusAddress64 srcOffset,
                                    ViUInt16 srcWidth, ViUInt16 destSpace, 
                                    ViBusAddress64 destOffset, ViUInt16 destWidth, 
                                    ViBusSize srcLength, ViPJobId jobId) -> ViStatus;
#endif

fn viMapAddress    (ViSession vi, ViUInt16 mapSpace, ViBusAddress mapOffset,
                                    ViBusSize mapSize, ViBoolean access,
                                    ViAddr suggested, ViPAddr address) -> ViStatus;
fn viUnmapAddress  (ViSession vi) -> ViStatus;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMapAddressEx  (ViSession vi, ViUInt16 mapSpace, ViBusAddress64 mapOffset,
                                    ViBusSize mapSize, ViBoolean access,
                                    ViAddr suggested, ViPAddr address) -> ViStatus;
#endif

fn viPeek8         (ViSession vi, ViAddr address, ViPUInt8  val8) -> ();
fn viPoke8         (ViSession vi, ViAddr address, ViUInt8   val8) -> ();
fn viPeek16        (ViSession vi, ViAddr address, ViPUInt16 val16) -> ();
fn viPoke16        (ViSession vi, ViAddr address, ViUInt16  val16) -> ();
fn viPeek32        (ViSession vi, ViAddr address, ViPUInt32 val32) -> ();
fn viPoke32        (ViSession vi, ViAddr address, ViUInt32  val32) -> ();

#if defined(_VI_INT64_UINT64_DEFINED)
fn viPeek64        (ViSession vi, ViAddr address, ViPUInt64 val64) -> ();
fn viPoke64        (ViSession vi, ViAddr address, ViUInt64  val64) -> ();
#endif

/*- Shared Memory Operations ------------------------------------------------*/

fn viMemAlloc      (ViSession vi, ViBusSize size, ViPBusAddress offset) -> ViStatus;
fn viMemFree       (ViSession vi, ViBusAddress offset) -> ViStatus;

#if defined(_VI_INT64_UINT64_DEFINED)
fn viMemAllocEx    (ViSession vi, ViBusSize size, ViPBusAddress64 offset) -> ViStatus;
fn viMemFreeEx     (ViSession vi, ViBusAddress64 offset) -> ViStatus;
#endif

/*- Interface Specific Operations -------------------------------------------*/

fn viGpibControlREN(ViSession vi, ViUInt16 mode) -> ViStatus;
fn viGpibControlATN(ViSession vi, ViUInt16 mode) -> ViStatus;
fn viGpibSendIFC   (ViSession vi) -> ViStatus;
fn viGpibCommand   (ViSession vi, ViBuf cmd, ViUInt32 cnt, ViPUInt32 retCnt) -> ViStatus;
fn viGpibPassControl(ViSession vi, ViUInt16 primAddr, ViUInt16 secAddr) -> ViStatus;

fn viVxiCommandQuery(ViSession vi, ViUInt16 mode, ViUInt32 cmd,
                                     ViPUInt32 response) -> ViStatus;
fn viAssertUtilSignal(ViSession vi, ViUInt16 line) -> ViStatus;
fn viAssertIntrSignal(ViSession vi, ViInt16 mode, ViUInt32 statusID) -> ViStatus;
fn viMapTrigger    (ViSession vi, ViInt16 trigSrc, ViInt16 trigDest, 
                                    ViUInt16 mode) -> ViStatus;
fn viUnmapTrigger  (ViSession vi, ViInt16 trigSrc, ViInt16 trigDest) -> ViStatus;
fn viUsbControlOut (ViSession vi, ViInt16 bmRequestType, ViInt16 bRequest,
                                    ViUInt16 wValue, ViUInt16 wIndex, ViUInt16 wLength,
                                    ViBuf buf) -> ViStatus;
fn viUsbControlIn  (ViSession vi, ViInt16 bmRequestType, ViInt16 bRequest,
                                    ViUInt16 wValue, ViUInt16 wIndex, ViUInt16 wLength,
                                    ViPBuf buf, ViPUInt16 retCnt) -> ViStatus;
fn viPxiReserveTriggers(ViSession vi, ViInt16 cnt, ViAInt16 trigBuses,
                                    ViAInt16 trigLines, ViPInt16 failureIndex) -> ViStatus;

