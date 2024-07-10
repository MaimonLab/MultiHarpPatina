//! Direct translation of the error codes from the MultiHarp DLL.
use std::error::Error;
use std::fmt::{Display, Debug};
use crate::error_to_string;

/// Macro to convert a result from a MultiHarp function to a Result
/// with the error code converted to a `MultiHarpError`
/// 
/// (`$result:expr`, `$val:expr`) -> `Result<$val, MultiHarpError>`
macro_rules! mh_to_result {
    ($result:expr, $val : expr) => {
        if $result == 0 {
            Ok($val)
        } else {
            Err(MultiHarpError::from($result))
        }
    };
}

pub (crate) use mh_to_result;

pub type CheckedResult<R, T> = Result<R, PatinaError<T>>;
pub type MultiHarpResult<R> = Result<R, MultiHarpError>;

#[derive(Debug, Clone, PartialEq)]
pub enum PatinaError<T> where T : Display + Debug {
    MultiHarpError(MultiHarpError),
    ArgumentError(String, T, String),
    NoDeviceAvailable,
    FeatureNotAvailable(String),
    NotImplemented,
}

impl<T> Display for PatinaError<T> where T: Display + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PatinaError::MultiHarpError(e) => {
                write!(f, "MultiHarpError: {}", error_to_string(*e as i32).unwrap())
            },
            PatinaError::ArgumentError(argname, arg, additional_text) => {
                write!(f, "Invalid argument {}: {}. Additional information: {}", argname, arg, additional_text)
            },
            PatinaError::FeatureNotAvailable(feature) => {
                write!(f, "Feature not available: {}", feature)
            },
            PatinaError::NoDeviceAvailable => write!(f, "No MultiHarp devices available"),
            PatinaError::NotImplemented => write!(f, "Functionality not implemented in Rust yet"),
        }
    }
}

impl <T> From <MultiHarpError> for PatinaError<T> where T: Display + Debug {
    fn from(e: MultiHarpError) -> Self {
        PatinaError::MultiHarpError(e)
    }
}

impl<T> std::error::Error for PatinaError<T> where T: Display + Debug {}


/// MultiHarp error codes from C
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum MultiHarpError {
    None = 0,
    DeviceOpenFail = -1,
    DeviceBusy = -2,
    DeviceHEventFail = -3,
    DeviceCallBSetFail = -4,
    DeviceBarMapFail = -5,
    DeviceCloseFail = -6,
    DeviceResetFail = -7,
    DeviceGetVersionFail = -8,
    DeviceVersionMismatch = -9,
    DeviceNotOpen = -10,
    DeviceLocked = -11,
    DeviceDriverVersionMismatch = -12,

    InstanceRunning = -16,
    InvalidArgument = -17,
    InvalidMode = -18,
    InvalidOption = -19,
    InvalidMemory = -20,
    InvalidRData = -21,
    NotInitialized = -22,
    NotCalibrated = -23,
    DMAFail = -24,
    XTDeviceFail = -25,
    FPGAConfFail = -26,
    IFConfFail = -27,
    FIFOResetFail = -28,
    ThreadStateFail = -29,
    ThreadLockFail = -30,

    USBGetDriverVersionFail = -32,
    USBDriverVersionMismatch = -33,
    USBGetIFInfoFail = -34,
    USBHiSpeedFail = -35,
    USBVCMDFail = -36,
    USBBulkReadFail = -37,
    USBResetFail = -38,

    LaneupTimeout = -40,
    DoneAllTimeout = -41,
    MBAckTimeoint = -42,
    MActiveTimeout = -43,
    MemClearFail = -44,
    MemTestFail = -45,
    CalibFail = -46,
    RefSelFail = -47,
    StatusFail = -48,
    ModNumberFail = -49,
    DigMuxFail = -50,
    ModMuxFail = -51,
    ModFirmwarePCBMismatch = -52,
    ModFirmwareVersionMismatch = -53,
    ModPropertyMismatch = -54,
    InvalidMagic = -55,
    InvalidLength = -56,
    RateFail = -57,
    ModFirmwareVersionTooOld = -58,
    ModFirmwareVersionTooNew = -59,
    MBAckFail = -60,

    EEPROMF01 = -64,
    EEPROMF02 = -65,
    EEPROMF03 = -66,
    EEPROMF04 = -67,
    EEPROMF05 = -68,
    EEPROMF06 = -69,
    EEPROMF07 = -70,
    EEPROMF08 = -71,
    EEPROMF09 = -72,
    EEPROMF10 = -73,
    EEPROMF11 = -74,
    EEPROMF12 = -75,
    EEPROMF13 = -76,
    EEPROMF14 = -77,
    EEPROMF15 = -78,

    InvalidError = -1000,
}

impl From<i32> for MultiHarpError {
    fn from(error: i32) -> Self {
        match error {
            0 => MultiHarpError::None,
            -1 => MultiHarpError::DeviceOpenFail,
            -2 => MultiHarpError::DeviceBusy,
            -3 => MultiHarpError::DeviceHEventFail,
            -4 => MultiHarpError::DeviceCallBSetFail,
            -5 => MultiHarpError::DeviceBarMapFail,
            -6 => MultiHarpError::DeviceCloseFail,
            -7 => MultiHarpError::DeviceResetFail,
            -8 => MultiHarpError::DeviceGetVersionFail,
            -9 => MultiHarpError::DeviceVersionMismatch,
            -10 => MultiHarpError::DeviceNotOpen,
            -11 => MultiHarpError::DeviceLocked,
            -12 => MultiHarpError::DeviceDriverVersionMismatch,

            -16 => MultiHarpError::InstanceRunning,
            -17 => MultiHarpError::InvalidArgument,
            -18 => MultiHarpError::InvalidMode,
            -19 => MultiHarpError::InvalidOption,
            -20 => MultiHarpError::InvalidMemory,
            -21 => MultiHarpError::InvalidRData,
            -22 => MultiHarpError::NotInitialized,
            -23 => MultiHarpError::NotCalibrated,
            -24 => MultiHarpError::DMAFail,
            -25 => MultiHarpError::XTDeviceFail,
            -26 => MultiHarpError::FPGAConfFail,
            -27 => MultiHarpError::IFConfFail,
            -28 => MultiHarpError::FIFOResetFail,
            -29 => MultiHarpError::ThreadStateFail,
            -30 => MultiHarpError::ThreadLockFail,

            -32 => MultiHarpError::USBGetDriverVersionFail,
            -33 => MultiHarpError::USBDriverVersionMismatch,
            -34 => MultiHarpError::USBGetIFInfoFail,
            -35 => MultiHarpError::USBHiSpeedFail,
            -36 => MultiHarpError::USBVCMDFail,
            -37 => MultiHarpError::USBBulkReadFail,
            -38 => MultiHarpError::USBResetFail,

            -40 => MultiHarpError::LaneupTimeout,
            -41 => MultiHarpError::DoneAllTimeout,
            -42 => MultiHarpError::MBAckTimeoint,
            -43 => MultiHarpError::MActiveTimeout,
            -44 => MultiHarpError::MemClearFail,
            -45 => MultiHarpError::MemTestFail,
            -46 => MultiHarpError::CalibFail,
            -47 => MultiHarpError::RefSelFail,
            -48 => MultiHarpError::StatusFail,
            -49 => MultiHarpError::ModNumberFail,
            -50 => MultiHarpError::DigMuxFail,
            -51 => MultiHarpError::ModMuxFail,
            -52 => MultiHarpError::ModFirmwarePCBMismatch,
            -53 => MultiHarpError::ModFirmwareVersionMismatch,
            -54 => MultiHarpError::ModPropertyMismatch,
            -55 => MultiHarpError::InvalidMagic,
            -56 => MultiHarpError::InvalidLength,
            -57 => MultiHarpError::RateFail,
            -58 => MultiHarpError::ModFirmwareVersionTooOld,
            -59 => MultiHarpError::ModFirmwareVersionTooNew,
            -60 => MultiHarpError::MBAckFail,

            -64 => MultiHarpError::EEPROMF01,
            -65 => MultiHarpError::EEPROMF02,
            -66 => MultiHarpError::EEPROMF03,
            -67 => MultiHarpError::EEPROMF04,
            -68 => MultiHarpError::EEPROMF05,
            -69 => MultiHarpError::EEPROMF06,
            -70 => MultiHarpError::EEPROMF07,
            -71 => MultiHarpError::EEPROMF08,
            -72 => MultiHarpError::EEPROMF09,
            -73 => MultiHarpError::EEPROMF10,
            -74 => MultiHarpError::EEPROMF11,
            -75 => MultiHarpError::EEPROMF12,
            -76 => MultiHarpError::EEPROMF13,
            -77 => MultiHarpError::EEPROMF14,
            -78 => MultiHarpError::EEPROMF15,
            _ => MultiHarpError::InvalidError,
        }    
    }
}

impl std::fmt::Display for MultiHarpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MultiHarpError::None => write!(f, "No error"),
            MultiHarpError::DeviceOpenFail => write!(f, "Device could not be opened"),
            MultiHarpError::DeviceBusy => write!(f, "Device busy -- may be used by another instance"),
            MultiHarpError::DeviceHEventFail => write!(f, "Device HEvent fail TODO what's this"),
            MultiHarpError::DeviceCallBSetFail => write!(f, "Device callback set fail"),
            MultiHarpError::DeviceBarMapFail => write!(f, "Device bar map fail"),
            MultiHarpError::DeviceCloseFail => write!(f, "Device could not be closed, may be in use"),
            MultiHarpError::DeviceResetFail => write!(f, "Device reset fail"),
            MultiHarpError::DeviceGetVersionFail => write!(f, "Could not retrieve version of device"),
            MultiHarpError::DeviceVersionMismatch => write!(f, "Device version mismatch"),
            MultiHarpError::DeviceNotOpen => write!(f, "Device not open -- try opening it first"),
            MultiHarpError::DeviceLocked => write!(f, "Device locked"),
            MultiHarpError::DeviceDriverVersionMismatch => write!(f, "Device driver version mismatch"),

            MultiHarpError::InstanceRunning => write!(f, "Instance running"),
            MultiHarpError::InvalidArgument => write!(f, "Invalid argument"),
            MultiHarpError::InvalidMode => write!(f, "Invalid mode"),
            MultiHarpError::InvalidOption => write!(f, "Invalid option"),
            MultiHarpError::InvalidMemory => write!(f, "Invalid memory"),
            MultiHarpError::InvalidRData => write!(f, "Invalid RData"),
            MultiHarpError::NotInitialized => write!(f, "Not initialized"),
            MultiHarpError::NotCalibrated => write!(f, "Not calibrated"),
            MultiHarpError::DMAFail => write!(f, "DMA fail"),
            MultiHarpError::XTDeviceFail => write!(f, "XT device fail"),
            MultiHarpError::FPGAConfFail => write!(f, "FPGA conf fail"),
            MultiHarpError::IFConfFail => write!(f, "IF conf fail"),
            MultiHarpError::FIFOResetFail => write!(f, "Failed to FIFO buffer"),
            MultiHarpError::ThreadStateFail => write!(f, "Thread state fail"),
            MultiHarpError::ThreadLockFail => write!(f, "Thread lock fail"),

            MultiHarpError::USBGetDriverVersionFail => write!(f, "Failed to get USB driver version"),
            MultiHarpError::USBDriverVersionMismatch => write!(f, "USB driver version mismatch"),
            MultiHarpError::USBGetIFInfoFail => write!(f, "Failed to get USB IF info"),
            MultiHarpError::USBHiSpeedFail => write!(f, "USB high speed fail"),
            MultiHarpError::USBVCMDFail => write!(f, "USB VCMD fail"),
            MultiHarpError::USBBulkReadFail => write!(f, "USB bulk read fail"),
            MultiHarpError::USBResetFail => write!(f, "USB reset fail"),

            MultiHarpError::LaneupTimeout => write!(f, "Laneup timeout"),
            MultiHarpError::DoneAllTimeout => write!(f, "Done all timeout"),
            MultiHarpError::MBAckTimeoint => write!(f, "MB ack timeout"),
            MultiHarpError::MActiveTimeout => write!(f, "M active timeout"),
            MultiHarpError::MemClearFail => write!(f, "Memory clear fail"),
            MultiHarpError::MemTestFail => write!(f, "Memory test fail"),
            MultiHarpError::CalibFail => write!(f, "Calibration fail"),
            MultiHarpError::RefSelFail => write!(f, "Reference select fail"),
            MultiHarpError::StatusFail => write!(f, "Status fail"),
            MultiHarpError::ModNumberFail => write!(f, "Module number fail"),
            MultiHarpError::DigMuxFail => write!(f, "Digital multiplexer fail"),
            MultiHarpError::ModMuxFail => write!(f, "Module multiplexer fail"),
            MultiHarpError::ModFirmwarePCBMismatch => write!(f, "Module firmware PCB mismatch"),
            MultiHarpError::ModFirmwareVersionMismatch => write!(f, "Module firmware version mismatch"),
            MultiHarpError::ModPropertyMismatch => write!(f, "Module property mismatch"),
            MultiHarpError::InvalidMagic => write!(f, "Invalid magic"),
            MultiHarpError::InvalidLength => write!(f, "Invalid length"),
            MultiHarpError::RateFail => write!(f, "Rate fail"),
            MultiHarpError::ModFirmwareVersionTooOld => write!(f, "Module firmware version too old"),
            MultiHarpError::ModFirmwareVersionTooNew => write!(f, "Module firmware version too new"),
            MultiHarpError::MBAckFail => write!(f, "MB ack fail"),

            MultiHarpError::EEPROMF01 => write!(f, "EEPROM F01"),
            MultiHarpError::EEPROMF02 => write!(f, "EEPROM F02"),
            MultiHarpError::EEPROMF03 => write!(f, "EEPROM F03"),
            MultiHarpError::EEPROMF04 => write!(f, "EEPROM F04"),
            MultiHarpError::EEPROMF05 => write!(f, "EEPROM F05"),
            MultiHarpError::EEPROMF06 => write!(f, "EEPROM F06"),
            MultiHarpError::EEPROMF07 => write!(f, "EEPROM F07"),
            MultiHarpError::EEPROMF08 => write!(f, "EEPROM F08"),
            MultiHarpError::EEPROMF09 => write!(f, "EEPROM F09"),
            MultiHarpError::EEPROMF10 => write!(f, "EEPROM F10"),
            MultiHarpError::EEPROMF11 => write!(f, "EEPROM F11"),
            MultiHarpError::EEPROMF12 => write!(f, "EEPROM F12"),
            MultiHarpError::EEPROMF13 => write!(f, "EEPROM F13"),
            MultiHarpError::EEPROMF14 => write!(f, "EEPROM F14"),
            MultiHarpError::EEPROMF15 => write!(f, "EEPROM F15"),

            MultiHarpError::InvalidError => write!(f, "Invalid error returned from MHLib \
            -- problem with `Multi-Harp-Patina` library"),
        }
    }
}

impl Error for MultiHarpError {}