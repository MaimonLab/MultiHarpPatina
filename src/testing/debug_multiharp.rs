//! For testing functions without a physical MultiHarp connected
use crate::multiharp::MultiHarpDevice;
use crate::error::{PatinaError, MultiHarpError};
use crate::mhconsts::{self, TriggerEdge, MeasurementControlMode, MeasurementMode};

/// A Debug struct used for testing the logic of
/// functions that use a MultiHarp device. Most
/// methods return `Ok(())` and do nothing.
#[allow(dead_code, unused_variables)]
pub struct DebugMultiHarp150 {
    index : i32,
    serial : String,
    _sync_div : i32,
    _sync_rate : f64,
    _sync_offset : i32,
    _sync_edge : TriggerEdge,
    _sync_level : i32,
    _sync_dead_time : i32,


    _input_edges : Vec<TriggerEdge>,
    _input_enables : Vec<bool>,
    _input_dead_times : Vec<i32>,
    _input_levels : Vec<i32>,
    _input_offsets : Vec<i32>,

    _mean_count_rate : f64,
    _num_channels : i32,

    _binning : i32,
    _histogram_len : i32,
    _offset : i32,
    _measurement_control : MeasurementControlMode,
    _measurement_mode : MeasurementMode,
    _reference_clock : mhconsts::ReferenceClock,

    _base_resolution : f64,

    _ctc_status : bool,

}

impl Default for DebugMultiHarp150 {
    fn default() -> Self {
        DebugMultiHarp150 {
            index: 0,
            serial: "1044272".to_string(),
            _sync_div : 1,
            _sync_rate : 80e7,
            _sync_offset : 0,
            _sync_edge : TriggerEdge::Rising,
            _sync_level : -150,
            _sync_dead_time : 0,

            _input_edges : vec![TriggerEdge::Rising; 4],
            _input_enables : vec![true; 4],
            _input_dead_times : vec![0; 4],
            _input_levels : vec![-150; 4],
            _input_offsets : vec![0; 4],

            _mean_count_rate: 1.0e5,
            _num_channels : 4,

            _binning : 0,
            _histogram_len : 0,
            _offset : 0,
            _measurement_control : MeasurementControlMode::SingleShotCtc,
            _measurement_mode : MeasurementMode::T3,
            _reference_clock : mhconsts::ReferenceClock::Internal,

            _base_resolution : 5.0,
            _ctc_status : false,
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
        Ok(
            DebugMultiHarp150 {
            index,
            serial: "1044272".to_string(),
            _mean_count_rate: 1.0e5,
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
            _mean_count_rate: 1.0e5,
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
        self._ctc_status = true;
        Ok(())
    }

    fn stop_measurement(&mut self) -> Result<(), MultiHarpError> {
        self._ctc_status = false;
        Ok(())
    }

    fn ctc_status(&self) -> Result<bool, MultiHarpError> {
        Ok(self._ctc_status)
    }

    fn get_index(&self) -> i32 {
        self.index
    }

    fn get_serial(&self) -> String {
        self.serial.clone()
    }
}