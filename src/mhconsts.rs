//! Port the `mhdefin.h` constants to rust

pub static LIB_VERSION : &str = "3.1"; // library version
pub const MAXDEVNUM : i32 = 8; // max number of USB devices
pub const MAXINPCHAN : i32 = 64; // max number of physical input channels
pub const DEBUGSTRLEN : usize = 65536; // length of debug string
pub const BINSTEPSMAX : i32 = 24; // max number of binning steps, get actual number via MH_GetBaseResolution()
pub const MAXHISTLEN : i32 = 65536; // max number of histogram bins
pub const TTREADMAX : i32 = 1048576; // number of event records that can be read by MH_ReadFiFo. Buffer must provide space for this number of dwords

pub const SYNCDIVMIN : i32 = 1; // min value for sync divider
pub const SYNCDIVMAX : i32 = 16; // max value for sync divider

/// millivolts
pub const TRGLVLMIN : i32 = -1200; // mV
/// millivolts
pub const TRGLVLMAX : i32 = 1200; // mV

/// picoseconds
pub const CHANNEL_OFFS_MIN : i32 = -99999; // ps
/// picoseconds
pub const CHANNEL_OFFS_MAX : i32 = 99999; // ps



/// MultiHarp modes
#[derive(Debug, Clone, Copy)]
pub enum MeasurementMode {
    Histogramming = 0,
    T2 = 2,
    T3 = 3,
}

/// Which clock the MultiHarp should be
/// referenced to
#[derive(Debug, Clone, Copy)]
pub enum ReferenceClock {
    /// Multiharp internal oscillator
    Internal = 0,
    /// 10 MHz external clock
    External = 1,
    /// White Rabbit master with generic partner
    WRMaster = 2,
    /// White Rabbit slave with generic partner
    WRSlave = 3,
    /// White Rabbit grandmaster with generic partner
    WRGrandmaster = 4,
    /// 10 MHz + PPS from GPS receiver
    PpsGps = 5,
    /// 10 MHz + PPS + time via UART from GPS receiver
    PpsUart = 6,
    /// White Rabbit master with MultiHarp partner
    WrMasterMH = 7,
    /// White Rabbit slave with MultiHarp partner
    WrSlaveMH = 8,
    /// White Rabbit grandmaster with MultiHarp partner
    WrGrandmasterMH = 9,
}

/// Hardware triggered measurements through TTL vs. 
/// software gating of the initiation of measurement.
pub enum MeasurementControls {
    /// Runs until the `tacq` time passed to `MH_StartMeas` elapses
    SingleShotCtc = 0,
    /// Data collected only when C1 is active (edge determined by `startedge` parameter)
    C1Gated = 1,
    /// Data collected  when C1 transitions, then stops when CTC expires (`tacq`)
    C1StartCtcStop = 2,
    /// Data collected when C1 transitions, then stops when C2 transitions
    C1StartC2Stop = 3,
    WrM2S = 4,
    WrS2M = 5,
    /// New since v3.1, `tacq` is ignored, measurement is
    /// controlled entirely by software, though this makes
    /// `MH_GetElapsedMeasTime` less accurate because it is
    /// constrained by the operating system timers.
    SwStartSwStop = 6,
}

/// Set edge used to identify triggers
pub enum TriggerEdge {
    Rising = 1,
    Falling = 0,
}

/// Allows checking of features available
/// in this device
pub enum FeatureMasks {
    /// Dll license available
    Dll = 0x0001,
    /// TTTR mode available
    Tttr = 0x0002,
    /// Markers available
    Markers = 0x0004,
    /// Long range mode available
    LowRes = 0x0008,
    /// Trigger output available
    TrigOut = 0x0010,
    /// Programmable deadtime available
    ProgTd = 0x0020,
    /// Interface for external FPGA available
    ExtFpga = 0x0040,
    /// Programmable input hysteresis available
    ProgHyst = 0x0080,
    /// Coincidence filtering available
    EvntFilt = 0x0100,
}

/// Masks used to read MH_GetFlags
pub enum Flags {
    /// Histogram mode only
    Overflow = 0x0001,
    /// TTTR mode only
    FifoFull = 0x0002,
    SyncLost = 0x0004,
    RefLost = 0x0008,
    /// Hardware error, must contact support
    SysError = 0x0010,
    /// Measurement is running
    Active = 0x0020,
    /// Counts were dropped
    CountsDropped = 0x0040,
}

// //limits for MH_SetHistoLen
// //note: length codes 0 and 1 will not work with MH_GetHistogram
// //if you need these short lengths then use MH_GetAllHistograms
// #define MINLENCODE  0	
// #define MAXLENCODE  6		//default

// //limits for MH_SetSyncDeadTime and MH_SetInputDeadTime
// #define EXTDEADMIN        800     // ps
// #define EXTDEADMAX     160000     // ps

// //limits for MH_SetOffset
// #define OFFSETMIN           0     // ns
// #define OFFSETMAX   100000000     // ns

// //limits for MH_StartMeas
// #define ACQTMIN             1     // ms
// #define ACQTMAX     360000000     // ms  (100*60*60*1000ms = 100h)

// //limits for MH_SetStopOverflow
// #define STOPCNTMIN          1
// #define STOPCNTMAX 4294967295     // 32 bit is mem max

// //limits for MH_SetTriggerOutput
// #define TRIGOUTMIN          0	  // 0=off
// #define TRIGOUTMAX   16777215     // in units of 100ns

// //limits for MH_SetMarkerHoldoffTime
// #define HOLDOFFMIN          0     // ns
// #define HOLDOFFMAX      25500     // ns

// //limits for MH_SetInputHysteresis
// #define HYSTCODEMIN         0     // approx. 3mV
// #define HYSTCODEMAX         1     // approx. 35mV

// //limits for MH_SetOflCompression
// #define HOLDTIMEMIN         0     // ms
// #define HOLDTIMEMAX       255     // ms

// //limits for MH_SetRowEventFilterXXX and MH_SetMainEventFilter
// #define ROWIDXMIN           0
// #define ROWIDXMAX           8     // actual upper limit is smaller, dep. on rows present
// #define MATCHCNTMIN         1     
// #define MATCHCNTMAX         6 
// #define INVERSEMIN          0
// #define INVERSEMAX          1
// #define TIMERANGEMIN        0     // ps
// #define TIMERANGEMAX   160000     // ps
// #define USECHANSMIN     0x000     // no channels used 
// #define USECHANSMAX     0x1FF     // note: sync bit 0x100 will be ignored in T3 mode and in row filter
// #define PASSCHANSMIN    0x000     // no channels passed 
// #define PASSCHANSMAX    0x1FF     // note: sync bit 0x100 will be ignored in T3 mode and in row filter

// //The following are bitmasks for results from GetWarnings()

// #define WARNING_SYNC_RATE_ZERO				0x0001
// #define WARNING_SYNC_RATE_VERY_LOW			0x0002
// #define WARNING_SYNC_RATE_TOO_HIGH			0x0004
// #define WARNING_INPT_RATE_ZERO				0x0010
// #define WARNING_INPT_RATE_TOO_HIGH			0x0040
// #define WARNING_INPT_RATE_RATIO				0x0100
// #define WARNING_DIVIDER_GREATER_ONE			0x0200
// #define WARNING_TIME_SPAN_TOO_SMALL			0x0400
// #define WARNING_OFFSET_UNNECESSARY			0x0800
// #define WARNING_DIVIDER_TOO_SMALL			0x1000
// #define WARNING_COUNTS_DROPPED				0x2000

// //The following is only for use with White Rabbit

// #define WR_STATUS_LINK_ON               0x00000001  // WR link is switched on
// #define WR_STATUS_LINK_UP               0x00000002  // WR link is established

// #define WR_STATUS_MODE_BITMASK          0x0000000C  // mask for the mode bits
// #define WR_STATUS_MODE_OFF              0x00000000  // mode is "off"
// #define WR_STATUS_MODE_SLAVE            0x00000004  // mode is "slave"
// #define WR_STATUS_MODE_MASTER           0x00000008  // mode is "master" 
// #define WR_STATUS_MODE_GMASTER          0x0000000C  // mode is "grandmaster"

// #define WR_STATUS_LOCKED_CALIBD         0x00000010  // locked and calibrated

// #define WR_STATUS_PTP_BITMASK           0x000000E0  // mask for the PTP bits
// #define WR_STATUS_PTP_LISTENING         0x00000020
// #define WR_STATUS_PTP_UNCLWRSLCK        0x00000040
// #define WR_STATUS_PTP_SLAVE             0x00000060
// #define WR_STATUS_PTP_MSTRWRMLCK        0x00000080
// #define WR_STATUS_PTP_MASTER            0x000000A0

// #define WR_STATUS_SERVO_BITMASK         0x00000700  // mask for the servo bits
// #define WR_STATUS_SERVO_UNINITLZD       0x00000100  //
// #define WR_STATUS_SERVO_SYNC_SEC        0x00000200  //
// #define WR_STATUS_SERVO_SYNC_NSEC       0x00000300  //
// #define WR_STATUS_SERVO_SYNC_PHASE      0x00000400  //
// #define WR_STATUS_SERVO_WAIT_OFFST      0x00000500  //
// #define WR_STATUS_SERVO_TRCK_PHASE      0x00000600  //

// #define WR_STATUS_MAC_SET               0x00000800  // user defined mac address is set
// #define WR_STATUS_IS_NEW                0x80000000  // status updated since last check



// //The following is only for use with an external FPGA connected to a MultiHarp 160

// #define EXTFPGA_MODE_OFF                0
// #define EXTFPGA_MODE_T2RAW              1
// #define EXTFPGA_MODE_T2                 2
// #define EXTFPGA_MODE_T3                 3

// #define EXTFPGA_LOOPBACK_OFF            0
// #define EXTFPGA_LOOPBACK_CUSTOM         1
// #define EXTFPGA_LOOPBACK_T2             2
// #define EXTFPGA_LOOPBACK_T3             3
