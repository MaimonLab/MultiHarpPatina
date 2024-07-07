//! For testing functions without a physical MultiHarp connected
use crate::multiharp::MultiHarpDevice;
use crate::error::{PatinaError, MultiHarpError};
use crate::mhconsts;

/// A dummy struct used for testing the logic of
/// functions that use a MultiHarp device.
pub struct DummyMultiHarp150 {
    index : i32,
    serial : String,
    mean_count_rate : f64,
}

impl MultiHarpDevice for DummyMultiHarp150 {
    fn open(index : Option<i32>) -> Result<Self, PatinaError<i32>> {
        if index.is_none() {
            return Err(PatinaError::NoDeviceAvailable);
        }
        let index = index.unwrap();
        if index < 0 || index > mhconsts::MAXDEVNUM {
            return Err(PatinaError::ArgumentError(
                "index".to_string(),
                index,
                "Index must be between 0 and 7".to_string())
            );
        }
        Ok(DummyMultiHarp150 {
            index,
            serial: "Dummy".to_string(),
            mean_count_rate: 1.0e5,
        })
    }

    fn open_by_serial(serial : &str) -> Result<Self, PatinaError<i32>> {
        if serial.len() > 8 {
            return Err(PatinaError::ArgumentError(
                "serial".to_string(),
                serial.len() as i32,
                "Serial number must be 8 characters or less".to_string())
            );
        }
        Ok(DummyMultiHarp150 {
            index: 0,
            serial: "Dummy".to_string(),
            mean_count_rate: 1.0e5,
        })
    }

    fn init(&mut self, mode : mhconsts::MeasurementMode, reference_clock : mhconsts::ReferenceClock) -> Result<(), MultiHarpError> {
        Ok(())
    }

    fn get_hardware_info(&self) -> Result<(String, String, String), MultiHarpError> {
        Ok(("".to_string(), "".to_string(), "".to_string()))
    }

    fn get_base_resolution(&self) -> Result<(f64, i32), MultiHarpError> {
        Ok((0.0, 0))
    }

    fn num_input_channels(&self) -> Result<i32, MultiHarpError> {
        Ok(0)
    }

    fn get_debug_info(&self) -> Result<String, MultiHarpError> {
        Ok("".to_string())
    }

    fn get_index(&self) -> i32 {
        self.index
    }

    fn get_serial(&self) -> String {
        self.serial.clone()
    }
}