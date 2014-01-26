// VISA special codes definitions

pub static _VI_ERROR: i32 = -0x80000000; // most negative integer, right?
pub static VI_SUCCESS: i32 = 0;
pub static VI_NULL: i32 = 0;
pub static VI_TRUE: i32 = 1;
pub static VI_FALSE: i32 = 0;
pub static VI_SPEC_VERSION: u32 = 0x00500100;

/*- Attributes (platform independent size) ----------------------------------*/

pub static VI_ATTR_RSRC_CLASS                   : u32 = 0xBFFF0001;
pub static VI_ATTR_RSRC_NAME                    : u32 = 0xBFFF0002;
pub static VI_ATTR_RSRC_IMPL_VERSION            : u32 = 0x3FFF0003;
pub static VI_ATTR_RSRC_LOCK_STATE              : u32 = 0x3FFF0004;
pub static VI_ATTR_MAX_QUEUE_LENGTH             : u32 = 0x3FFF0005;
pub static VI_ATTR_USER_DATA_32                 : u32 = 0x3FFF0007;
pub static VI_ATTR_FDC_CHNL                     : u32 = 0x3FFF000D;
pub static VI_ATTR_FDC_MODE                     : u32 = 0x3FFF000F;
pub static VI_ATTR_FDC_GEN_SIGNAL_EN            : u32 = 0x3FFF0011;
pub static VI_ATTR_FDC_USE_PAIR                 : u32 = 0x3FFF0013;
pub static VI_ATTR_SEND_END_EN                  : u32 = 0x3FFF0016;
pub static VI_ATTR_TERMCHAR                     : u32 = 0x3FFF0018;
pub static VI_ATTR_TMO_VALUE                    : u32 = 0x3FFF001A;
pub static VI_ATTR_GPIB_READDR_EN               : u32 = 0x3FFF001B;
pub static VI_ATTR_IO_PROT                      : u32 = 0x3FFF001C;
pub static VI_ATTR_DMA_ALLOW_EN                 : u32 = 0x3FFF001E;
pub static VI_ATTR_ASRL_BAUD                    : u32 = 0x3FFF0021;
pub static VI_ATTR_ASRL_DATA_BITS               : u32 = 0x3FFF0022;
pub static VI_ATTR_ASRL_PARITY                  : u32 = 0x3FFF0023;
pub static VI_ATTR_ASRL_STOP_BITS               : u32 = 0x3FFF0024;
pub static VI_ATTR_ASRL_FLOW_CNTRL              : u32 = 0x3FFF0025;
pub static VI_ATTR_RD_BUF_OPER_MODE             : u32 = 0x3FFF002A;
pub static VI_ATTR_RD_BUF_SIZE                  : u32 = 0x3FFF002B;
pub static VI_ATTR_WR_BUF_OPER_MODE             : u32 = 0x3FFF002D;
pub static VI_ATTR_WR_BUF_SIZE                  : u32 = 0x3FFF002E;
pub static VI_ATTR_SUPPRESS_END_EN              : u32 = 0x3FFF0036;
pub static VI_ATTR_TERMCHAR_EN                  : u32 = 0x3FFF0038;
pub static VI_ATTR_DEST_ACCESS_PRIV             : u32 = 0x3FFF0039;
pub static VI_ATTR_DEST_BYTE_ORDER              : u32 = 0x3FFF003A;
pub static VI_ATTR_SRC_ACCESS_PRIV              : u32 = 0x3FFF003C;
pub static VI_ATTR_SRC_BYTE_ORDER               : u32 = 0x3FFF003D;
pub static VI_ATTR_SRC_INCREMENT                : u32 = 0x3FFF0040;
pub static VI_ATTR_DEST_INCREMENT               : u32 = 0x3FFF0041;
pub static VI_ATTR_WIN_ACCESS_PRIV              : u32 = 0x3FFF0045;
pub static VI_ATTR_WIN_BYTE_ORDER               : u32 = 0x3FFF0047;
pub static VI_ATTR_GPIB_ATN_STATE               : u32 = 0x3FFF0057;
pub static VI_ATTR_GPIB_ADDR_STATE              : u32 = 0x3FFF005C;
pub static VI_ATTR_GPIB_CIC_STATE               : u32 = 0x3FFF005E;
pub static VI_ATTR_GPIB_NDAC_STATE              : u32 = 0x3FFF0062;
pub static VI_ATTR_GPIB_SRQ_STATE               : u32 = 0x3FFF0067;
pub static VI_ATTR_GPIB_SYS_CNTRL_STATE         : u32 = 0x3FFF0068;
pub static VI_ATTR_GPIB_HS488_CBL_LEN           : u32 = 0x3FFF0069;
pub static VI_ATTR_CMDR_LA                      : u32 = 0x3FFF006B;
pub static VI_ATTR_VXI_DEV_CLASS                : u32 = 0x3FFF006C;
pub static VI_ATTR_MAINFRAME_LA                 : u32 = 0x3FFF0070;
pub static VI_ATTR_MANF_NAME                    : u32 = 0xBFFF0072;
pub static VI_ATTR_MODEL_NAME                   : u32 = 0xBFFF0077;
pub static VI_ATTR_VXI_VME_INTR_STATUS          : u32 = 0x3FFF008B;
pub static VI_ATTR_VXI_TRIG_STATUS              : u32 = 0x3FFF008D;
pub static VI_ATTR_VXI_VME_SYSFAIL_STATE        : u32 = 0x3FFF0094;
pub static VI_ATTR_WIN_BASE_ADDR_32             : u32 = 0x3FFF0098;
pub static VI_ATTR_WIN_SIZE_32                  : u32 = 0x3FFF009A;
pub static VI_ATTR_ASRL_AVAIL_NUM               : u32 = 0x3FFF00AC;
pub static VI_ATTR_MEM_BASE_32                  : u32 = 0x3FFF00AD;
pub static VI_ATTR_ASRL_CTS_STATE               : u32 = 0x3FFF00AE;
pub static VI_ATTR_ASRL_DCD_STATE               : u32 = 0x3FFF00AF;
pub static VI_ATTR_ASRL_DSR_STATE               : u32 = 0x3FFF00B1;
pub static VI_ATTR_ASRL_DTR_STATE               : u32 = 0x3FFF00B2;
pub static VI_ATTR_ASRL_END_IN                  : u32 = 0x3FFF00B3;
pub static VI_ATTR_ASRL_END_OUT                 : u32 = 0x3FFF00B4;
pub static VI_ATTR_ASRL_REPLACE_CHAR            : u32 = 0x3FFF00BE;
pub static VI_ATTR_ASRL_RI_STATE                : u32 = 0x3FFF00BF;
pub static VI_ATTR_ASRL_RTS_STATE               : u32 = 0x3FFF00C0;
pub static VI_ATTR_ASRL_XON_CHAR                : u32 = 0x3FFF00C1;
pub static VI_ATTR_ASRL_XOFF_CHAR               : u32 = 0x3FFF00C2;
pub static VI_ATTR_WIN_ACCESS                   : u32 = 0x3FFF00C3;
pub static VI_ATTR_RM_SESSION                   : u32 = 0x3FFF00C4;
pub static VI_ATTR_VXI_LA                       : u32 = 0x3FFF00D5;
pub static VI_ATTR_MANF_ID                      : u32 = 0x3FFF00D9;
pub static VI_ATTR_MEM_SIZE_32                  : u32 = 0x3FFF00DD;
pub static VI_ATTR_MEM_SPACE                    : u32 = 0x3FFF00DE;
pub static VI_ATTR_MODEL_CODE                   : u32 = 0x3FFF00DF;
pub static VI_ATTR_SLOT                         : u32 = 0x3FFF00E8;
pub static VI_ATTR_INTF_INST_NAME               : u32 = 0xBFFF00E9;
pub static VI_ATTR_IMMEDIATE_SERV               : u32 = 0x3FFF0100;
pub static VI_ATTR_INTF_PARENT_NUM              : u32 = 0x3FFF0101;
pub static VI_ATTR_RSRC_SPEC_VERSION            : u32 = 0x3FFF0170;
pub static VI_ATTR_INTF_TYPE                    : u32 = 0x3FFF0171;
pub static VI_ATTR_GPIB_PRIMARY_ADDR            : u32 = 0x3FFF0172;
pub static VI_ATTR_GPIB_SECONDARY_ADDR          : u32 = 0x3FFF0173;
pub static VI_ATTR_RSRC_MANF_NAME               : u32 = 0xBFFF0174;
pub static VI_ATTR_RSRC_MANF_ID                 : u32 = 0x3FFF0175;
pub static VI_ATTR_INTF_NUM                     : u32 = 0x3FFF0176;
pub static VI_ATTR_TRIG_ID                      : u32 = 0x3FFF0177;
pub static VI_ATTR_GPIB_REN_STATE               : u32 = 0x3FFF0181;
pub static VI_ATTR_GPIB_UNADDR_EN               : u32 = 0x3FFF0184;
pub static VI_ATTR_DEV_STATUS_BYTE              : u32 = 0x3FFF0189;
pub static VI_ATTR_FILE_APPEND_EN               : u32 = 0x3FFF0192;
pub static VI_ATTR_VXI_TRIG_SUPPORT             : u32 = 0x3FFF0194;
pub static VI_ATTR_TCPIP_ADDR                   : u32 = 0xBFFF0195;
pub static VI_ATTR_TCPIP_HOSTNAME               : u32 = 0xBFFF0196;
pub static VI_ATTR_TCPIP_PORT                   : u32 = 0x3FFF0197;
pub static VI_ATTR_TCPIP_DEVICE_NAME            : u32 = 0xBFFF0199;
pub static VI_ATTR_TCPIP_NODELAY                : u32 = 0x3FFF019A;
pub static VI_ATTR_TCPIP_KEEPALIVE              : u32 = 0x3FFF019B;
pub static VI_ATTR_4882_COMPLIANT               : u32 = 0x3FFF019F;
pub static VI_ATTR_USB_SERIAL_NUM               : u32 = 0xBFFF01A0;
pub static VI_ATTR_USB_INTFC_NUM                : u32 = 0x3FFF01A1;
pub static VI_ATTR_USB_PROTOCOL                 : u32 = 0x3FFF01A7;
pub static VI_ATTR_USB_MAX_INTR_SIZE            : u32 = 0x3FFF01AF;
pub static VI_ATTR_PXI_DEV_NUM                  : u32 = 0x3FFF0201;
pub static VI_ATTR_PXI_FUNC_NUM                 : u32 = 0x3FFF0202;
pub static VI_ATTR_PXI_BUS_NUM                  : u32 = 0x3FFF0205;
pub static VI_ATTR_PXI_CHASSIS                  : u32 = 0x3FFF0206;
pub static VI_ATTR_PXI_SLOTPATH                 : u32 = 0xBFFF0207;
pub static VI_ATTR_PXI_SLOT_LBUS_LEFT           : u32 = 0x3FFF0208;
pub static VI_ATTR_PXI_SLOT_LBUS_RIGHT          : u32 = 0x3FFF0209;
pub static VI_ATTR_PXI_TRIG_BUS                 : u32 = 0x3FFF020A;
pub static VI_ATTR_PXI_STAR_TRIG_BUS            : u32 = 0x3FFF020B;
pub static VI_ATTR_PXI_STAR_TRIG_LINE           : u32 = 0x3FFF020C;
pub static VI_ATTR_PXI_SRC_TRIG_BUS             : u32 = 0x3FFF020D;
pub static VI_ATTR_PXI_DEST_TRIG_BUS            : u32 = 0x3FFF020E;
pub static VI_ATTR_PXI_MEM_TYPE_BAR0            : u32 = 0x3FFF0211;
pub static VI_ATTR_PXI_MEM_TYPE_BAR1            : u32 = 0x3FFF0212;
pub static VI_ATTR_PXI_MEM_TYPE_BAR2            : u32 = 0x3FFF0213;
pub static VI_ATTR_PXI_MEM_TYPE_BAR3            : u32 = 0x3FFF0214;
pub static VI_ATTR_PXI_MEM_TYPE_BAR4            : u32 = 0x3FFF0215;
pub static VI_ATTR_PXI_MEM_TYPE_BAR5            : u32 = 0x3FFF0216;
pub static VI_ATTR_PXI_MEM_BASE_BAR0_32         : u32 = 0x3FFF0221;
pub static VI_ATTR_PXI_MEM_BASE_BAR1_32         : u32 = 0x3FFF0222;
pub static VI_ATTR_PXI_MEM_BASE_BAR2_32         : u32 = 0x3FFF0223;
pub static VI_ATTR_PXI_MEM_BASE_BAR3_32         : u32 = 0x3FFF0224;
pub static VI_ATTR_PXI_MEM_BASE_BAR4_32         : u32 = 0x3FFF0225;
pub static VI_ATTR_PXI_MEM_BASE_BAR5_32         : u32 = 0x3FFF0226;
pub static VI_ATTR_PXI_MEM_BASE_BAR0_64         : u32 = 0x3FFF0228;
pub static VI_ATTR_PXI_MEM_BASE_BAR1_64         : u32 = 0x3FFF0229;
pub static VI_ATTR_PXI_MEM_BASE_BAR2_64         : u32 = 0x3FFF022A;
pub static VI_ATTR_PXI_MEM_BASE_BAR3_64         : u32 = 0x3FFF022B;
pub static VI_ATTR_PXI_MEM_BASE_BAR4_64         : u32 = 0x3FFF022C;
pub static VI_ATTR_PXI_MEM_BASE_BAR5_64         : u32 = 0x3FFF022D;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0_32         : u32 = 0x3FFF0231;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1_32         : u32 = 0x3FFF0232;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2_32         : u32 = 0x3FFF0233;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3_32         : u32 = 0x3FFF0234;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4_32         : u32 = 0x3FFF0235;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5_32         : u32 = 0x3FFF0236;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0_64         : u32 = 0x3FFF0238;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1_64         : u32 = 0x3FFF0239;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2_64         : u32 = 0x3FFF023A;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3_64         : u32 = 0x3FFF023B;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4_64         : u32 = 0x3FFF023C;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5_64         : u32 = 0x3FFF023D;
pub static VI_ATTR_PXI_IS_EXPRESS               : u32 = 0x3FFF0240;
pub static VI_ATTR_PXI_SLOT_LWIDTH              : u32 = 0x3FFF0241;
pub static VI_ATTR_PXI_MAX_LWIDTH               : u32 = 0x3FFF0242;
pub static VI_ATTR_PXI_ACTUAL_LWIDTH            : u32 = 0x3FFF0243;
pub static VI_ATTR_PXI_DSTAR_BUS                : u32 = 0x3FFF0244;
pub static VI_ATTR_PXI_DSTAR_SET                : u32 = 0x3FFF0245;
pub static VI_ATTR_PXI_ALLOW_WRITE_COMBINE      : u32 = 0x3FFF0246;
pub static VI_ATTR_TCPIP_HISLIP_OVERLAP_EN      : u32 = 0x3FFF0300;
pub static VI_ATTR_TCPIP_HISLIP_VERSION         : u32 = 0x3FFF0301;
pub static VI_ATTR_TCPIP_HISLIP_MAX_MESSAGE_KB  : u32 = 0x3FFF0302;
pub static VI_ATTR_TCPIP_IS_HISLIP              : u32 = 0x3FFF0303;
 
pub static VI_ATTR_JOB_ID                       : u32 = 0x3FFF4006;
pub static VI_ATTR_EVENT_TYPE                   : u32 = 0x3FFF4010;
pub static VI_ATTR_SIGP_STATUS_ID               : u32 = 0x3FFF4011;
pub static VI_ATTR_RECV_TRIG_ID                 : u32 = 0x3FFF4012;
pub static VI_ATTR_INTR_STATUS_ID               : u32 = 0x3FFF4023;
pub static VI_ATTR_STATUS                       : u32 = 0x3FFF4025;
pub static VI_ATTR_RET_COUNT_32                 : u32 = 0x3FFF4026;
pub static VI_ATTR_BUFFER                       : u32 = 0x3FFF4027;
pub static VI_ATTR_RECV_INTR_LEVEL              : u32 = 0x3FFF4041;
pub static VI_ATTR_OPER_NAME                    : u32 = 0xBFFF4042;
pub static VI_ATTR_GPIB_RECV_CIC_STATE          : u32 = 0x3FFF4193;
pub static VI_ATTR_RECV_TCPIP_ADDR              : u32 = 0xBFFF4198;
pub static VI_ATTR_USB_RECV_INTR_SIZE           : u32 = 0x3FFF41B0;
pub static VI_ATTR_USB_RECV_INTR_DATA           : u32 = 0xBFFF41B1;
pub static VI_ATTR_PXI_RECV_INTR_SEQ            : u32 = 0x3FFF4240;
pub static VI_ATTR_PXI_RECV_INTR_DATA           : u32 = 0x3FFF4241;

/*- Attributes (platform dependent size)------------------------------------*/

// #if defined(_VI_INT64_UINT64_DEFINED) && defined(_VISA_ENV_IS_64_BIT)
pub static VI_ATTR_USER_DATA_64                 : u32 = 0x3FFF000A;
pub static VI_ATTR_RET_COUNT_64                 : u32 = 0x3FFF4028;
/* pub static VI_ATTR_USER_DATA                     = VI_ATTR_USER_DATA_64;
pub static VI_ATTR_RET_COUNT                     = VI_ATTR_RET_COUNT_64;
#else */
pub static VI_ATTR_USER_DATA                    : u32 = VI_ATTR_USER_DATA_32;
pub static VI_ATTR_RET_COUNT                    : u32 = VI_ATTR_RET_COUNT_32;
// #endif

// #if defined(_VI_INT64_UINT64_DEFINED;
pub static VI_ATTR_WIN_BASE_ADDR_64             : u32 = 0x3FFF009B;
pub static VI_ATTR_WIN_SIZE_64                  : u32 = 0x3FFF009C;
pub static VI_ATTR_MEM_BASE_64                  : u32 = 0x3FFF00D0;
pub static VI_ATTR_MEM_SIZE_64                  : u32 = 0x3FFF00D1;
/* #endif
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
#else */
pub static VI_ATTR_WIN_BASE_ADDR                : u32 = VI_ATTR_WIN_BASE_ADDR_32;
pub static VI_ATTR_WIN_SIZE                     : u32 = VI_ATTR_WIN_SIZE_32;
pub static VI_ATTR_MEM_BASE                     : u32 = VI_ATTR_MEM_BASE_32;
pub static VI_ATTR_MEM_SIZE                     : u32 = VI_ATTR_MEM_SIZE_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR0            : u32 = VI_ATTR_PXI_MEM_BASE_BAR0_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR1            : u32 = VI_ATTR_PXI_MEM_BASE_BAR1_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR2            : u32 = VI_ATTR_PXI_MEM_BASE_BAR2_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR3            : u32 = VI_ATTR_PXI_MEM_BASE_BAR3_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR4            : u32 = VI_ATTR_PXI_MEM_BASE_BAR4_32;
pub static VI_ATTR_PXI_MEM_BASE_BAR5            : u32 = VI_ATTR_PXI_MEM_BASE_BAR5_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR0            : u32 = VI_ATTR_PXI_MEM_SIZE_BAR0_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR1            : u32 = VI_ATTR_PXI_MEM_SIZE_BAR1_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR2            : u32 = VI_ATTR_PXI_MEM_SIZE_BAR2_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR3            : u32 = VI_ATTR_PXI_MEM_SIZE_BAR3_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR4            : u32 = VI_ATTR_PXI_MEM_SIZE_BAR4_32;
pub static VI_ATTR_PXI_MEM_SIZE_BAR5            : u32 = VI_ATTR_PXI_MEM_SIZE_BAR5_32;
// #endif 

/*- Event Types -------------------------------------------------------------*/

pub static VI_EVENT_IO_COMPLETION               : u32 = 0x3FFF2009;
pub static VI_EVENT_TRIG                        : u32 = 0xBFFF200A;
pub static VI_EVENT_SERVICE_REQ                 : u32 = 0x3FFF200B;
pub static VI_EVENT_CLEAR                       : u32 = 0x3FFF200D;
pub static VI_EVENT_EXCEPTION                   : u32 = 0xBFFF200E;
pub static VI_EVENT_GPIB_CIC                    : u32 = 0x3FFF2012;
pub static VI_EVENT_GPIB_TALK                   : u32 = 0x3FFF2013;
pub static VI_EVENT_GPIB_LISTEN                 : u32 = 0x3FFF2014;
pub static VI_EVENT_VXI_VME_SYSFAIL             : u32 = 0x3FFF201D;
pub static VI_EVENT_VXI_VME_SYSRESET            : u32 = 0x3FFF201E;
pub static VI_EVENT_VXI_SIGP                    : u32 = 0x3FFF2020;
pub static VI_EVENT_VXI_VME_INTR                : u32 = 0xBFFF2021;
pub static VI_EVENT_PXI_INTR                    : u32 = 0x3FFF2022;
pub static VI_EVENT_TCPIP_CONNECT               : u32 = 0x3FFF2036;
pub static VI_EVENT_USB_INTR                    : u32 = 0x3FFF2037;
                                                
pub static VI_ALL_ENABLED_EVENTS                : u32 = 0x3FFF7FFF;

/*- Completion and Error Codes ----------------------------------------------*/

pub static VI_SUCCESS_EVENT_EN                  : i32 = 0x3FFF0002; /* 3FFF0002,  1073676290 */
pub static VI_SUCCESS_EVENT_DIS                 : i32 = 0x3FFF0003; /* 3FFF0003,  1073676291 */
pub static VI_SUCCESS_QUEUE_EMPTY               : i32 = 0x3FFF0004; /* 3FFF0004,  1073676292 */
pub static VI_SUCCESS_TERM_CHAR                 : i32 = 0x3FFF0005; /* 3FFF0005,  1073676293 */
pub static VI_SUCCESS_MAX_CNT                   : i32 = 0x3FFF0006; /* 3FFF0006,  1073676294 */
pub static VI_SUCCESS_DEV_NPRESENT              : i32 = 0x3FFF007D; /* 3FFF007D,  1073676413 */
pub static VI_SUCCESS_TRIG_MAPPED               : i32 = 0x3FFF007E; /* 3FFF007E,  1073676414 */
pub static VI_SUCCESS_QUEUE_NEMPTY              : i32 = 0x3FFF0080; /* 3FFF0080,  1073676416 */
pub static VI_SUCCESS_NCHAIN                    : i32 = 0x3FFF0098; /* 3FFF0098,  1073676440 */
pub static VI_SUCCESS_NESTED_SHARED             : i32 = 0x3FFF0099; /* 3FFF0099,  1073676441 */
pub static VI_SUCCESS_NESTED_EXCLUSIVE          : i32 = 0x3FFF009A; /* 3FFF009A,  1073676442 */
pub static VI_SUCCESS_SYNC                      : i32 = 0x3FFF009B; /* 3FFF009B,  1073676443 */

pub static VI_WARN_QUEUE_OVERFLOW               : i32 = 0x3FFF000C; /* 3FFF000C,  1073676300 */
pub static VI_WARN_CONFIG_NLOADED               : i32 = 0x3FFF0077; /* 3FFF0077,  1073676407 */
pub static VI_WARN_NULL_OBJECT                  : i32 = 0x3FFF0082; /* 3FFF0082,  1073676418 */
pub static VI_WARN_NSUP_ATTR_STATE              : i32 = 0x3FFF0084; /* 3FFF0084,  1073676420 */
pub static VI_WARN_UNKNOWN_STATUS               : i32 = 0x3FFF0085; /* 3FFF0085,  1073676421 */
pub static VI_WARN_NSUP_BUF                     : i32 = 0x3FFF0088; /* 3FFF0088,  1073676424 */
pub static VI_WARN_EXT_FUNC_NIMPL               : i32 = 0x3FFF00A9; /* 3FFF00A9,  1073676457 */

pub static VI_ERROR_SYSTEM_ERROR      : i32 = _VI_ERROR+0x3FFF0000; /* BFFF0000, -1073807360 */
pub static VI_ERROR_INV_OBJECT        : i32 = _VI_ERROR+0x3FFF000E; /* BFFF000E, -1073807346 */
pub static VI_ERROR_RSRC_LOCKED       : i32 = _VI_ERROR+0x3FFF000F; /* BFFF000F, -1073807345 */
pub static VI_ERROR_INV_EXPR          : i32 = _VI_ERROR+0x3FFF0010; /* BFFF0010, -1073807344 */
pub static VI_ERROR_RSRC_NFOUND       : i32 = _VI_ERROR+0x3FFF0011; /* BFFF0011, -1073807343 */
pub static VI_ERROR_INV_RSRC_NAME     : i32 = _VI_ERROR+0x3FFF0012; /* BFFF0012, -1073807342 */
pub static VI_ERROR_INV_ACC_MODE      : i32 = _VI_ERROR+0x3FFF0013; /* BFFF0013, -1073807341 */
pub static VI_ERROR_TMO               : i32 = _VI_ERROR+0x3FFF0015; /* BFFF0015, -1073807339 */
pub static VI_ERROR_CLOSING_FAILED    : i32 = _VI_ERROR+0x3FFF0016; /* BFFF0016, -1073807338 */
pub static VI_ERROR_INV_DEGREE        : i32 = _VI_ERROR+0x3FFF001B; /* BFFF001B, -1073807333 */
pub static VI_ERROR_INV_JOB_ID        : i32 = _VI_ERROR+0x3FFF001C; /* BFFF001C, -1073807332 */
pub static VI_ERROR_NSUP_ATTR         : i32 = _VI_ERROR+0x3FFF001D; /* BFFF001D, -1073807331 */
pub static VI_ERROR_NSUP_ATTR_STATE   : i32 = _VI_ERROR+0x3FFF001E; /* BFFF001E, -1073807330 */
pub static VI_ERROR_ATTR_READONLY     : i32 = _VI_ERROR+0x3FFF001F; /* BFFF001F, -1073807329 */
pub static VI_ERROR_INV_LOCK_TYPE     : i32 = _VI_ERROR+0x3FFF0020; /* BFFF0020, -1073807328 */
pub static VI_ERROR_INV_ACCESS_KEY    : i32 = _VI_ERROR+0x3FFF0021; /* BFFF0021, -1073807327 */
pub static VI_ERROR_INV_EVENT         : i32 = _VI_ERROR+0x3FFF0026; /* BFFF0026, -1073807322 */
pub static VI_ERROR_INV_MECH          : i32 = _VI_ERROR+0x3FFF0027; /* BFFF0027, -1073807321 */
pub static VI_ERROR_HNDLR_NINSTALLED  : i32 = _VI_ERROR+0x3FFF0028; /* BFFF0028, -1073807320 */
pub static VI_ERROR_INV_HNDLR_REF     : i32 = _VI_ERROR+0x3FFF0029; /* BFFF0029, -1073807319 */
pub static VI_ERROR_INV_CONTEXT       : i32 = _VI_ERROR+0x3FFF002A; /* BFFF002A, -1073807318 */
pub static VI_ERROR_QUEUE_OVERFLOW    : i32 = _VI_ERROR+0x3FFF002D; /* BFFF002D, -1073807315 */
pub static VI_ERROR_NENABLED          : i32 = _VI_ERROR+0x3FFF002F; /* BFFF002F, -1073807313 */
pub static VI_ERROR_ABORT             : i32 = _VI_ERROR+0x3FFF0030; /* BFFF0030, -1073807312 */
pub static VI_ERROR_RAW_WR_PROT_VIOL  : i32 = _VI_ERROR+0x3FFF0034; /* BFFF0034, -1073807308 */
pub static VI_ERROR_RAW_RD_PROT_VIOL  : i32 = _VI_ERROR+0x3FFF0035; /* BFFF0035, -1073807307 */
pub static VI_ERROR_OUTP_PROT_VIOL    : i32 = _VI_ERROR+0x3FFF0036; /* BFFF0036, -1073807306 */
pub static VI_ERROR_INP_PROT_VIOL     : i32 = _VI_ERROR+0x3FFF0037; /* BFFF0037, -1073807305 */
pub static VI_ERROR_BERR              : i32 = _VI_ERROR+0x3FFF0038; /* BFFF0038, -1073807304 */
pub static VI_ERROR_IN_PROGRESS       : i32 = _VI_ERROR+0x3FFF0039; /* BFFF0039, -1073807303 */
pub static VI_ERROR_INV_SETUP         : i32 = _VI_ERROR+0x3FFF003A; /* BFFF003A, -1073807302 */
pub static VI_ERROR_QUEUE_ERROR       : i32 = _VI_ERROR+0x3FFF003B; /* BFFF003B, -1073807301 */
pub static VI_ERROR_ALLOC             : i32 = _VI_ERROR+0x3FFF003C; /* BFFF003C, -1073807300 */
pub static VI_ERROR_INV_MASK          : i32 = _VI_ERROR+0x3FFF003D; /* BFFF003D, -1073807299 */
pub static VI_ERROR_IO                : i32 = _VI_ERROR+0x3FFF003E; /* BFFF003E, -1073807298 */
pub static VI_ERROR_INV_FMT           : i32 = _VI_ERROR+0x3FFF003F; /* BFFF003F, -1073807297 */
pub static VI_ERROR_NSUP_FMT          : i32 = _VI_ERROR+0x3FFF0041; /* BFFF0041, -1073807295 */
pub static VI_ERROR_LINE_IN_USE       : i32 = _VI_ERROR+0x3FFF0042; /* BFFF0042, -1073807294 */
pub static VI_ERROR_NSUP_MODE         : i32 = _VI_ERROR+0x3FFF0046; /* BFFF0046, -1073807290 */
pub static VI_ERROR_SRQ_NOCCURRED     : i32 = _VI_ERROR+0x3FFF004A; /* BFFF004A, -1073807286 */
pub static VI_ERROR_INV_SPACE         : i32 = _VI_ERROR+0x3FFF004E; /* BFFF004E, -1073807282 */
pub static VI_ERROR_INV_OFFSET        : i32 = _VI_ERROR+0x3FFF0051; /* BFFF0051, -1073807279 */
pub static VI_ERROR_INV_WIDTH         : i32 = _VI_ERROR+0x3FFF0052; /* BFFF0052, -1073807278 */
pub static VI_ERROR_NSUP_OFFSET       : i32 = _VI_ERROR+0x3FFF0054; /* BFFF0054, -1073807276 */
pub static VI_ERROR_NSUP_VAR_WIDTH    : i32 = _VI_ERROR+0x3FFF0055; /* BFFF0055, -1073807275 */
pub static VI_ERROR_WINDOW_NMAPPED    : i32 = _VI_ERROR+0x3FFF0057; /* BFFF0057, -1073807273 */
pub static VI_ERROR_RESP_PENDING      : i32 = _VI_ERROR+0x3FFF0059; /* BFFF0059, -1073807271 */
pub static VI_ERROR_NLISTENERS        : i32 = _VI_ERROR+0x3FFF005F; /* BFFF005F, -1073807265 */
pub static VI_ERROR_NCIC              : i32 = _VI_ERROR+0x3FFF0060; /* BFFF0060, -1073807264 */
pub static VI_ERROR_NSYS_CNTLR        : i32 = _VI_ERROR+0x3FFF0061; /* BFFF0061, -1073807263 */
pub static VI_ERROR_NSUP_OPER         : i32 = _VI_ERROR+0x3FFF0067; /* BFFF0067, -1073807257 */
pub static VI_ERROR_INTR_PENDING      : i32 = _VI_ERROR+0x3FFF0068; /* BFFF0068, -1073807256 */
pub static VI_ERROR_ASRL_PARITY       : i32 = _VI_ERROR+0x3FFF006A; /* BFFF006A, -1073807254 */
pub static VI_ERROR_ASRL_FRAMING      : i32 = _VI_ERROR+0x3FFF006B; /* BFFF006B, -1073807253 */
pub static VI_ERROR_ASRL_OVERRUN      : i32 = _VI_ERROR+0x3FFF006C; /* BFFF006C, -1073807252 */
pub static VI_ERROR_TRIG_NMAPPED      : i32 = _VI_ERROR+0x3FFF006E; /* BFFF006E, -1073807250 */
pub static VI_ERROR_NSUP_ALIGN_OFFSET : i32 = _VI_ERROR+0x3FFF0070; /* BFFF0070, -1073807248 */
pub static VI_ERROR_USER_BUF          : i32 = _VI_ERROR+0x3FFF0071; /* BFFF0071, -1073807247 */
pub static VI_ERROR_RSRC_BUSY         : i32 = _VI_ERROR+0x3FFF0072; /* BFFF0072, -1073807246 */
pub static VI_ERROR_NSUP_WIDTH        : i32 = _VI_ERROR+0x3FFF0076; /* BFFF0076, -1073807242 */
pub static VI_ERROR_INV_PARAMETER     : i32 = _VI_ERROR+0x3FFF0078; /* BFFF0078, -1073807240 */
pub static VI_ERROR_INV_PROT          : i32 = _VI_ERROR+0x3FFF0079; /* BFFF0079, -1073807239 */
pub static VI_ERROR_INV_SIZE          : i32 = _VI_ERROR+0x3FFF007B; /* BFFF007B, -1073807237 */
pub static VI_ERROR_WINDOW_MAPPED     : i32 = _VI_ERROR+0x3FFF0080; /* BFFF0080, -1073807232 */
pub static VI_ERROR_NIMPL_OPER        : i32 = _VI_ERROR+0x3FFF0081; /* BFFF0081, -1073807231 */
pub static VI_ERROR_INV_LENGTH        : i32 = _VI_ERROR+0x3FFF0083; /* BFFF0083, -1073807229 */
pub static VI_ERROR_INV_MODE          : i32 = _VI_ERROR+0x3FFF0091; /* BFFF0091, -1073807215 */
pub static VI_ERROR_SESN_NLOCKED      : i32 = _VI_ERROR+0x3FFF009C; /* BFFF009C, -1073807204 */
pub static VI_ERROR_MEM_NSHARED       : i32 = _VI_ERROR+0x3FFF009D; /* BFFF009D, -1073807203 */
pub static VI_ERROR_LIBRARY_NFOUND    : i32 = _VI_ERROR+0x3FFF009E; /* BFFF009E, -1073807202 */
pub static VI_ERROR_NSUP_INTR         : i32 = _VI_ERROR+0x3FFF009F; /* BFFF009F, -1073807201 */
pub static VI_ERROR_INV_LINE          : i32 = _VI_ERROR+0x3FFF00A0; /* BFFF00A0, -1073807200 */
pub static VI_ERROR_FILE_ACCESS       : i32 = _VI_ERROR+0x3FFF00A1; /* BFFF00A1, -1073807199 */
pub static VI_ERROR_FILE_IO           : i32 = _VI_ERROR+0x3FFF00A2; /* BFFF00A2, -1073807198 */
pub static VI_ERROR_NSUP_LINE         : i32 = _VI_ERROR+0x3FFF00A3; /* BFFF00A3, -1073807197 */
pub static VI_ERROR_NSUP_MECH         : i32 = _VI_ERROR+0x3FFF00A4; /* BFFF00A4, -1073807196 */
pub static VI_ERROR_INTF_NUM_NCONFIG  : i32 = _VI_ERROR+0x3FFF00A5; /* BFFF00A5, -1073807195 */
pub static VI_ERROR_CONN_LOST         : i32 = _VI_ERROR+0x3FFF00A6; /* BFFF00A6, -1073807194 */
pub static VI_ERROR_MACHINE_NAVAIL    : i32 = _VI_ERROR+0x3FFF00A7; /* BFFF00A7, -1073807193 */
pub static VI_ERROR_NPERMISSION       : i32 = _VI_ERROR+0x3FFF00A8; /* BFFF00A8, -1073807192 */

/*- Other VISA Definitions --------------------------------------------------*/

// not as fast, but turned macros into functions.
/*
pub static VI_VERSION_MAJOR(ver)       = (((ViVersion)ver) & 0xFFF00000u32) >> 20;
pub static VI_VERSION_MINOR(ver)       = (((ViVersion)ver) & 0x000FFF00u32) >>  8;
pub static VI_VERSION_SUBMINOR(ver)    = (((ViVersion)ver) & 0x000000FFu32);
*/
pub fn VI_VERSION_MAJOR(ver: u32) -> u32 {
	(ver & 0xFFF00000u32) >> 20
}
pub fn VI_VERSION_MINOR(ver: u32) -> u32 {
	(ver & 0x000FFF00u32) >> 8
}
pub fn VI_VERSION_SUBMINOR(ver: u32) -> u32 {
	(ver & 0x000000FFu32)
}
pub static VI_FIND_BUFLEN             : i32 = 256;

pub static VI_INTF_GPIB               : i32 = 1;
pub static VI_INTF_VXI                : i32 = 2;
pub static VI_INTF_GPIB_VXI           : i32 = 3;
pub static VI_INTF_ASRL               : i32 = 4;
pub static VI_INTF_PXI                : i32 = 5;
pub static VI_INTF_TCPIP              : i32 = 6;
pub static VI_INTF_USB                : i32 = 7;

pub static VI_PROT_NORMAL             : i32 = 1;
pub static VI_PROT_FDC                : i32 = 2;
pub static VI_PROT_HS488              : i32 = 3;
pub static VI_PROT_4882_STRS          : i32 = 4;
pub static VI_PROT_USBTMC_VENDOR      : i32 = 5;

pub static VI_FDC_NORMAL              : i32 = 1;
pub static VI_FDC_STREAM              : i32 = 2;

pub static VI_LOCAL_SPACE             : i32 = 0;
pub static VI_A16_SPACE               : i32 = 1;
pub static VI_A24_SPACE               : i32 = 2;
pub static VI_A32_SPACE               : i32 = 3;
pub static VI_A64_SPACE               : i32 = 4;
pub static VI_PXI_ALLOC_SPACE         : i32 = 9;
pub static VI_PXI_CFG_SPACE           : i32 = 10;
pub static VI_PXI_BAR0_SPACE          : i32 = 11;
pub static VI_PXI_BAR1_SPACE          : i32 = 12;
pub static VI_PXI_BAR2_SPACE          : i32 = 13;
pub static VI_PXI_BAR3_SPACE          : i32 = 14;
pub static VI_PXI_BAR4_SPACE          : i32 = 15;
pub static VI_PXI_BAR5_SPACE          : i32 = 16;
pub static VI_OPAQUE_SPACE            : i32 = 0xFFFF;

pub static VI_UNKNOWN_LA              : i32 = -1;
pub static VI_UNKNOWN_SLOT            : i32 = -1;
pub static VI_UNKNOWN_LEVEL           : i32 = -1;
pub static VI_UNKNOWN_CHASSIS         : i32 = -1;

pub static VI_QUEUE                   : i32 = 1;
pub static VI_HNDLR                   : i32 = 2;
pub static VI_SUSPEND_HNDLR           : i32 = 4;
pub static VI_ALL_MECH                : i32 = 0xFFFF;

pub static VI_ANY_HNDLR               : i32 = 0;

pub static VI_TRIG_ALL                : i32 = -2;
pub static VI_TRIG_SW                 : i32 = -1;
pub static VI_TRIG_TTL0               : i32 = 0;
pub static VI_TRIG_TTL1               : i32 = 1;
pub static VI_TRIG_TTL2               : i32 = 2;
pub static VI_TRIG_TTL3               : i32 = 3;
pub static VI_TRIG_TTL4               : i32 = 4;
pub static VI_TRIG_TTL5               : i32 = 5;
pub static VI_TRIG_TTL6               : i32 = 6;
pub static VI_TRIG_TTL7               : i32 = 7;
pub static VI_TRIG_ECL0               : i32 = 8;
pub static VI_TRIG_ECL1               : i32 = 9;
pub static VI_TRIG_ECL2               : i32 = 10;
pub static VI_TRIG_ECL3               : i32 = 11;
pub static VI_TRIG_ECL4               : i32 = 12;
pub static VI_TRIG_ECL5               : i32 = 13;
pub static VI_TRIG_STAR_SLOT1         : i32 = 14;
pub static VI_TRIG_STAR_SLOT2         : i32 = 15;
pub static VI_TRIG_STAR_SLOT3         : i32 = 16;
pub static VI_TRIG_STAR_SLOT4         : i32 = 17;
pub static VI_TRIG_STAR_SLOT5         : i32 = 18;
pub static VI_TRIG_STAR_SLOT6         : i32 = 19;
pub static VI_TRIG_STAR_SLOT7         : i32 = 20;
pub static VI_TRIG_STAR_SLOT8         : i32 = 21;
pub static VI_TRIG_STAR_SLOT9         : i32 = 22;
pub static VI_TRIG_STAR_SLOT10        : i32 = 23;
pub static VI_TRIG_STAR_SLOT11        : i32 = 24;
pub static VI_TRIG_STAR_SLOT12        : i32 = 25;
pub static VI_TRIG_STAR_INSTR         : i32 = 26;
pub static VI_TRIG_PANEL_IN           : i32 = 27;
pub static VI_TRIG_PANEL_OUT          : i32 = 28;
pub static VI_TRIG_STAR_VXI0          : i32 = 29;
pub static VI_TRIG_STAR_VXI1          : i32 = 30;
pub static VI_TRIG_STAR_VXI2          : i32 = 31;
                                      
pub static VI_TRIG_PROT_DEFAULT       : i32 = 0;
pub static VI_TRIG_PROT_ON            : i32 = 1;
pub static VI_TRIG_PROT_OFF           : i32 = 2;
pub static VI_TRIG_PROT_SYNC          : i32 = 5;
pub static VI_TRIG_PROT_RESERVE       : i32 = 6;
pub static VI_TRIG_PROT_UNRESERVE     : i32 = 7;

pub static VI_READ_BUF                : i32 = 1;
pub static VI_WRITE_BUF               : i32 = 2;
pub static VI_READ_BUF_DISCARD        : i32 = 4;
pub static VI_WRITE_BUF_DISCARD       : i32 = 8;
pub static VI_IO_IN_BUF               : i32 = 16;
pub static VI_IO_OUT_BUF              : i32 = 32;
pub static VI_IO_IN_BUF_DISCARD       : i32 = 64;
pub static VI_IO_OUT_BUF_DISCARD      : i32 = 128;

pub static VI_FLUSH_ON_ACCESS         : i32 = 1;
pub static VI_FLUSH_WHEN_FULL         : i32 = 2;
pub static VI_FLUSH_DISABLE           : i32 = 3;

pub static VI_NMAPPED                 : i32 = 1;
pub static VI_USE_OPERS               : i32 = 2;
pub static VI_DEREF_ADDR              : i32 = 3;
pub static VI_DEREF_ADDR_BYTE_SWAP    : i32 = 4;

pub static VI_TMO_IMMEDIATE           : i32 = 0;
pub static VI_TMO_INFINITE            : u32 = 0xFFFFFFFF;

pub static VI_NO_LOCK                 : i32 = 0;
pub static VI_EXCLUSIVE_LOCK          : i32 = 1;
pub static VI_SHARED_LOCK             : i32 = 2;
pub static VI_LOAD_CONFIG             : i32 = 4;

pub static VI_NO_SEC_ADDR             : i32 = 0xFFFF;

pub static VI_ASRL_PAR_NONE           : i32 = 0;
pub static VI_ASRL_PAR_ODD            : i32 = 1;
pub static VI_ASRL_PAR_EVEN           : i32 = 2;
pub static VI_ASRL_PAR_MARK           : i32 = 3;
pub static VI_ASRL_PAR_SPACE          : i32 = 4;

pub static VI_ASRL_STOP_ONE           : i32 = 10;
pub static VI_ASRL_STOP_ONE5          : i32 = 15;
pub static VI_ASRL_STOP_TWO           : i32 = 20;

pub static VI_ASRL_FLOW_NONE          : i32 = 0;
pub static VI_ASRL_FLOW_XON_XOFF      : i32 = 1;
pub static VI_ASRL_FLOW_RTS_CTS       : i32 = 2;
pub static VI_ASRL_FLOW_DTR_DSR       : i32 = 4;

pub static VI_ASRL_END_NONE           : i32 = 0;
pub static VI_ASRL_END_LAST_BIT       : i32 = 1;
pub static VI_ASRL_END_TERMCHAR       : i32 = 2;
pub static VI_ASRL_END_BREAK          : i32 = 3;

pub static VI_STATE_ASSERTED          : i32 = 1;
pub static VI_STATE_UNASSERTED        : i32 = 0;
pub static VI_STATE_UNKNOWN           : i32 = -1;

pub static VI_BIG_ENDIAN              : i32 = 0;
pub static VI_LITTLE_ENDIAN           : i32 = 1;

pub static VI_DATA_PRIV               : i32 = 0;
pub static VI_DATA_NPRIV              : i32 = 1;
pub static VI_PROG_PRIV               : i32 = 2;
pub static VI_PROG_NPRIV              : i32 = 3;
pub static VI_BLCK_PRIV               : i32 = 4;
pub static VI_BLCK_NPRIV              : i32 = 5;
pub static VI_D64_PRIV                : i32 = 6;
pub static VI_D64_NPRIV               : i32 = 7;
pub static VI_D64_2EVME               : i32 = 8;
pub static VI_D64_SST160              : i32 = 9;
pub static VI_D64_SST267              : i32 = 10;
pub static VI_D64_SST320              : i32 = 11;

pub static VI_WIDTH_8                 : i32 = 1;
pub static VI_WIDTH_16                : i32 = 2;
pub static VI_WIDTH_32                : i32 = 4;
pub static VI_WIDTH_64                : i32 = 8;

pub static VI_GPIB_REN_DEASSERT       : i32 = 0;
pub static VI_GPIB_REN_ASSERT         : i32 = 1;
pub static VI_GPIB_REN_DEASSERT_GTL   : i32 = 2;
pub static VI_GPIB_REN_ASSERT_ADDRESS : i32 = 3;
pub static VI_GPIB_REN_ASSERT_LLO     : i32 = 4;
pub static VI_GPIB_REN_ASSERT_ADDRESS_LLO : i32= 5;
pub static VI_GPIB_REN_ADDRESS_GTL    : i32 = 6;

pub static VI_GPIB_ATN_DEASSERT       : i32 = 0;
pub static VI_GPIB_ATN_ASSERT         : i32 = 1;
pub static VI_GPIB_ATN_DEASSERT_HANDSHAKE : i32= 2;
pub static VI_GPIB_ATN_ASSERT_IMMEDIATE : i32= 3;

pub static VI_GPIB_HS488_DISABLED     : i32 = 0;
pub static VI_GPIB_HS488_NIMPL        : i32 = -1;

pub static VI_GPIB_UNADDRESSED        : i32 = 0;
pub static VI_GPIB_TALKER             : i32 = 1;
pub static VI_GPIB_LISTENER           : i32 = 2;
                                      
pub static VI_VXI_CMD16               : i32 = 0x0200;
pub static VI_VXI_CMD16_RESP16        : i32 = 0x0202;
pub static VI_VXI_RESP16              : i32 = 0x0002;
pub static VI_VXI_CMD32               : i32 = 0x0400;
pub static VI_VXI_CMD32_RESP16        : i32 = 0x0402;
pub static VI_VXI_CMD32_RESP32        : i32 = 0x0404;
pub static VI_VXI_RESP32              : i32 = 0x0004;

pub static VI_ASSERT_SIGNAL           : i32 = -1;
pub static VI_ASSERT_USE_ASSIGNED     : i32 = 0;
pub static VI_ASSERT_IRQ1             : i32 = 1;
pub static VI_ASSERT_IRQ2             : i32 = 2;
pub static VI_ASSERT_IRQ3             : i32 = 3;
pub static VI_ASSERT_IRQ4             : i32 = 4;
pub static VI_ASSERT_IRQ5             : i32 = 5;
pub static VI_ASSERT_IRQ6             : i32 = 6;
pub static VI_ASSERT_IRQ7             : i32 = 7;

pub static VI_UTIL_ASSERT_SYSRESET    : i32 = 1;
pub static VI_UTIL_ASSERT_SYSFAIL     : i32 = 2;
pub static VI_UTIL_DEASSERT_SYSFAIL   : i32 = 3;

pub static VI_VXI_CLASS_MEMORY        : i32 = 0;
pub static VI_VXI_CLASS_EXTENDED      : i32 = 1;
pub static VI_VXI_CLASS_MESSAGE       : i32 = 2;
pub static VI_VXI_CLASS_REGISTER      : i32 = 3;
pub static VI_VXI_CLASS_OTHER         : i32 = 4;

pub static VI_PXI_ADDR_NONE           : i32 = 0;
pub static VI_PXI_ADDR_MEM            : i32 = 1;
pub static VI_PXI_ADDR_IO             : i32 = 2;
pub static VI_PXI_ADDR_CFG            : i32 = 3;

pub static VI_TRIG_UNKNOWN            : i32 = -1;

pub static VI_PXI_LBUS_UNKNOWN        : i32 = -1;
pub static VI_PXI_LBUS_NONE           : i32 = 0;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_0: i32 = 1000;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_1: i32 = 1001;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_2: i32 = 1002;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_3: i32 = 1003;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_4: i32 = 1004;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_5: i32 = 1005;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_6: i32 = 1006;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_7: i32 = 1007;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_8: i32 = 1008;
pub static VI_PXI_LBUS_STAR_TRIG_BUS_9: i32 = 1009;
pub static VI_PXI_STAR_TRIG_CONTROLLER: i32 = 1413;

/*- Backward Compatibility Macros -------------------------------------------*/

// pub static viGetDefaultRM(vi)          viOpenDefaultRM(vi);
// the rename should be exported wherever viOpenDefaultRM is ported. TODO.
pub static VI_ERROR_INV_SESSION       : i32 = VI_ERROR_INV_OBJECT;
pub static VI_INFINITE                : u32 = VI_TMO_INFINITE;
pub static VI_NORMAL                  : i32 = VI_PROT_NORMAL;
pub static VI_FDC                     : i32 = VI_PROT_FDC;
pub static VI_HS488                   : i32 = VI_PROT_HS488;
pub static VI_ASRL488                 : i32 = VI_PROT_4882_STRS;
pub static VI_ASRL_IN_BUF             : i32 = VI_IO_IN_BUF;
pub static VI_ASRL_OUT_BUF            : i32 = VI_IO_OUT_BUF;
pub static VI_ASRL_IN_BUF_DISCARD     : i32 = VI_IO_IN_BUF_DISCARD;
pub static VI_ASRL_OUT_BUF_DISCARD    : i32 = VI_IO_OUT_BUF_DISCARD;

/*- The End -----------------------------------------------------------------*/
