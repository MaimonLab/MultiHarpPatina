//! A thin wrapper of the PicoQuant `MultiHarp 150` control library
//! with Rust. Hence, a patina. Exposes a few functions to generate
//! `MultiHarp` structs, and then the `MultiHarp` struct itself takes
//! care of most reading, device management, etc.
//! 
//! Provides a `MultiHarp150` struct for interaction with
//! the MultiHarp 150 device, as well as a `DebugMultiHarp150`
//! for offline testing of functionality.
//! 
//! # Crate features

//! ### MultiHarp library features

//! *  - nolib
//! When enabled, this will ignore all the `mhlib` library features
//! and only allow access to the `DebugMultiHarp` structs. This allows
//! for testing without the `MHLib` library (e.g. MacOS).

//! * - MHLv3_0_0
//! Enables features that are only available in version 3.0.0 of the
//! `MHLib` library. This includes the `MH_SetInputHysteresis` function.

//! * - MHLv3_1_0
//! Enables features that are only available in version 3.1.0 of the
//! `MHLib` library. This includes the `MH_SetSyncChannelEnable` function
//! and the various Gating methods

#[cfg(all(feature = "nolib", feature = "MHLib"))]
compile_error!("features `nolib` and `MHLib` are mutually \
exclusive. If you want to use the `nolib` feature, you must disable \
default features `--no-default-features`.");

mod error;
mod mhlib;
mod mhconsts;
mod multiharp;
mod testing;

pub use crate::mhconsts::*;
pub use crate::multiharp::MultiHarpDevice;
#[cfg(feature = "MHLib")]
pub use crate::multiharp::MultiHarp150;
pub use crate::testing::debug_multiharp::DebugMultiHarp150;
pub use crate::error::{PatinaError, MultiHarpError};
use crate::mhlib::*;
use crate::error::mh_to_result;
use std::ffi::*;

/// Iterates over available MultiHarps,
/// returning the index and serial number of each.
struct MHDeviceIterator {devidx : i32}

impl MHDeviceIterator {
    /// Initializes at device index 0, will iterate up
    /// to index 7 (including 7).
    fn new() -> Self {
        MHDeviceIterator {devidx: 0}
    }

    /// Iterates and returns status for all possible device numbers
    /// 
    /// # Returns
    /// 
    /// * Vec<(i32, String, String)> - A `Vec` of tuples containing the index, serial number,
    /// and status of all possible MultiHarp devices as `(device_index, serial_number, status)`.
    /// If the device is open, status is "Open". If the device is busy, status is "Busy".
    /// If the device is locked, status is "Locked". If there is no device at that index,
    /// status is "No device".
    #[allow(dead_code)]
    fn list_devices_and_status() -> Vec<(i32, String, String)> {
        (0..mhconsts::MAXDEVNUM)
            .map(|i| {
                let mut serial = [0 as c_char; 8];
                #[cfg(feature = "MHLib")]
                let mh_result = unsafe{ MH_OpenDevice(i, serial.as_mut_ptr()) };
                #[cfg(feature = "nolib")]
                let mh_result = 0;
                match mh_result {
                    0 => {
                        Some((i, unsafe{ CStr::from_ptr(serial.as_mut_ptr()) }.to_str().unwrap().to_string(), "Available".to_string())) 
                    },
                    -1 => {
                        Some((i, unsafe{ CStr::from_ptr(serial.as_mut_ptr()) }.to_str().unwrap().to_string(), "No device".to_string()))
                    },
                    -2 => {
                        Some((i, unsafe{ CStr::from_ptr(serial.as_mut_ptr()) }.to_str().unwrap().to_string(), "Busy".to_string()))
                    },
                    -11 => {
                        Some((i, unsafe{ CStr::from_ptr(serial.as_mut_ptr()) }.to_str().unwrap().to_string(), "Locked".to_string()))
                    },
                    _ => {
                        Some((i, "".to_string(), "No device".to_string()))
                    }
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec::<(i32, String, String)>>()
    }
}

impl Iterator for MHDeviceIterator {
    type Item = (i32, String);

    /// Scans until it finds an available device or
    /// exhausts the possible indices.
    fn next(&mut self) -> Option<Self::Item> {
        if self.devidx < 8 {
            let mut serial = [0 as c_char; 8];
            #[cfg(feature = "MHLib")]
            let mh_result = unsafe{ MH_OpenDevice(self.devidx, serial.as_mut_ptr()) };
            #[cfg(feature = "nolib")]
            let mh_result = 0;
            if mh_result != 0 {
                // Keep going until you either run out
                // of devices or find one that opens.
                self.devidx += 1;

                return self.next();
            }

            // Close it, we were just checking if it's available.
            #[cfg(feature = "MHLib")]
            unsafe { MH_CloseDevice(self.devidx) };
            
            #[cfg(feature = "MHLib")]
            let serial_str = unsafe{ CStr::from_ptr(serial.as_mut_ptr()) }.to_str().unwrap().to_string();
            #[cfg(feature = "nolib")]
            let serial_str = "Debug00".to_string();
            
            let result = Some((self.devidx, serial_str));
            self.devidx += 1;
            return result
        } else {
            None
        }
    }
}

/// A single configuration structure
/// to set many parameters in one function call
/// 
/// Any parameters set to `None` will not be set
pub struct MultiHarpConfig {
    pub sync_div : Option<i32>,
    pub sync_trigger_edge : Option<(i32, TriggerEdge)>,
    pub sync_channel_offset : Option<i32>,
    #[cfg(feature = "MHLv3_1_0")]
    pub sync_channel_enable : Option<bool>,
    pub sync_dead_time: Option<(bool, i32)>,

    /// Vector of (channel, offset, edge)
    pub input_edges : Option<Vec<(i32, i32, TriggerEdge)>>,
    /// Vector of (channel, offset)
    pub input_offsets : Option<Vec<(i32,i32)>>,
    /// Vector of (channel, enable)
    pub input_enables : Option<Vec<(i32,bool)>>,
    pub input_dead_times : Option<Vec<(i32, bool, i32)>>,
    #[cfg(feature = "MHLv3_0_0")]
    pub input_hysteresis : Option<bool>,

    pub stop_overflow : Option<(bool, u32)>,

    pub binning : Option<i32>,
    pub offset : Option<i32>,
    pub histo_len : Option<i32>,

    pub meas_control : Option<(MeasurementControlMode, Option<TriggerEdge>, Option<TriggerEdge>)>,
    pub trigger_output : Option<i32>,

    #[cfg(feature = "MHLv3_1_0")]
    pub ofl_compression : Option<i32>,

    pub marker_edges : Option<[TriggerEdge;4]>,
    pub marker_enable : Option<[bool;4]>,
    pub marker_holdoff : Option<i32>,
}

impl Default for MultiHarpConfig {
    fn default() -> Self {
        MultiHarpConfig {
            sync_div : None,
            sync_trigger_edge : None,
            sync_channel_offset : None,
            #[cfg(feature = "MHLv3_1_0")]
            sync_channel_enable : None,
            sync_dead_time: None,

            input_edges : None,
            input_offsets : None,
            input_enables : None,
            input_dead_times : None,
            #[cfg(feature = "MHLv3_0_0")]
            input_hysteresis : None,

            stop_overflow : None,

            binning : None,
            offset : None,
            histo_len : None,

            meas_control : None,
            trigger_output : None,

            #[cfg(feature = "MHLv3_1_0")]
            ofl_compression : None,

            marker_edges : None,
            marker_enable : None,
            marker_holdoff : None,
        }
    }
}

/// Scans all possible device numbers and returns a list of
/// available MultiHarp devices by index and serial number.
/// 
/// # Returns
/// 
/// * Vec<(i32, String)> - A `Vec` of tuples containing the index and serial number
/// of available MultiHarp devices as `(device_index, serial_number)`.
/// 
/// # Example
/// 
/// ```
/// use multi_harp_patina::*;
/// 
/// let devs = available_devices();
/// println!("Available devices : {:?}", devs);
/// ```
pub fn available_devices() -> Vec<(i32, String)> {
    MHDeviceIterator::new().collect::<Vec<_>>()
}

/// Opens first available MultiHarp device.
/// 
/// ## Errors
/// 
/// * `PatinaError::NoDeviceAvailable` - If no devices are available.
/// * `MultiHarpError` - If there is an error opening the device.
/// 
/// # Returns
/// 
/// * `Result<MH, PatinaError>` - A `Result` containing the `MultiHarp` struct
/// if any are found and available, otherwise an error.
/// 
/// # Example
/// 
/// ```
/// use multi_harp_patina::*;
/// 
/// // For an actual use case
/// #[cfg(feature = "MHLib")]
/// let mh = open_first_device::<MultiHarp150>();
/// 
/// // For a debug test case
/// #[cfg(feature = "nolib")]
/// let mh = open_first_device::<DebugMultiHarp150>();
/// 
/// ```
pub fn open_first_device<MH : MultiHarpDevice>() -> Result<MH, PatinaError>{
    let dev_vec = available_devices();
    if dev_vec.len() == 0 {
        return Err(PatinaError::NoDeviceAvailable);
    }

    MH::open(Some(dev_vec[0].0))
}

/// Returns the version of the MHLib as a String of length 8
/// 
/// ## Example
/// 
/// ```
/// use multi_harp_patina::*;
/// 
/// let version = get_library_version();
/// println!["Library version: {}", version.unwrap()];
/// ```
pub fn get_library_version() -> Result<String, MultiHarpError> {
    let mut version = [0 as c_char; 8];
    #[cfg(feature = "MHLib")]
    let mh_result = unsafe { MH_GetLibraryVersion(version.as_mut_ptr()) };
    #[cfg(feature = "nolib")]
    let mh_result = 0;

    mh_to_result!(
        mh_result,
        unsafe{
            CStr::from_ptr(version.as_mut_ptr())
        }.to_str().unwrap().to_string()
    )
}

/// Should almost certainly never be used, but if something goes
/// wrong with the `MultiHarp` struct and the device remains
/// open, this can be used to try to close it again.
pub fn _close_by_index(index : i32) -> Result<(), MultiHarpError> {
    #[cfg(feature = "MHLib")]{
    mh_to_result!(
        unsafe { MH_CloseDevice(index) },
        ()
    )}
    #[cfg(feature="nolib")]{
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    /// Flexible definition for debugging
    /// without a real multiharp connected.
    #[cfg(feature = "MHLib")]
    type TestMH = MultiHarp150;
    #[cfg(feature = "nolib")]
    type TestMH = DebugMultiHarp150;

    #[test]
    fn test_available_devices() {
        let devs = available_devices();
        println!("Available devices : {:?}", devs);

        let all_devs = MHDeviceIterator::list_devices_and_status();
        println!("All devices: {:?}", all_devs);
    }

    #[test]
    fn test_open_device() {
        let mh = open_first_device::<TestMH>();
        assert!(mh.is_ok());
        let mh = mh.unwrap();
        println!("Opened device with serial number {}", mh.get_serial()); 
    }

    #[test]
    /// This one only works on my demo machine... bad test!
    fn test_open_by_serial() {
        let mh = TestMH::open_by_serial("01044272");
        assert!(mh.is_ok());
        let mh = mh.unwrap();
        println!("Opened device with serial number {}", mh.get_serial());
        let mh = open_first_device::<TestMH>();
    }
}