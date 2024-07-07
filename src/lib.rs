//! A thin wrapper of the PicoQuant `MultiHarp 150` control library
//! with Rust. Hence, a patina. Exposes a few functions to generate
//! `MultiHarp` structs, and then the `MultiHarp` struct itself takes
//! care of most reading, device management, etc.
mod error;
mod mhlib;
mod mhconsts;

pub use crate::mhconsts::*;
use crate::mhlib::*;
use crate::error::{MultiHarpError, PatinaError, mh_to_result};
use std::ffi::*;
use std::ops::Mul;


/// A more object-oriented way to
/// interface with the MultiHarp.
/// 
/// Each method calls the corresponding `MHLib` function
/// with the device index of that `MultiHarp` instance.
/// 
/// Successful creation of a `MultiHarp` instance guarantees
/// that the device has been opened, and the device is
/// closed when the instance is dropped.
pub struct MultiHarp {
    index : i32,
    serial : String,
    initialized : bool
}

impl MultiHarp {
    /// Open a MultiHarp device by index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the device to open (0..7).
    /// If no index is provided, will open the first `MultiHarp`
    /// encountered.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the opened MultiHarp device
    /// or an error.
    /// 
    /// # Errors
    pub fn open(index : Option<i32>) -> Result<Self, PatinaError<i32>> {
        if index.is_none() {
            let dev_vec = available_devices();
            if dev_vec.len() == 0 {
                return Err(PatinaError::NoDeviceAvailable);
            }
            return MultiHarp::open(Some(dev_vec[0].0));
        }
        let index = index.unwrap();
        if index < 0 || index > mhconsts::MAXDEVNUM {
            return Err(PatinaError::ArgumentError(
                "index".to_string(),
                index,
                "Index must be between 0 and 7".to_string())
            );
        }
        let mut serial = [0 as c_char; 8];
        let mh_result = unsafe { MH_OpenDevice(index, serial.as_mut_ptr()) };
        mh_to_result!(
            mh_result,
            MultiHarp {
                index,
                serial: unsafe { CString::from_raw(serial.as_mut_ptr()) }.to_str().unwrap().to_string(),
                initialized: false
            }
        ).map_err(|e| PatinaError::from(e))
    }

    /// Iterate over MultiHarp device indices until the provided serial number
    /// is found, then open that device.
    pub fn open_by_serial(serial : &str) -> Result<Self, PatinaError<i32>> {
        if serial.len() > 8 {
            return Err(PatinaError::ArgumentError(
                "serial".to_string(),
                serial.len() as i32,
                "Serial number must be 8 characters or less".to_string())
            );
        }

        MHDeviceIterator::new().skip_while(|(_, s)| s != serial)
        .next()
        .map(|(index, _)| MultiHarp::open(Some(index)))
        .unwrap_or(Err(PatinaError::NoDeviceAvailable))
    }

    /// Initialize an opened MultiHarp in the mode requested.
    /// 
    /// # Arguments
    /// 
    /// * `mode` - The measurement mode to initialize the device in.
    /// 
    /// * `reference_clock` - The reference clock to use for the device.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing `()` if successful, or an error.
    pub fn init(&mut self, mode : MeasurementMode, reference_clock : ReferenceClock) -> Result<(), MultiHarpError> {
        let mh_result = unsafe { MH_Initialize(self.index, mode as c_int, reference_clock as c_int) };
        mh_to_result!(
            mh_result,
            {
                self.initialized = true;
                ()
            }
        )
    }

    /// Returns the model code of the MultiHarp device, its part number, and its version.
    /// 
    /// # Returns
    /// 
    /// * `(Model, PartNumber, Version)`
    pub fn get_hardware_info(&self) -> Result<(String, String, String), MultiHarpError> {
        let mut model_code = [0 as c_char; 24];
        let mut part_number = [0 as c_char; 8];
        let mut version = [0 as c_char; 8];

        mh_to_result!(
            unsafe { MH_GetHardwareInfo(self.index, model_code.as_mut_ptr(), part_number.as_mut_ptr(), version.as_mut_ptr()) },
            (
                unsafe { CString::from_raw(model_code.as_mut_ptr()) }.to_str().unwrap().to_string(),
                unsafe { CString::from_raw(part_number.as_mut_ptr()) }.to_str().unwrap().to_string(),
                unsafe { CString::from_raw(version.as_mut_ptr()) }.to_str().unwrap().to_string()
            )
        )
    }

    /// Returns the base resolution in picoseconds -- the finest possible bins --
    /// as well as the total number of allowed bins.
    /// 
    /// # Returns
    /// 
    /// * `(base_resolution, bin_steps)` - The base resolution in picoseconds and the maximum
    /// number of bin steps. In T3 and histogramming mode, the maximum number of bins
    /// you can use is `binsteps-1`
    /// 
    pub fn get_base_resolution(&self) -> Result<(f64, i32), MultiHarpError> {
        let mut base_resolution: f64 = 0.0;
        let mut bin_steps = 0;
        mh_to_result!(
            unsafe { MH_GetBaseResolution(self.index, &mut base_resolution, &mut bin_steps) },
            (base_resolution, bin_steps)
        )
    }

    /// Returns the number of input channels in the device.
    pub fn num_input_channels(&self) -> Result<i32, MultiHarpError> {
        let mut num_channels = 0;
        mh_to_result!(
            unsafe { MH_GetNumOfInputChannels(self.index, &mut num_channels) },
            num_channels
        )
    }

    /// Returns an informative error message by querying the MultiHarp.
    /// Should be called on a `MultiHarpError` to get more information.
    pub fn get_debug_info(&self) -> Result<String, MultiHarpError> {
        let debug_string = [0 as c_char; DEBUGSTRLEN];
        let mh_result = unsafe { MH_GetErrorString(debug_string.as_ptr() as *mut c_char, self.index) };
        mh_to_result!(
            mh_result,
            unsafe { CString::from_raw(debug_string.as_ptr() as *mut c_char) }.to_str().unwrap().to_string()
        )
    }

    /// Return a copy of the MultiHarp device index.
    pub fn get_index(&self) -> i32 {
        self.index
    }

    /// Return a copy of the serial number of the MultiHarp
    pub fn get_serial(&self) -> String {
        self.serial.clone()
    }
}

impl Drop for MultiHarp {
    fn drop(&mut self) {
        let mh_return = unsafe { MH_CloseDevice(self.index) };
        if mh_return != 0 {
            eprintln!("Error closing device {}: {}", self.index, error_to_string(mh_return as i32).unwrap());
        }
    }
}

/// Iterates over available MultiHarps,
/// returning the index and serial number of each.
struct MHDeviceIterator {devidx : i32}

impl MHDeviceIterator {
    /// Initialize at device index 0.
    fn new() -> Self {
        MHDeviceIterator {devidx: 0}
    }
}

impl Iterator for MHDeviceIterator {
    type Item = (i32, String);

    /// Scans until it finds an available device or
    /// exhausts the possible indices.
    fn next(&mut self) -> Option<Self::Item> {
        if self.devidx < 8 {
            let mut serial = [0 as c_char; 8];
            let mh_result = unsafe{ MH_OpenDevice(self.devidx, serial.as_mut_ptr()) };
            if mh_result != 0 {
                // Keep going until you either run out
                // of devices or find one that opens.
                self.devidx += 1;

                return self.next();
            }

            // Close it, we were just checking if it's available.
            unsafe { MH_CloseDevice(self.devidx) };

            let serial_str = unsafe{ CString::from_raw(serial.as_mut_ptr()) }.to_str().unwrap().to_string();
            let result = Some((self.devidx, serial_str));
            self.devidx += 1;
            result
        } else {
            None
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
pub fn available_devices() -> Vec<(i32, String)> {
    MHDeviceIterator::new().collect::<Vec<_>>()
}

/// Returns the version of the MHLib as a String of length 8
pub fn get_library_version() -> Result<String, MultiHarpError> {
    let mut version = [0 as c_char; 8];
    let mh_result = unsafe { MH_GetLibraryVersion(version.as_mut_ptr()) };
    mh_to_result!(
        mh_result,
        unsafe{CString::from_raw(version.as_mut_ptr())}.to_str().unwrap().to_string()
    )
}

/// Should almost certainly never be used, but if something goes
/// wrong with the `MultiHarp` struct and the device remains
/// open, this can be used to try to close it again.
pub fn _close_by_index(index : i32) -> Result<(), MultiHarpError> {
    mh_to_result!(
        unsafe { MH_CloseDevice(index) },
        ()
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_open_device() {
        let mh = super::MultiHarp::open(None);
        assert!(mh.is_ok());
    }

}