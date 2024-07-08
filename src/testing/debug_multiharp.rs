//! For testing functions without a physical MultiHarp connected
use crate::multiharp::MultiHarpDevice;
use crate::error::{PatinaError, MultiHarpError};
use crate::mhconsts;

/// A Debug struct used for testing the logic of
/// functions that use a MultiHarp device. Most
/// methods return `Ok(())` and do nothing.
#[allow(dead_code, unused_variables)]
pub struct DebugMultiHarp150 {
    index : i32,
    serial : String,
    sync_rate : f64,
    sync_offset : i32,

    input_offsets : Vec<i32>,

    mean_count_rate : f64,
    num_channels : i32,

    binning : i32,
    histogram_len : i32,

    ctc_status : bool,

}

impl Default for DebugMultiHarp150 {
    fn default() -> Self {
        DebugMultiHarp150 {
            index: 0,
            serial: "1044272".to_string(),
            sync_rate : 80e7,
            sync_offset : 0,
            mean_count_rate: 1.0e5,
            num_channels : 4,
            input_offsets : vec![0; 4],
            binning : 0,
            histogram_len : 65536,
            ctc_status : false,
        }
    }
}

#[allow(dead_code, unused_variables)]
impl MultiHarpDevice for DebugMultiHarp150 {
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
        Ok(DebugMultiHarp150 {
            index,
            serial: "1044272".to_string(),
            mean_count_rate: 1.0e5,
            ..Default::default()
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
        Ok(DebugMultiHarp150 {
            index: 0,
            serial: "1044272".to_string(),
            mean_count_rate: 1.0e5,
            ..Default::default()
        })
    }

    fn init(
        &mut self,
        mode : mhconsts::MeasurementMode,
        reference_clock : mhconsts::ReferenceClock
    ) -> Result<(), MultiHarpError> {
        Ok(())
    }

    /// TODO make this start filling and dumping the histogram!
    fn start_measurement(&mut self, acquisition_time : i32) -> Result<(), PatinaError<i32>> {
        self.ctc_status = true;
        Ok(())
    }

    fn stop_measurement(&mut self) -> Result<(), MultiHarpError> {
        self.ctc_status = false;
        Ok(())
    }

    fn ctc_status(&self) -> Result<bool, MultiHarpError> {
        Ok(self.ctc_status)
    }

    fn get_index(&self) -> i32 {
        self.index
    }

    fn get_serial(&self) -> String {
        self.serial.clone()
    }
}