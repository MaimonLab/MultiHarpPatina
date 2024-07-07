//! Code for interfacing with a MultiHarp 150

use std::ffi::*;
use crate::error::{MultiHarpError, PatinaError, mh_to_result};
use crate::mhconsts;
use crate::mhlib::*;
use crate::{available_devices, MHDeviceIterator};

/// A trait for MultiHarp devices -- must implement
/// all of the below methods.
pub trait MultiHarpDevice : Sized {
    fn open(index : Option<i32>) -> Result<Self, PatinaError<i32>>;
    fn open_by_serial(serial : &str) -> Result<Self, PatinaError<i32>>;
    fn init(&mut self, mode : mhconsts::MeasurementMode, reference_clock : mhconsts::ReferenceClock) -> Result<(), MultiHarpError>;
    fn get_hardware_info(&self) -> Result<(String, String, String), MultiHarpError>;
    fn get_base_resolution(&self) -> Result<(f64, i32), MultiHarpError>;
    fn num_input_channels(&self) -> Result<i32, MultiHarpError>;
    fn get_debug_info(&self) -> Result<String, MultiHarpError>;
    fn get_index(&self) -> i32;
    fn get_serial(&self) -> String;
}

/// A more object-oriented way to
/// interface with the MultiHarp.
/// 
/// Each method calls the corresponding `MHLib` function
/// with the device index of that `MultiHarp` instance.
/// 
/// Successful creation of a `MultiHarp` instance guarantees
/// that the device has been opened, and the device is
/// closed when the instance is dropped.
pub struct MultiHarp150 {
    index : i32,
    serial : String,
    initialized : bool
}

impl MultiHarpDevice for MultiHarp150 {
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
    fn open(index : Option<i32>) -> Result<Self, PatinaError<i32>> {
        if index.is_none() {
            let dev_vec = available_devices();
            if dev_vec.len() == 0 {
                return Err(PatinaError::NoDeviceAvailable);
            }
            return MultiHarp150::open(Some(dev_vec[0].0));
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
            MultiHarp150 {
                index,
                serial: unsafe { CString::from_raw(serial.as_mut_ptr()) }.to_str().unwrap().to_string(),
                initialized: false
            }
        ).map_err(|e| PatinaError::from(e))
    }

    /// Iterate over MultiHarp device indices until the provided serial number
    /// is found, then open that device.
    fn open_by_serial(serial : &str) -> Result<Self, PatinaError<i32>> {
        if serial.len() > 8 {
            return Err(PatinaError::ArgumentError(
                "serial".to_string(),
                serial.len() as i32,
                "Serial number must be 8 characters or less".to_string())
            );
        }

        MHDeviceIterator::new().skip_while(|(_, s)| s != serial)
        .next()
        .map(|(index, _)| MultiHarp150::open(Some(index)))
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
    fn init(&mut self, mode : mhconsts::MeasurementMode, reference_clock : mhconsts::ReferenceClock) -> Result<(), MultiHarpError> {
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
    fn get_hardware_info(&self) -> Result<(String, String, String), MultiHarpError> {
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
    fn get_base_resolution(&self) -> Result<(f64, i32), MultiHarpError> {
        let mut base_resolution: f64 = 0.0;
        let mut bin_steps = 0;
        mh_to_result!(
            unsafe { MH_GetBaseResolution(self.index, &mut base_resolution, &mut bin_steps) },
            (base_resolution, bin_steps)
        )
    }

    /// Returns the number of input channels in the device.
    fn num_input_channels(&self) -> Result<i32, MultiHarpError> {
        let mut num_channels = 0;
        mh_to_result!(
            unsafe { MH_GetNumOfInputChannels(self.index, &mut num_channels) },
            num_channels
        )
    }

    /// Returns an informative error message by querying the MultiHarp.
    /// Should be called on a `MultiHarpError` to get more information.
    fn get_debug_info(&self) -> Result<String, MultiHarpError> {
        let debug_string = [0 as c_char; mhconsts::DEBUGSTRLEN];
        let mh_result = unsafe { MH_GetErrorString(debug_string.as_ptr() as *mut c_char, self.index) };
        mh_to_result!(
            mh_result,
            unsafe { CString::from_raw(debug_string.as_ptr() as *mut c_char) }.to_str().unwrap().to_string()
        )
    }

    /// Return a copy of the MultiHarp device index.
    fn get_index(&self) -> i32 {
        self.index
    }

    /// Return a copy of the serial number of the MultiHarp
    fn get_serial(&self) -> String {
        self.serial.clone()
    }
}

impl Drop for MultiHarp150 {
    fn drop(&mut self) {
        let mh_return = unsafe { MH_CloseDevice(self.index) };
        if mh_return != 0 {
            eprintln!("Error closing device {}: {}", self.index, error_to_string(mh_return as i32).unwrap());
        }
    }
}