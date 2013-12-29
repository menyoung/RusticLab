use std::libc::{c_char, c_void};

pub static _VI_ERROR: i32 = 0x80000000;
pub static VI_SUCCESS: i32 = 0;
pub static VI_NULL: i32 = 0;
pub static VI_TRUE: i32 = 1;
pub static VI_FALSE: i32 = 0;
pub static VI_SPEC_VERSION: u32 = 0x00500100;

struct ViUInt64(u64);
struct ViInt64(i64);
struct ViUInt32(u32);
struct ViInt32(i32);
struct ViUInt16(u16);
struct ViInt16(i16);
struct ViUInt8(u8);
struct ViInt8(i8);
struct ViChar(c_char);
struct ViByte(u8);
struct ViAddr(~c_void);
struct ViReal32(f32);
struct ViReal64(f64);
struct ViBuf(u8);
struct ViString(~c_char);
struct ViRsrc(~c_char);
struct ViBoolean(u16);
struct ViStatus(i32);
struct ViVersion(u32);
struct ViObject(u32);
struct ViSession(u32);
struct ViAttr(u32);
struct ViEvent(u32);
struct ViFindList(u32);
struct ViBusAddress(uint);
struct ViBusSize(uint);
struct ViAttrState(uint);
struct ViBusAddress64(u64);
struct ViEventType(u32);
struct ViKeyId(~c_char);
struct ViJobId(u32);
struct ViAccessMode(u32);
struct ViEventFilter(u32);
// struct ViVAList(????);

/*- Attributes (platform independent size) ----------------------------------*/

pub static VI_ATTR_RSRC_CLASS                    = 0xBFFF0001u32;
pub static VI_ATTR_RSRC_NAME                     = 0xBFFF0002u32;
pub static VI_ATTR_RSRC_IMPL_VERSION             = 0x3FFF0003u32;
pub static VI_ATTR_RSRC_LOCK_STATE               = 0x3FFF0004u32;
pub static VI_ATTR_MAX_QUEUE_LENGTH              = 0x3FFF0005u32;
pub static VI_ATTR_USER_DATA_32                  = 0x3FFF0007u32;
pub static VI_ATTR_FDC_CHNL                      = 0x3FFF000Du32;
pub static VI_ATTR_FDC_MODE                      = 0x3FFF000Fu32;
pub static VI_ATTR_FDC_GEN_SIGNAL_EN             = 0x3FFF0011u32;
pub static VI_ATTR_FDC_USE_PAIR                  = 0x3FFF0013u32;
pub static VI_ATTR_SEND_END_EN                   = 0x3FFF0016u32;
pub static VI_ATTR_TERMCHAR                      = 0x3FFF0018u32;
pub static VI_ATTR_TMO_VALUE                     = 0x3FFF001Au32;
pub static VI_ATTR_GPIB_READDR_EN                = 0x3FFF001Bu32;
pub static VI_ATTR_IO_PROT                       = 0x3FFF001Cu32;
pub static VI_ATTR_DMA_ALLOW_EN                  = 0x3FFF001Eu32;
pub static VI_ATTR_ASRL_BAUD                     = 0x3FFF0021u32;
pub static VI_ATTR_ASRL_DATA_BITS                = 0x3FFF0022u32;
pub static VI_ATTR_ASRL_PARITY                   = 0x3FFF0023u32;
pub static VI_ATTR_ASRL_STOP_BITS                = 0x3FFF0024u32;
pub static VI_ATTR_ASRL_FLOW_CNTRL               = 0x3FFF0025u32;
pub static VI_ATTR_RD_BUF_OPER_MODE              = 0x3FFF002Au32;
pub static VI_ATTR_RD_BUF_SIZE                   = 0x3FFF002Bu32;
pub static VI_ATTR_WR_BUF_OPER_MODE              = 0x3FFF002Du32;
pub static VI_ATTR_WR_BUF_SIZE                   = 0x3FFF002Eu32;
pub static VI_ATTR_SUPPRESS_END_EN               = 0x3FFF0036u32;
pub static VI_ATTR_TERMCHAR_EN                   = 0x3FFF0038u32;
pub static VI_ATTR_DEST_ACCESS_PRIV              = 0x3FFF0039u32;
pub static VI_ATTR_DEST_BYTE_ORDER               = 0x3FFF003Au32;
pub static VI_ATTR_SRC_ACCESS_PRIV               = 0x3FFF003Cu32;
pub static VI_ATTR_SRC_BYTE_ORDER                = 0x3FFF003Du32;
pub static VI_ATTR_SRC_INCREMENT                 = 0x3FFF0040u32;
pub static VI_ATTR_DEST_INCREMENT                = 0x3FFF0041u32;
pub static VI_ATTR_WIN_ACCESS_PRIV               = 0x3FFF0045u32;
pub static VI_ATTR_WIN_BYTE_ORDER                = 0x3FFF0047u32;
pub static VI_ATTR_GPIB_ATN_STATE                = 0x3FFF0057u32;
pub static VI_ATTR_GPIB_ADDR_STATE               = 0x3FFF005Cu32;
pub static VI_ATTR_GPIB_CIC_STATE                = 0x3FFF005Eu32;
pub static VI_ATTR_GPIB_NDAC_STATE               = 0x3FFF0062u32;
pub static VI_ATTR_GPIB_SRQ_STATE                = 0x3FFF0067u32;
pub static VI_ATTR_GPIB_SYS_CNTRL_STATE          = 0x3FFF0068u32;
pub static VI_ATTR_GPIB_HS488_CBL_LEN            = 0x3FFF0069u32;
pub static VI_ATTR_CMDR_LA                       = 0x3FFF006Bu32;
pub static VI_ATTR_VXI_DEV_CLASS                 = 0x3FFF006Cu32;
pub static VI_ATTR_MAINFRAME_LA                  = 0x3FFF0070u32;
pub static VI_ATTR_MANF_NAME                     = 0xBFFF0072u32;
pub static VI_ATTR_MODEL_NAME                    = 0xBFFF0077u32;
pub static VI_ATTR_VXI_VME_INTR_STATUS           = 0x3FFF008Bu32;
pub static VI_ATTR_VXI_TRIG_STATUS               = 0x3FFF008Du32;
pub static VI_ATTR_VXI_VME_SYSFAIL_STATE         = 0x3FFF0094u32;
pub static VI_ATTR_WIN_BASE_ADDR_32              = 0x3FFF0098u32;
pub static VI_ATTR_WIN_SIZE_32                   = 0x3FFF009Au32;
pub static VI_ATTR_ASRL_AVAIL_NUM                = 0x3FFF00ACu32;
pub static VI_ATTR_MEM_BASE_32                   = 0x3FFF00ADu32;
pub static VI_ATTR_ASRL_CTS_STATE                = 0x3FFF00AEu32;
pub static VI_ATTR_ASRL_DCD_STATE                = 0x3FFF00AFu32;
pub static VI_ATTR_ASRL_DSR_STATE                = 0x3FFF00B1u32;
pub static VI_ATTR_ASRL_DTR_STATE                = 0x3FFF00B2u32;
pub static VI_ATTR_ASRL_END_IN                   = 0x3FFF00B3u32;
pub static VI_ATTR_ASRL_END_OUT                  = 0x3FFF00B4u32;
pub static VI_ATTR_ASRL_REPLACE_CHAR             = 0x3FFF00BEu32;
pub static VI_ATTR_ASRL_RI_STATE                 = 0x3FFF00BFu32;
pub static VI_ATTR_ASRL_RTS_STATE                = 0x3FFF00C0u32;
pub static VI_ATTR_ASRL_XON_CHAR                 = 0x3FFF00C1u32;
pub static VI_ATTR_ASRL_XOFF_CHAR                = 0x3FFF00C2u32;
pub static VI_ATTR_WIN_ACCESS                    = 0x3FFF00C3u32;
pub static VI_ATTR_RM_SESSION                    = 0x3FFF00C4u32;
pub static VI_ATTR_VXI_LA                        = 0x3FFF00D5u32;
pub static VI_ATTR_MANF_ID                       = 0x3FFF00D9u32;
pub static VI_ATTR_MEM_SIZE_32                   = 0x3FFF00DDu32;
pub static VI_ATTR_MEM_SPACE                     = 0x3FFF00DEu32;
pub static VI_ATTR_MODEL_CODE                    = 0x3FFF00DFu32;
pub static VI_ATTR_SLOT                          = 0x3FFF00E8u32;
pub static VI_ATTR_INTF_INST_NAME                = 0xBFFF00E9u32;
pub static VI_ATTR_IMMEDIATE_SERV                = 0x3FFF0100u32;
pub static VI_ATTR_INTF_PARENT_NUM               = 0x3FFF0101u32;
pub static VI_ATTR_RSRC_SPEC_VERSION             = 0x3FFF0170u32;
pub static VI_ATTR_INTF_TYPE                     = 0x3FFF0171u32;
pub static VI_ATTR_GPIB_PRIMARY_ADDR             = 0x3FFF0172u32;
pub static VI_ATTR_GPIB_SECONDARY_ADDR           = 0x3FFF0173u32;
pub static VI_ATTR_RSRC_MANF_NAME                = 0xBFFF0174u32;
pub static VI_ATTR_RSRC_MANF_ID                  = 0x3FFF0175u32;
pub static VI_ATTR_INTF_NUM                      = 0x3FFF0176u32;
pub static VI_ATTR_TRIG_ID                       = 0x3FFF0177u32;
pub static VI_ATTR_GPIB_REN_STATE                = 0x3FFF0181u32;
pub static VI_ATTR_GPIB_UNADDR_EN                = 0x3FFF0184u32;
pub static VI_ATTR_DEV_STATUS_BYTE               = 0x3FFF0189u32;
pub static VI_ATTR_FILE_APPEND_EN                = 0x3FFF0192u32;
pub static VI_ATTR_VXI_TRIG_SUPPORT              = 0x3FFF0194u32;
pub static VI_ATTR_TCPIP_ADDR                    = 0xBFFF0195u32;
pub static VI_ATTR_TCPIP_HOSTNAME                = 0xBFFF0196u32;
pub static VI_ATTR_TCPIP_PORT                    = 0x3FFF0197u32;
pub static VI_ATTR_TCPIP_DEVICE_NAME             = 0xBFFF0199u32;
pub static VI_ATTR_TCPIP_NODELAY                 = 0x3FFF019Au32;
pub static VI_ATTR_TCPIP_KEEPALIVE               = 0x3FFF019Bu32;
pub static VI_ATTR_4882_COMPLIANT                = 0x3FFF019Fu32;
pub static VI_ATTR_USB_SERIAL_NUM                = 0xBFFF01A0u32;
pub static VI_ATTR_USB_INTFC_NUM                 = 0x3FFF01A1u32;
pub static VI_ATTR_USB_PROTOCOL                  = 0x3FFF01A7u32;
pub static VI_ATTR_USB_MAX_INTR_SIZE             = 0x3FFF01AFu32;
pub static VI_ATTR_PXI_DEV_NUM                   = 0x3FFF0201u32;
pub static VI_ATTR_PXI_FUNC_NUM                  = 0x3FFF0202u32;
pub static VI_ATTR_PXI_BUS_NUM                   = 0x3FFF0205u32;
pub static VI_ATTR_PXI_CHASSIS                   = 0x3FFF0206u32;
pub static VI_ATTR_PXI_SLOTPATH                  = 0xBFFF0207u32;
pub static VI_ATTR_PXI_SLOT_LBUS_LEFT            = 0x3FFF0208u32;
pub static VI_ATTR_PXI_SLOT_LBUS_RIGHT           = 0x3FFF0209u32;
pub static VI_ATTR_PXI_TRIG_BUS                  = 0x3FFF020Au32;
pub static VI_ATTR_PXI_STAR_TRIG_BUS             = 0x3FFF020Bu32;
pub static VI_ATTR_PXI_STAR_TRIG_LINE            = 0x3FFF020Cu32;
pub static VI_ATTR_PXI_SRC_TRIG_BUS              = 0x3FFF020Du32;
pub static VI_ATTR_PXI_DEST_TRIG_BUS             = 0x3FFF020Eu32;
pub static VI_ATTR_PXI_MEM_TYPE_BAR0             = 0x3FFF0211u32;
pub static VI_ATTR_PXI_MEM_TYPE_BAR1             = 0x3FFF0212u32;
pub static VI_ATTR_PXI_MEM_TYPE_BAR2             = 0x3FFF0213u32;
pub static VI_ATTR_PXI_MEM_TYPE_BAR3             = 0x3FFF0214u32;
pub static VI_ATTR_PXI_MEM_TYPE_BAR4             = 0x3FFF0215u32;
pub static VI_ATTR_PXI_MEM_TYPE_BAR5             = 0x3FFF0216u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR0_32          = 0x3FFF0221u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR1_32          = 0x3FFF0222u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR2_32          = 0x3FFF0223u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR3_32          = 0x3FFF0224u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR4_32          = 0x3FFF0225u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR5_32          = 0x3FFF0226u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR0_64          = 0x3FFF0228u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR1_64          = 0x3FFF0229u32;
pub static VI_ATTR_PXI_MEM_BASE_BAR2_64          = 0x3FFF022Au32;
pub static VI_ATTR_PXI_MEM_BASE_BAR3_64          = 0x3FFF022Bu32;
pub static VI_ATTR_PXI_MEM_BASE_BAR4_64          = 0x3FFF022Cu32;
pub static VI_ATTR_PXI_MEM_BASE_BAR5_64          = 0x3FFF022Du32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0_32          = 0x3FFF0231u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1_32          = 0x3FFF0232u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2_32          = 0x3FFF0233u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3_32          = 0x3FFF0234u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4_32          = 0x3FFF0235u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5_32          = 0x3FFF0236u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0_64          = 0x3FFF0238u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1_64          = 0x3FFF0239u32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2_64          = 0x3FFF023Au32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3_64          = 0x3FFF023Bu32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4_64          = 0x3FFF023Cu32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5_64          = 0x3FFF023Du32;
pub static VI_ATTR_PXI_IS_EXPRESS                = 0x3FFF0240u32;
pub static VI_ATTR_PXI_SLOT_LWIDTH               = 0x3FFF0241u32;
pub static VI_ATTR_PXI_MAX_LWIDTH                = 0x3FFF0242u32;
pub static VI_ATTR_PXI_ACTUAL_LWIDTH             = 0x3FFF0243u32;
pub static VI_ATTR_PXI_DSTAR_BUS                 = 0x3FFF0244u32;
pub static VI_ATTR_PXI_DSTAR_SET                 = 0x3FFF0245u32;
pub static VI_ATTR_PXI_ALLOW_WRITE_COMBINE       = 0x3FFF0246u32;
pub static VI_ATTR_TCPIP_HISLIP_OVERLAP_EN       = 0x3FFF0300u32;
pub static VI_ATTR_TCPIP_HISLIP_VERSION          = 0x3FFF0301u32;
pub static VI_ATTR_TCPIP_HISLIP_MAX_MESSAGE_KB   = 0x3FFF0302u32;
pub static VI_ATTR_TCPIP_IS_HISLIP               = 0x3FFF0303u32;

pub static VI_ATTR_JOB_ID                        = 0x3FFF4006u32;
pub static VI_ATTR_EVENT_TYPE                    = 0x3FFF4010u32;
pub static VI_ATTR_SIGP_STATUS_ID                = 0x3FFF4011u32;
pub static VI_ATTR_RECV_TRIG_ID                  = 0x3FFF4012u32;
pub static VI_ATTR_INTR_STATUS_ID                = 0x3FFF4023u32;
pub static VI_ATTR_STATUS                        = 0x3FFF4025u32;
pub static VI_ATTR_RET_COUNT_32                  = 0x3FFF4026u32;
pub static VI_ATTR_BUFFER                        = 0x3FFF4027u32;
pub static VI_ATTR_RECV_INTR_LEVEL               = 0x3FFF4041u32;
pub static VI_ATTR_OPER_NAME                     = 0xBFFF4042u32;
pub static VI_ATTR_GPIB_RECV_CIC_STATE           = 0x3FFF4193u32;
pub static VI_ATTR_RECV_TCPIP_ADDR               = 0xBFFF4198u32;
pub static VI_ATTR_USB_RECV_INTR_SIZE            = 0x3FFF41B0u32;
pub static VI_ATTR_USB_RECV_INTR_DATA            = 0xBFFF41B1u32;
pub static VI_ATTR_PXI_RECV_INTR_SEQ             = 0x3FFF4240u32;
pub static VI_ATTR_PXI_RECV_INTR_DATA            = 0x3FFF4241u32;

/*- Attributes (platform dependent size)------------------------------------*/

#if defined(_VI_INT64_UINT64_DEFINED) && defined(_VISA_ENV_IS_64_BIT)
pub static VI_ATTR_USER_DATA_64                  = 0x3FFF000Au32;
pub static VI_ATTR_RET_COUNT_64                  = 0x3FFF4028u32;
pub static VI_ATTR_USER_DATA                     = VI_ATTR_USER_DATA_64;
pub static VI_ATTR_RET_COUNT                     = VI_ATTR_RET_COUNT_64;
#else
pub static VI_ATTR_USER_DATA                     = VI_ATTR_USER_DATA_32;
pub static VI_ATTR_RET_COUNT                     = VI_ATTR_RET_COUNT_32;
#endif

#if defined(_VI_INT64_UINT64_DEFINED;
pub static VI_ATTR_WIN_BASE_ADDR_64              = 0x3FFF009Bu32;
pub static VI_ATTR_WIN_SIZE_64                   = 0x3FFF009Cu32;
pub static VI_ATTR_MEM_BASE_64                   = 0x3FFF00D0u32;
pub static VI_ATTR_MEM_SIZE_64                   = 0x3FFF00D1u32;
#endif
#if defined(_VI_INT64_UINT64_DEFINED) && defined(_VISA_ENV_IS_64_BIT)
pub static VI_ATTR_WIN_BASE_ADDR                 = VI_ATTR_WIN_BASE_ADDR_64;
pub static VI_ATTR_WIN_SIZE                      = VI_ATTR_WIN_SIZE_64;
pub static VI_ATTR_MEM_BASE                      = VI_ATTR_MEM_BASE_64;
pub static VI_ATTR_MEM_SIZE                      = VI_ATTR_MEM_SIZE_64;
pub static VI_ATTR_PXI_MEM_BASE_BAR0             = VI_ATTR_PXI_MEM_BASE_BAR0_64;
pub static VI_ATTR_PXI_MEM_BASE_BAR1             = VI_ATTR_PXI_MEM_BASE_BAR1_64;
pub static VI_ATTR_PXI_MEM_BASE_BAR2             = VI_ATTR_PXI_MEM_BASE_BAR2_64;
pub static VI_ATTR_PXI_MEM_BASE_BAR3             = VI_ATTR_PXI_MEM_BASE_BAR3_64;
pub static VI_ATTR_PXI_MEM_BASE_BAR4             = VI_ATTR_PXI_MEM_BASE_BAR4_64;
pub static VI_ATTR_PXI_MEM_BASE_BAR5             = VI_ATTR_PXI_MEM_BASE_BAR5_64;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0             = VI_ATTR_PXI_MEM_SIZE_BAR0_64;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1             = VI_ATTR_PXI_MEM_SIZE_BAR1_64;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2             = VI_ATTR_PXI_MEM_SIZE_BAR2_64;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3             = VI_ATTR_PXI_MEM_SIZE_BAR3_64;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4             = VI_ATTR_PXI_MEM_SIZE_BAR4_64;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5             = VI_ATTR_PXI_MEM_SIZE_BAR5_64;
#else
pub static VI_ATTR_WIN_BASE_ADDR                 = VI_ATTR_WIN_BASE_ADDR_32;
pub static VI_ATTR_WIN_SIZE                      = VI_ATTR_WIN_SIZE_32;
pub static VI_ATTR_MEM_BASE                      = VI_ATTR_MEM_BASE_32;
pub static VI_ATTR_MEM_SIZE                      = VI_ATTR_MEM_SIZE_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR0             = VI_ATTR_PXI_MEM_BASE_BAR0_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR1             = VI_ATTR_PXI_MEM_BASE_BAR1_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR2             = VI_ATTR_PXI_MEM_BASE_BAR2_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR3             = VI_ATTR_PXI_MEM_BASE_BAR3_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR4             = VI_ATTR_PXI_MEM_BASE_BAR4_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR5             = VI_ATTR_PXI_MEM_BASE_BAR5_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0             = VI_ATTR_PXI_MEM_SIZE_BAR0_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1             = VI_ATTR_PXI_MEM_SIZE_BAR1_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2             = VI_ATTR_PXI_MEM_SIZE_BAR2_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3             = VI_ATTR_PXI_MEM_SIZE_BAR3_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4             = VI_ATTR_PXI_MEM_SIZE_BAR4_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5             = VI_ATTR_PXI_MEM_SIZE_BAR5_32;
#endif 

/*- Event Types -------------------------------------------------------------*/

pub static VI_EVENT_IO_COMPLETION                = 0x3FFF2009u32;
pub static VI_EVENT_TRIG                         = 0xBFFF200Au32;
pub static VI_EVENT_SERVICE_REQ                  = 0x3FFF200Bu32;
pub static VI_EVENT_CLEAR                        = 0x3FFF200Du32;
pub static VI_EVENT_EXCEPTION                    = 0xBFFF200Eu32;
pub static VI_EVENT_GPIB_CIC                     = 0x3FFF2012u32;
pub static VI_EVENT_GPIB_TALK                    = 0x3FFF2013u32;
pub static VI_EVENT_GPIB_LISTEN                  = 0x3FFF2014u32;
pub static VI_EVENT_VXI_VME_SYSFAIL              = 0x3FFF201Du32;
pub static VI_EVENT_VXI_VME_SYSRESET             = 0x3FFF201Eu32;
pub static VI_EVENT_VXI_SIGP                     = 0x3FFF2020u32;
pub static VI_EVENT_VXI_VME_INTR                 = 0xBFFF2021u32;
pub static VI_EVENT_PXI_INTR                     = 0x3FFF2022u32;
pub static VI_EVENT_TCPIP_CONNECT                = 0x3FFF2036u32;
pub static VI_EVENT_USB_INTR                     = 0x3FFF2037u32;

pub static VI_ALL_ENABLED_EVENTS                 = 0x3FFF7FFFu32;

/*- Completion and Error Codes ----------------------------------------------*/

pub static VI_SUCCESS_EVENT_EN                   = 0x3FFF0002i32; /* 3FFF0002,  1073676290 */
pub static VI_SUCCESS_EVENT_DIS                  = 0x3FFF0003i32; /* 3FFF0003,  1073676291 */
pub static VI_SUCCESS_QUEUE_EMPTY                = 0x3FFF0004i32; /* 3FFF0004,  1073676292 */
pub static VI_SUCCESS_TERM_CHAR                  = 0x3FFF0005i32; /* 3FFF0005,  1073676293 */
pub static VI_SUCCESS_MAX_CNT                    = 0x3FFF0006i32; /* 3FFF0006,  1073676294 */
pub static VI_SUCCESS_DEV_NPRESENT               = 0x3FFF007Di32; /* 3FFF007D,  1073676413 */
pub static VI_SUCCESS_TRIG_MAPPED                = 0x3FFF007Ei32; /* 3FFF007E,  1073676414 */
pub static VI_SUCCESS_QUEUE_NEMPTY               = 0x3FFF0080i32; /* 3FFF0080,  1073676416 */
pub static VI_SUCCESS_NCHAIN                     = 0x3FFF0098i32; /* 3FFF0098,  1073676440 */
pub static VI_SUCCESS_NESTED_SHARED              = 0x3FFF0099i32; /* 3FFF0099,  1073676441 */
pub static VI_SUCCESS_NESTED_EXCLUSIVE           = 0x3FFF009Ai32; /* 3FFF009A,  1073676442 */
pub static VI_SUCCESS_SYNC                       = 0x3FFF009Bi32; /* 3FFF009B,  1073676443 */

pub static VI_WARN_QUEUE_OVERFLOW                = 0x3FFF000Ci32; /* 3FFF000C,  1073676300 */
pub static VI_WARN_CONFIG_NLOADED                = 0x3FFF0077i32; /* 3FFF0077,  1073676407 */
pub static VI_WARN_NULL_OBJECT                   = 0x3FFF0082i32; /* 3FFF0082,  1073676418 */
pub static VI_WARN_NSUP_ATTR_STATE               = 0x3FFF0084i32; /* 3FFF0084,  1073676420 */
pub static VI_WARN_UNKNOWN_STATUS                = 0x3FFF0085i32; /* 3FFF0085,  1073676421 */
pub static VI_WARN_NSUP_BUF                      = 0x3FFF0088i32; /* 3FFF0088,  1073676424 */
pub static VI_WARN_EXT_FUNC_NIMPL                = 0x3FFF00A9i32; /* 3FFF00A9,  1073676457 */

pub static VI_ERROR_SYSTEM_ERROR       = _VI_ERROR+0x3FFF0000i32; /* BFFF0000, -1073807360 */
pub static VI_ERROR_INV_OBJECT         = _VI_ERROR+0x3FFF000Ei32; /* BFFF000E, -1073807346 */
pub static VI_ERROR_RSRC_LOCKED        = _VI_ERROR+0x3FFF000Fi32; /* BFFF000F, -1073807345 */
pub static VI_ERROR_INV_EXPR           = _VI_ERROR+0x3FFF0010i32; /* BFFF0010, -1073807344 */
pub static VI_ERROR_RSRC_NFOUND        = _VI_ERROR+0x3FFF0011i32; /* BFFF0011, -1073807343 */
pub static VI_ERROR_INV_RSRC_NAME      = _VI_ERROR+0x3FFF0012i32; /* BFFF0012, -1073807342 */
pub static VI_ERROR_INV_ACC_MODE       = _VI_ERROR+0x3FFF0013i32; /* BFFF0013, -1073807341 */
pub static VI_ERROR_TMO                = _VI_ERROR+0x3FFF0015i32; /* BFFF0015, -1073807339 */
pub static VI_ERROR_CLOSING_FAILED     = _VI_ERROR+0x3FFF0016i32; /* BFFF0016, -1073807338 */
pub static VI_ERROR_INV_DEGREE         = _VI_ERROR+0x3FFF001Bi32; /* BFFF001B, -1073807333 */
pub static VI_ERROR_INV_JOB_ID         = _VI_ERROR+0x3FFF001Ci32; /* BFFF001C, -1073807332 */
pub static VI_ERROR_NSUP_ATTR          = _VI_ERROR+0x3FFF001Di32; /* BFFF001D, -1073807331 */
pub static VI_ERROR_NSUP_ATTR_STATE    = _VI_ERROR+0x3FFF001Ei32; /* BFFF001E, -1073807330 */
pub static VI_ERROR_ATTR_READONLY      = _VI_ERROR+0x3FFF001Fi32; /* BFFF001F, -1073807329 */
pub static VI_ERROR_INV_LOCK_TYPE      = _VI_ERROR+0x3FFF0020i32; /* BFFF0020, -1073807328 */
pub static VI_ERROR_INV_ACCESS_KEY     = _VI_ERROR+0x3FFF0021i32; /* BFFF0021, -1073807327 */
pub static VI_ERROR_INV_EVENT          = _VI_ERROR+0x3FFF0026i32; /* BFFF0026, -1073807322 */
pub static VI_ERROR_INV_MECH           = _VI_ERROR+0x3FFF0027i32; /* BFFF0027, -1073807321 */
pub static VI_ERROR_HNDLR_NINSTALLED   = _VI_ERROR+0x3FFF0028i32; /* BFFF0028, -1073807320 */
pub static VI_ERROR_INV_HNDLR_REF      = _VI_ERROR+0x3FFF0029i32; /* BFFF0029, -1073807319 */
pub static VI_ERROR_INV_CONTEXT        = _VI_ERROR+0x3FFF002Ai32; /* BFFF002A, -1073807318 */
pub static VI_ERROR_QUEUE_OVERFLOW     = _VI_ERROR+0x3FFF002Di32; /* BFFF002D, -1073807315 */
pub static VI_ERROR_NENABLED           = _VI_ERROR+0x3FFF002Fi32; /* BFFF002F, -1073807313 */
pub static VI_ERROR_ABORT              = _VI_ERROR+0x3FFF0030i32; /* BFFF0030, -1073807312 */
pub static VI_ERROR_RAW_WR_PROT_VIOL   = _VI_ERROR+0x3FFF0034i32; /* BFFF0034, -1073807308 */
pub static VI_ERROR_RAW_RD_PROT_VIOL   = _VI_ERROR+0x3FFF0035i32; /* BFFF0035, -1073807307 */
pub static VI_ERROR_OUTP_PROT_VIOL     = _VI_ERROR+0x3FFF0036i32; /* BFFF0036, -1073807306 */
pub static VI_ERROR_INP_PROT_VIOL      = _VI_ERROR+0x3FFF0037i32; /* BFFF0037, -1073807305 */
pub static VI_ERROR_BERR               = _VI_ERROR+0x3FFF0038i32; /* BFFF0038, -1073807304 */
pub static VI_ERROR_IN_PROGRESS        = _VI_ERROR+0x3FFF0039i32; /* BFFF0039, -1073807303 */
pub static VI_ERROR_INV_SETUP          = _VI_ERROR+0x3FFF003Ai32; /* BFFF003A, -1073807302 */
pub static VI_ERROR_QUEUE_ERROR        = _VI_ERROR+0x3FFF003Bi32; /* BFFF003B, -1073807301 */
pub static VI_ERROR_ALLOC              = _VI_ERROR+0x3FFF003Ci32; /* BFFF003C, -1073807300 */
pub static VI_ERROR_INV_MASK           = _VI_ERROR+0x3FFF003Di32; /* BFFF003D, -1073807299 */
pub static VI_ERROR_IO                 = _VI_ERROR+0x3FFF003Ei32; /* BFFF003E, -1073807298 */
pub static VI_ERROR_INV_FMT            = _VI_ERROR+0x3FFF003Fi32; /* BFFF003F, -1073807297 */
pub static VI_ERROR_NSUP_FMT           = _VI_ERROR+0x3FFF0041i32; /* BFFF0041, -1073807295 */
pub static VI_ERROR_LINE_IN_USE        = _VI_ERROR+0x3FFF0042i32; /* BFFF0042, -1073807294 */
pub static VI_ERROR_NSUP_MODE          = _VI_ERROR+0x3FFF0046i32; /* BFFF0046, -1073807290 */
pub static VI_ERROR_SRQ_NOCCURRED      = _VI_ERROR+0x3FFF004Ai32; /* BFFF004A, -1073807286 */
pub static VI_ERROR_INV_SPACE          = _VI_ERROR+0x3FFF004Ei32; /* BFFF004E, -1073807282 */
pub static VI_ERROR_INV_OFFSET         = _VI_ERROR+0x3FFF0051i32; /* BFFF0051, -1073807279 */
pub static VI_ERROR_INV_WIDTH          = _VI_ERROR+0x3FFF0052i32; /* BFFF0052, -1073807278 */
pub static VI_ERROR_NSUP_OFFSET        = _VI_ERROR+0x3FFF0054i32; /* BFFF0054, -1073807276 */
pub static VI_ERROR_NSUP_VAR_WIDTH     = _VI_ERROR+0x3FFF0055i32; /* BFFF0055, -1073807275 */
pub static VI_ERROR_WINDOW_NMAPPED     = _VI_ERROR+0x3FFF0057i32; /* BFFF0057, -1073807273 */
pub static VI_ERROR_RESP_PENDING       = _VI_ERROR+0x3FFF0059i32; /* BFFF0059, -1073807271 */
pub static VI_ERROR_NLISTENERS         = _VI_ERROR+0x3FFF005Fi32; /* BFFF005F, -1073807265 */
pub static VI_ERROR_NCIC               = _VI_ERROR+0x3FFF0060i32; /* BFFF0060, -1073807264 */
pub static VI_ERROR_NSYS_CNTLR         = _VI_ERROR+0x3FFF0061i32; /* BFFF0061, -1073807263 */
pub static VI_ERROR_NSUP_OPER          = _VI_ERROR+0x3FFF0067i32; /* BFFF0067, -1073807257 */
pub static VI_ERROR_INTR_PENDING       = _VI_ERROR+0x3FFF0068i32; /* BFFF0068, -1073807256 */
pub static VI_ERROR_ASRL_PARITY        = _VI_ERROR+0x3FFF006Ai32; /* BFFF006A, -1073807254 */
pub static VI_ERROR_ASRL_FRAMING       = _VI_ERROR+0x3FFF006Bi32; /* BFFF006B, -1073807253 */
pub static VI_ERROR_ASRL_OVERRUN       = _VI_ERROR+0x3FFF006Ci32; /* BFFF006C, -1073807252 */
pub static VI_ERROR_TRIG_NMAPPED       = _VI_ERROR+0x3FFF006Ei32; /* BFFF006E, -1073807250 */
pub static VI_ERROR_NSUP_ALIGN_OFFSET  = _VI_ERROR+0x3FFF0070i32; /* BFFF0070, -1073807248 */
pub static VI_ERROR_USER_BUF           = _VI_ERROR+0x3FFF0071i32; /* BFFF0071, -1073807247 */
pub static VI_ERROR_RSRC_BUSY          = _VI_ERROR+0x3FFF0072i32; /* BFFF0072, -1073807246 */
pub static VI_ERROR_NSUP_WIDTH         = _VI_ERROR+0x3FFF0076i32; /* BFFF0076, -1073807242 */
pub static VI_ERROR_INV_PARAMETER      = _VI_ERROR+0x3FFF0078i32; /* BFFF0078, -1073807240 */
pub static VI_ERROR_INV_PROT           = _VI_ERROR+0x3FFF0079i32; /* BFFF0079, -1073807239 */
pub static VI_ERROR_INV_SIZE           = _VI_ERROR+0x3FFF007Bi32; /* BFFF007B, -1073807237 */
pub static VI_ERROR_WINDOW_MAPPED      = _VI_ERROR+0x3FFF0080i32; /* BFFF0080, -1073807232 */
pub static VI_ERROR_NIMPL_OPER         = _VI_ERROR+0x3FFF0081i32; /* BFFF0081, -1073807231 */
pub static VI_ERROR_INV_LENGTH         = _VI_ERROR+0x3FFF0083i32; /* BFFF0083, -1073807229 */
pub static VI_ERROR_INV_MODE           = _VI_ERROR+0x3FFF0091i32; /* BFFF0091, -1073807215 */
pub static VI_ERROR_SESN_NLOCKED       = _VI_ERROR+0x3FFF009Ci32; /* BFFF009C, -1073807204 */
pub static VI_ERROR_MEM_NSHARED        = _VI_ERROR+0x3FFF009Di32; /* BFFF009D, -1073807203 */
pub static VI_ERROR_LIBRARY_NFOUND     = _VI_ERROR+0x3FFF009Ei32; /* BFFF009E, -1073807202 */
pub static VI_ERROR_NSUP_INTR          = _VI_ERROR+0x3FFF009Fi32; /* BFFF009F, -1073807201 */
pub static VI_ERROR_INV_LINE           = _VI_ERROR+0x3FFF00A0i32; /* BFFF00A0, -1073807200 */
pub static VI_ERROR_FILE_ACCESS        = _VI_ERROR+0x3FFF00A1i32; /* BFFF00A1, -1073807199 */
pub static VI_ERROR_FILE_IO            = _VI_ERROR+0x3FFF00A2i32; /* BFFF00A2, -1073807198 */
pub static VI_ERROR_NSUP_LINE          = _VI_ERROR+0x3FFF00A3i32; /* BFFF00A3, -1073807197 */
pub static VI_ERROR_NSUP_MECH          = _VI_ERROR+0x3FFF00A4i32; /* BFFF00A4, -1073807196 */
pub static VI_ERROR_INTF_NUM_NCONFIG   = _VI_ERROR+0x3FFF00A5i32; /* BFFF00A5, -1073807195 */
pub static VI_ERROR_CONN_LOST          = _VI_ERROR+0x3FFF00A6i32; /* BFFF00A6, -1073807194 */
pub static VI_ERROR_MACHINE_NAVAIL     = _VI_ERROR+0x3FFF00A7i32; /* BFFF00A7, -1073807193 */
pub static VI_ERROR_NPERMISSION        = _VI_ERROR+0x3FFF00A8i32; /* BFFF00A8, -1073807192 */

/*- Other VISA Definitions --------------------------------------------------*/

pub static VI_VERSION_MAJOR(ver)       = (((ViVersion)ver) & 0xFFF00000u32) >> 20;
pub static VI_VERSION_MINOR(ver)       = (((ViVersion)ver) & 0x000FFF00u32) >>  8;
pub static VI_VERSION_SUBMINOR(ver)    = (((ViVersion)ver) & 0x000000FFu32);

pub static VI_FIND_BUFLEN              = 256;

pub static VI_INTF_GPIB                = 1;
pub static VI_INTF_VXI                 = 2;
pub static VI_INTF_GPIB_VXI            = 3;
pub static VI_INTF_ASRL                = 4;
pub static VI_INTF_PXI                 = 5;
pub static VI_INTF_TCPIP               = 6;
pub static VI_INTF_USB                 = 7;

pub static VI_PROT_NORMAL              = 1;
pub static VI_PROT_FDC                 = 2;
pub static VI_PROT_HS488               = 3;
pub static VI_PROT_4882_STRS           = 4;
pub static VI_PROT_USBTMC_VENDOR       = 5;

pub static VI_FDC_NORMAL               = 1;
pub static VI_FDC_STREAM               = 2;

pub static VI_LOCAL_SPACE              = 0;
pub static VI_A16_SPACE                = 1;
pub static VI_A24_SPACE                = 2;
pub static VI_A32_SPACE                = 3;
pub static VI_A64_SPACE                = 4;
pub static VI_PXI_ALLOC_SPACE          = 9;
pub static VI_PXI_CFG_SPACE            = 10;
pub static VI_PXI_BAR0_SPACE           = 11;
pub static VI_PXI_BAR1_SPACE           = 12;
pub static VI_PXI_BAR2_SPACE           = 13;
pub static VI_PXI_BAR3_SPACE           = 14;
pub static VI_PXI_BAR4_SPACE           = 15;
pub static VI_PXI_BAR5_SPACE           = 16;
pub static VI_OPAQUE_SPACE             = 0xFFFF;

pub static VI_UNKNOWN_LA               = -1;
pub static VI_UNKNOWN_SLOT             = -1;
pub static VI_UNKNOWN_LEVEL            = -1;
pub static VI_UNKNOWN_CHASSIS          = -1;

pub static VI_QUEUE                    = 1;
pub static VI_HNDLR                    = 2;
pub static VI_SUSPEND_HNDLR            = 4;
pub static VI_ALL_MECH                 = 0xFFFF;

pub static VI_ANY_HNDLR                = 0;

pub static VI_TRIG_ALL                 = -2;
pub static VI_TRIG_SW                  = -1;
pub static VI_TRIG_TTL0                = 0;
pub static VI_TRIG_TTL1                = 1;
pub static VI_TRIG_TTL2                = 2;
pub static VI_TRIG_TTL3                = 3;
pub static VI_TRIG_TTL4                = 4;
pub static VI_TRIG_TTL5                = 5;
pub static VI_TRIG_TTL6                = 6;
pub static VI_TRIG_TTL7                = 7;
pub static VI_TRIG_ECL0                = 8;
pub static VI_TRIG_ECL1                = 9;
pub static VI_TRIG_ECL2                = 10;
pub static VI_TRIG_ECL3                = 11;
pub static VI_TRIG_ECL4                = 12;
pub static VI_TRIG_ECL5                = 13;
pub static VI_TRIG_STAR_SLOT1          = 14;
pub static VI_TRIG_STAR_SLOT2          = 15;
pub static VI_TRIG_STAR_SLOT3          = 16;
pub static VI_TRIG_STAR_SLOT4          = 17;
pub static VI_TRIG_STAR_SLOT5          = 18;
pub static VI_TRIG_STAR_SLOT6          = 19;
pub static VI_TRIG_STAR_SLOT7          = 20;
pub static VI_TRIG_STAR_SLOT8          = 21;
pub static VI_TRIG_STAR_SLOT9          = 22;
pub static VI_TRIG_STAR_SLOT10         = 23;
pub static VI_TRIG_STAR_SLOT11         = 24;
pub static VI_TRIG_STAR_SLOT12         = 25;
pub static VI_TRIG_STAR_INSTR          = 26;
pub static VI_TRIG_PANEL_IN            = 27;
pub static VI_TRIG_PANEL_OUT           = 28;
pub static VI_TRIG_STAR_VXI0           = 29;
pub static VI_TRIG_STAR_VXI1           = 30;
pub static VI_TRIG_STAR_VXI2           = 31;

pub static VI_TRIG_PROT_DEFAULT        = 0;
pub static VI_TRIG_PROT_ON             = 1;
pub static VI_TRIG_PROT_OFF            = 2;
pub static VI_TRIG_PROT_SYNC           = 5;
pub static VI_TRIG_PROT_RESERVE        = 6;
pub static VI_TRIG_PROT_UNRESERVE      = 7;

pub static VI_READ_BUF                 = 1;
pub static VI_WRITE_BUF                = 2;
pub static VI_READ_BUF_DISCARD         = 4;
pub static VI_WRITE_BUF_DISCARD        = 8;
pub static VI_IO_IN_BUF                = 16;
pub static VI_IO_OUT_BUF               = 32;
pub static VI_IO_IN_BUF_DISCARD        = 64;
pub static VI_IO_OUT_BUF_DISCARD       = 128;

pub static VI_FLUSH_ON_ACCESS          = 1;
pub static VI_FLUSH_WHEN_FULL          = 2;
pub static VI_FLUSH_DISABLE            = 3;

pub static VI_NMAPPED                  = 1;
pub static VI_USE_OPERS                = 2;
pub static VI_DEREF_ADDR               = 3;
pub static VI_DEREF_ADDR_BYTE_SWAP     = 4;

pub static VI_TMO_IMMEDIATE            = 0i32;
pub static VI_TMO_INFINITE             = 0xFFFFFFFFu32;

pub static VI_NO_LOCK                  = 0;
pub static VI_EXCLUSIVE_LOCK           = 1;
pub static VI_SHARED_LOCK              = 2;
pub static VI_LOAD_CONFIG              = 4;

pub static VI_NO_SEC_ADDR              = 0xFFFF;

pub static VI_ASRL_PAR_NONE            = 0;
pub static VI_ASRL_PAR_ODD             = 1;
pub static VI_ASRL_PAR_EVEN            = 2;
pub static VI_ASRL_PAR_MARK            = 3;
pub static VI_ASRL_PAR_SPACE           = 4;

pub static VI_ASRL_STOP_ONE            = 10;
pub static VI_ASRL_STOP_ONE5           = 15;
pub static VI_ASRL_STOP_TWO            = 20;

pub static VI_ASRL_FLOW_NONE           = 0;
pub static VI_ASRL_FLOW_XON_XOFF       = 1;
pub static VI_ASRL_FLOW_RTS_CTS        = 2;
pub static VI_ASRL_FLOW_DTR_DSR        = 4;

pub static VI_ASRL_END_NONE            = 0;
pub static VI_ASRL_END_LAST_BIT        = 1;
pub static VI_ASRL_END_TERMCHAR        = 2;
pub static VI_ASRL_END_BREAK           = 3;

pub static VI_STATE_ASSERTED           = 1;
pub static VI_STATE_UNASSERTED         = 0;
pub static VI_STATE_UNKNOWN            = -1;

pub static VI_BIG_ENDIAN               = 0;
pub static VI_LITTLE_ENDIAN            = 1;

pub static VI_DATA_PRIV                = 0;
pub static VI_DATA_NPRIV               = 1;
pub static VI_PROG_PRIV                = 2;
pub static VI_PROG_NPRIV               = 3;
pub static VI_BLCK_PRIV                = 4;
pub static VI_BLCK_NPRIV               = 5;
pub static VI_D64_PRIV                 = 6;
pub static VI_D64_NPRIV                = 7;
pub static VI_D64_2EVME                = 8;
pub static VI_D64_SST160               = 9;
pub static VI_D64_SST267               = 10;
pub static VI_D64_SST320               = 11;

pub static VI_WIDTH_8                  = 1;
pub static VI_WIDTH_16                 = 2;
pub static VI_WIDTH_32                 = 4;
pub static VI_WIDTH_64                 = 8;

pub static VI_GPIB_REN_DEASSERT        = 0;
pub static VI_GPIB_REN_ASSERT          = 1;
pub static VI_GPIB_REN_DEASSERT_GTL    = 2;
pub static VI_GPIB_REN_ASSERT_ADDRESS  = 3;
pub static VI_GPIB_REN_ASSERT_LLO      = 4;
pub static VI_GPIB_REN_ASSERT_ADDRESS_LLO = 5;
pub static VI_GPIB_REN_ADDRESS_GTL     = 6;

pub static VI_GPIB_ATN_DEASSERT        = 0;
pub static VI_GPIB_ATN_ASSERT          = 1;
pub static VI_GPIB_ATN_DEASSERT_HANDSHAKE = 2;
pub static VI_GPIB_ATN_ASSERT_IMMEDIATE = 3;

pub static VI_GPIB_HS488_DISABLED      = 0;
pub static VI_GPIB_HS488_NIMPL         = -1;

pub static VI_GPIB_UNADDRESSED         = 0;
pub static VI_GPIB_TALKER              = 1;
pub static VI_GPIB_LISTENER            = 2;

pub static VI_VXI_CMD16                = 0x0200;
pub static VI_VXI_CMD16_RESP16         = 0x0202;
pub static VI_VXI_RESP16               = 0x0002;
pub static VI_VXI_CMD32                = 0x0400;
pub static VI_VXI_CMD32_RESP16         = 0x0402;
pub static VI_VXI_CMD32_RESP32         = 0x0404;
pub static VI_VXI_RESP32               = 0x0004;

pub static VI_ASSERT_SIGNAL            = -1;
pub static VI_ASSERT_USE_ASSIGNED      = 0;
pub static VI_ASSERT_IRQ1              = 1;
pub static VI_ASSERT_IRQ2              = 2;
pub static VI_ASSERT_IRQ3              = 3;
pub static VI_ASSERT_IRQ4              = 4;
pub static VI_ASSERT_IRQ5              = 5;
pub static VI_ASSERT_IRQ6              = 6;
pub static VI_ASSERT_IRQ7              = 7;

pub static VI_UTIL_ASSERT_SYSRESET     = 1;
pub static VI_UTIL_ASSERT_SYSFAIL      = 2;
pub static VI_UTIL_DEASSERT_SYSFAIL    = 3;

pub static VI_VXI_CLASS_MEMORY         = 0;
pub static VI_VXI_CLASS_EXTENDED       = 1;
pub static VI_VXI_CLASS_MESSAGE        = 2;
pub static VI_VXI_CLASS_REGISTER       = 3;
pub static VI_VXI_CLASS_OTHER          = 4;

pub static VI_PXI_ADDR_NONE            = 0;
pub static VI_PXI_ADDR_MEM             = 1;
pub static VI_PXI_ADDR_IO              = 2;
pub static VI_PXI_ADDR_CFG             = 3;

pub static VI_TRIG_UNKNOWN             = -1;

pub static VI_PXI_LBUS_UNKNOWN         = -1;
pub static VI_PXI_LBUS_NONE            = 0;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_0 = 1000;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_1 = 1001;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_2 = 1002;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_3 = 1003;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_4 = 1004;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_5 = 1005;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_6 = 1006;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_7 = 1007;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_8 = 1008;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_9 = 1009;
pub static VI_PXI_STAR_TRIG_CONTROLLER = 1413;

/*- Backward Compatibility Macros -------------------------------------------*/

pub static viGetDefaultRM(vi)          viOpenDefaultRM(vi);
pub static VI_ERROR_INV_SESSION        = VI_ERROR_INV_OBJECT;
pub static VI_INFINITE                 = VI_TMO_INFINITE;
pub static VI_NORMAL                   = VI_PROT_NORMAL;
pub static VI_FDC                      = VI_PROT_FDC;
pub static VI_HS488                    = VI_PROT_HS488;
pub static VI_ASRL488                  = VI_PROT_4882_STRS;
pub static VI_ASRL_IN_BUF              = VI_IO_IN_BUF;
pub static VI_ASRL_OUT_BUF             = VI_IO_OUT_BUF;
pub static VI_ASRL_IN_BUF_DISCARD      = VI_IO_IN_BUF_DISCARD;
pub static VI_ASRL_OUT_BUF_DISCARD     = VI_IO_OUT_BUF_DISCARD;

/*- The End -----------------------------------------------------------------*/
