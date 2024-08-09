//! Port the `mhdefin.h` constants to rust

#[cfg(feature = "MHLv3_0_0")]
pub static LIB_VERSION : &str = "3.0"; // library version
#[cfg(feature = "MHLv3_1_0")]
pub static LIB_VERSION : &str = "3.1"; // library version
// Otherwise it's >= 1.0
#[cfg(not(any(feature = "MHLv3_0_0", feature = "MHLv3_1_0")))]
pub static LIB_VERSION : &str = "1.0"; // library version

/// Max number of USB devices
pub const MAXDEVNUM : i32 = 8;
/// Max number of physical input channels
pub const MAXINPCHAN : i32 = 64;
/// Debug string length
pub const DEBUGSTRLEN : usize = 65536;
/// Max number of binning steps, can get actual number with `MH_GetBaseResolution`
pub const BINSTEPSMAX : i32 = 24;
/// Max number of histogram bins
pub const MAXHISTLEN : usize = 65536;
/// Number of records in the FIFO buffer
pub const TTREADMAX : usize = 1048576;

/// Min sync divider value
pub const SYNCDIVMIN : i32 = 1;
/// Max sync divider value
pub const SYNCDIVMAX : i32 = 16;

/// special marker for TTTR mode -- overflow and markers
pub const SPECIAL : u32 = 1 << 31;
/// channel mask for TTTR mode
pub const CHANNEL : u32 = (1 << 31) - (1 << 25);
/// arrival time mask for T2 mode
pub const HISTOTAG_T2 : u32 = (1 << 25) - 1;
/// arrival time mask for T3 mode
pub const HISTOTAG_T3 : u32 = (1 << 25) - (1 << 10);
/// sync counter -- 10 lowest bits -- for T3 only
pub const SYNCTAG : u32 = (1 << 10) - 1;

/// millivolts
pub const TRGLVLMIN : i32 = -1200; // mV
/// millivolts
pub const TRGLVLMAX : i32 = 1200; // mV

/// picoseconds
pub const CHANNEL_OFFS_MIN : i32 = -99999; // ps
/// picoseconds
pub const CHANNEL_OFFS_MAX : i32 = 99999; // ps
/// picoseconds
pub const EXTDEADMIN : i32 = 800; // ps
/// picoseconds
pub const EXTDEADMAX : i32 = 160000; // ps
///picoseconds
pub const OFFSETMIN : i32 = 0; // ns
/// picoseconds
pub const OFFSETMAX : i32 = 100000000; // ns
/// milliseconds
pub const ACQTMIN : i32 = 1; // ms
/// milliseconds
pub const ACQTMAX : i32 = 360000000; // ms  (100*60*60*1000ms = 100h)

pub const STOPCNTMIN : u32 = 1;
/// 32 bit is max memory
pub const STOPCNTMAX : u32 = 4294967295; // 32 bit is mem max

/// Off
pub const TRIGOUTMIN : i32 = 0; // 0=off
/// In units of 100 ns
pub const TRIGOUTMAX : i32 = 16777215; // in units of 100ns

/// 0 ns
pub const HOLDOFFMIN : i32 = 0; // ns
/// 25.5 microseconds
pub const HOLDOFFMAX : i32 = 25500; // ns

/// approx 3 mV
pub const HYSTCODEMIN : i32 = 0; // approx. 3mV
/// approx 35 mV
pub const HYSTCODEMAX : i32 = 1; // approx. 35mV

/// 0 ms
pub const HOLDTIMEMIN : i32 = 0; // ms
/// 255 ms
pub const HOLDTIMEMAX : i32 = 255;

pub const MINLENCODE : i32 = 0;
/// default
pub const MAXLENCODE : i32 = 6; // default

//The following are bitmasks for results from GetWarnings()
pub const WARNLEN : usize = 16384; // length of warning string

pub const WARNING_SYNC_RATE_ZERO : i32 = 0x0001;
pub const WARNING_SYNC_RATE_VERY_LOW : i32 = 0x0002;
pub const WARNING_SYNC_RATE_TOO_HIGH : i32 = 0x0004;
pub const WARNING_INPT_RATE_ZERO : i32 = 0x0010;
pub const WARNING_INPT_RATE_TOO_HIGH : i32 = 0x0040;
pub const WARNING_INPT_RATE_RATIO : i32 = 0x0100;
pub const WARNING_DIVIDER_GREATER_ONE : i32 = 0x0200;
pub const WARNING_TIME_SPAN_TOO_SMALL : i32 = 0x0400;
pub const WARNING_OFFSET_UNNECESSARY : i32 = 0x0800;
pub const WARNING_DIVIDER_TOO_SMALL : i32 = 0x1000;
pub const WARNING_COUNTS_DROPPED : i32 = 0x2000;

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
#[derive(Debug, Clone, Copy)]
pub enum MeasurementControlMode {
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
    #[cfg(feature = "MHLv3_1_0")]
    SwStartSwStop = 6,
}

/// Set edge used to identify triggers
#[derive(Debug, Clone, Copy)]
pub enum TriggerEdge {
    Rising = 1,
    Falling = 0,
}

/// Allows checking of features available
/// in this device
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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

pub const ROWIDXMIN : i32 = 0;
pub const ROWIDXMAX : i32 = 8;

pub const MATCHCNTMIN : i32 = 1;
pub const MATCHCNTMAX : i32 = 6;

pub const INVERSEMIN : i32 = 0;
pub const INVERSEMAX : i32 = 1;

/// picoseconds
pub const TIMERANGEMIN : i32 = 0;
/// picoseconds
pub const TIMERANGEMAX : i32 = 160000;

pub const USECHANSMIN : i32 = 0x000;
pub const USECHANSMAX : i32 = 0x1FF;

pub const PASSCHANSMIN : i32 = 0x000;
pub const PASSCHANSMAX : i32 = 0x1FF;

/// White Rabbit link is switched on
pub const WR_STATUS_LINK_ON : i32 = 0x00000001;
/// WR link is established
pub const WR_STATUS_LINK_UP : i32 = 0x00000002;

/// White Rabbit mode bit mask
pub const WR_STATUS_MODE_BITMASK : i32 = 0x0000000C;
pub const WR_STATUS_MODE_OFF : i32 = 0x00000000;
pub const WR_STATUS_MODE_SLAVE : i32 = 0x00000004;
pub const WR_STATUS_MODE_MASTER : i32 = 0x00000008;
pub const WR_STATUS_MODE_GMASTER : i32 = 0x0000000C;

/// Locked and calibrated
pub const WR_STATUS_LOCKED_CALIBD : i32 = 0x00000010;

/// White Rabbit PTP bit mask
pub const WR_STATUS_PTP_BITMASK : i32 = 0x000000E0;
pub const WR_STATUS_PTP_LISTENING : i32 = 0x00000020;
pub const WR_STATUS_PTP_UNCLWRSLCK : i32 = 0x00000040;
pub const WR_STATUS_PTP_SLAVE : i32 = 0x00000060;
pub const WR_STATUS_PTP_MSTRWRMLCK : i32 = 0x00000080;
pub const WR_STATUS_PTP_MASTER : i32 = 0x000000A0;

/// White Rabbit servo bit mask
pub const WR_STATUS_SERVO_BITMASK : i32 = 0x00000700;
pub const WR_STATUS_SERVO_UNINITLZD : i32 = 0x00000100;
pub const WR_STATUS_SERVO_SYNC_SEC : i32 = 0x00000200;
pub const WR_STATUS_SERVO_SYNC_NSEC : i32 = 0x00000300;
pub const WR_STATUS_SERVO_SYNC_PHASE : i32 = 0x00000400;
pub const WR_STATUS_SERVO_WAIT_OFFST : i32 = 0x00000500;
pub const WR_STATUS_SERVO_TRCK_PHASE : i32 = 0x00000600;

pub const WR_MAC_LEN : usize = 6;
pub const WR_SCRIPT_LEN : usize = 256;
pub const WR_TERM_LEN : usize = 513;

pub enum WRMode {
    Off = 0,
    Slave = 1,
    Master = 2,
    Grandmaster = 3,
}

/// User defined MAC address is set
pub const WR_STATUS_MAC_SET : i32 = 0x00000800;

/// Status updated since last check
pub const WR_STATUS_IS_NEW : u32 = 0x80000000;

/// Only usable with an external FPGA
/// connected to a MultiHarp 160
pub enum ExtFpgaMode {
    Off = 0,
    T2Raw = 1,
    T2 = 2,
    T3 = 3,
}

pub enum ExtFpgaLoopback {
    Off = 0,
    Custom = 1,
    T2 = 2,
    T3 = 3,
}