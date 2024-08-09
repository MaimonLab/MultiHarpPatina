//! For testing functions without a physical MultiHarp connected
use crate::multiharp::MultiHarpDevice;
use crate::error::{PatinaError, MultiHarpError, MultiHarpResult, CheckedResult};
use crate::mhconsts::{self, TriggerEdge, MeasurementControlMode, MeasurementMode};
use std::thread;

//#[cfg(not(feature = "MHLib"))]
static mut OCCUPIED_DEBUG_DEVICES : Vec<i32> = Vec::<i32>::new();

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
    _resolution : f64,

    _base_resolution : f64,

    _ctc_status : bool,

    _internal_buffer : Vec<u32>,
    /// Generation method should be `Send` so that the
    /// `MultiHarp` can be passed around between threads.
    _generation_method : Box<dyn Fn(&mut Vec<u32>) -> u16 + Send>,
    _n_photons_in_hist : u16,
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
            _resolution : 5.0,
            _ctc_status : false,
            _internal_buffer : Vec::<u32>::with_capacity(mhconsts::TTREADMAX),
            _generation_method : Box::new(Self::_default_tick),
            _n_photons_in_hist : 0
        }
    }
}

impl DebugMultiHarp150 {
    pub fn set_sync_rate(&mut self, rate : f64) {
        self._sync_rate = rate;
    }

    pub fn set_mean_count_rate(&mut self, rate : f64) {
        self._mean_count_rate = rate;
    }

    pub fn new(mean_count_rate : f64, sync_rate : f64) -> Self {
        DebugMultiHarp150 {
            _mean_count_rate: mean_count_rate,
            _sync_rate: sync_rate,
            index: 0,
            serial: "1044272".to_string(),
            _sync_div : 1,
            // _sync_rate : 80e7,
            _sync_offset : 0,
            _sync_edge : TriggerEdge::Rising,
            _sync_level : -150,
            _sync_dead_time : 0,

            _input_edges : vec![TriggerEdge::Rising; 4],
            _input_enables : vec![true; 4],
            _input_dead_times : vec![0; 4],
            _input_levels : vec![-150; 4],
            _input_offsets : vec![0; 4],

            // _mean_count_rate: 1.0e5,
            _num_channels : 4,

            _binning : 0,
            _histogram_len : 0,
            _offset : 0,
            _measurement_control : MeasurementControlMode::SingleShotCtc,
            _measurement_mode : MeasurementMode::T3,
            _reference_clock : mhconsts::ReferenceClock::Internal,

            _base_resolution : 5.0,
            _resolution : 5.0,
            _ctc_status : false,
            _internal_buffer : Vec::<u32>::with_capacity(mhconsts::TTREADMAX),
            _generation_method : Box::new(Self::_default_tick),
            _n_photons_in_hist : 0
        }
    }

    /// Create a new histogrma_tick_method
    pub fn set_histogram_tick_method(&mut self, f : Box<dyn Fn(&mut Vec<u32>) -> u16 + Send>)
    -> () {
        self._generation_method = f;
    }

    /// Populates randomly
    fn _default_tick(hist : &mut Vec<u32>) -> u16 {

        let n_photons = rand::random::<u16>();
        for _ in 0..n_photons {
            let arrival_time = rand::random::<u16>() % (1<<14);
            let channel = rand::random::<u8>() % 4;
            let syncs = rand::random::<u16>() % (1<<10);
            hist.push(
                ((channel as u32) << 26)
                | ((arrival_time as u32) << 10)
                | (syncs as u32)
            );
        }

        n_photons 
    }

    /// Internal method to emulate the population of the internal
    /// buffer
    fn _populate_histogram_tick(&mut self) -> () {
        self._n_photons_in_hist += 
            (self._generation_method)(&mut self._internal_buffer);
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
        if unsafe { OCCUPIED_DEBUG_DEVICES.contains(&index) } {
            return Err(PatinaError::ArgumentError(
                "index".to_string(),
                index,
                "Device already occupied".to_string())
            );
        }
        else {
            unsafe { OCCUPIED_DEBUG_DEVICES.push(index); }
        }
        Ok(
            DebugMultiHarp150 {
            index,
            serial: "1044272".to_string(),
            _mean_count_rate: 1.0e5,
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

            _num_channels : 4,

            _binning : 0,
            _histogram_len : 0,
            _offset : 0,
            _measurement_control : MeasurementControlMode::SingleShotCtc,
            _measurement_mode : MeasurementMode::T3,
            _reference_clock : mhconsts::ReferenceClock::Internal,

            _base_resolution : 5.0,
            _resolution : 5.0,
            _ctc_status : false,
            _internal_buffer : Vec::<u32>::with_capacity(mhconsts::TTREADMAX),
            _generation_method : Box::new(Self::_default_tick),
            _n_photons_in_hist : 0
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

            _num_channels : 4,

            _binning : 0,
            _histogram_len : 0,
            _offset : 0,
            _measurement_control : MeasurementControlMode::SingleShotCtc,
            _measurement_mode : MeasurementMode::T3,
            _reference_clock : mhconsts::ReferenceClock::Internal,

            _base_resolution : 5.0,
            _resolution : 5.0,
            _ctc_status : false,
            _internal_buffer : Vec::<u32>::with_capacity(mhconsts::TTREADMAX),
            _generation_method : Box::new(Self::_default_tick),
            _n_photons_in_hist : 0
        })
    }

    fn init(
        &mut self,
        mode : mhconsts::MeasurementMode,
        reference_clock : mhconsts::ReferenceClock
    ) -> Result<(), MultiHarpError> {
        Ok(())
    }

    fn get_base_resolution(&self) -> crate::error::MultiHarpResult<(f64, i32)> {
        Ok((self._base_resolution, 2500))
    }

    fn set_sync_div(&mut self, sync_div : i32) -> CheckedResult<(), i32> {
        self._sync_div = sync_div;
        Ok(())
    }

    fn set_sync_edge_trigger(&mut self, level : i32, edge : TriggerEdge) -> CheckedResult<(), i32> {
        self._sync_edge = edge;
        self._sync_level = level;
        Ok(())
    }

    fn set_sync_channel_offset(&mut self, offset : i32) -> CheckedResult<(), i32> {
        self._sync_offset = offset;
        Ok(())
    }

    fn set_sync_dead_time(&mut self, on : bool, dead_time : i32) -> CheckedResult<(), i32> {
        self._sync_dead_time = dead_time;
        Ok(())
    }

    fn set_input_edge_trigger(&mut self, channel : i32, level : i32, edge : TriggerEdge) -> CheckedResult<(), i32> {
        self._input_edges[channel as usize] = edge;
        self._input_levels[channel as usize] = level;
        Ok(())
    }

    fn set_input_channel_offset(&mut self, channel : i32, offset : i32) -> CheckedResult<(), i32> {
        self._input_offsets[channel as usize] = offset;
        Ok(())
    }

    fn set_input_dead_time(&mut self, channel : i32, on : bool, dead_time : i32) -> CheckedResult<(), i32> {
        self._input_dead_times[channel as usize] = dead_time;
        Ok(())
    }

    fn set_input_channel_enable(&mut self, channel : i32, enable : bool) -> CheckedResult<(), i32> {
        self._input_enables[channel as usize] = enable;
        Ok(())
    }

    fn set_binning(&mut self, binning : i32) -> CheckedResult<(), i32> {
        self._binning = binning;
        Ok(())
    }

    fn set_offset(&mut self, offset : i32) -> CheckedResult<(), i32> {
        self._offset = offset;
        Ok(())
    }

    /// TODO not right just a dummy!!
    fn set_histogram_len(&mut self, len_code : i32) -> CheckedResult<i32, i32> {
        self._histogram_len = len_code;
        Ok(5)
    }

    fn set_measurement_control_mode(&mut self, control : MeasurementControlMode, start_edge : Option<TriggerEdge>, stop_edge : Option<TriggerEdge>) -> CheckedResult<(), String> {
        self._measurement_control = control;
        Ok(())
    }

    fn set_trigger_output(&mut self, period : i32) -> CheckedResult<(), i32> {
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

    fn get_histogram_by_copy(&mut self, channel : i32) -> CheckedResult<Vec<u32>, i32> {
        Ok(vec![0])
    }

    fn get_all_histograms_by_copy(&mut self) -> MultiHarpResult<Vec<u32>>{
        Ok(vec![0])
    }

    fn fill_histogram(&mut self, histogram : &mut Vec<u32>, channel : i32) -> CheckedResult<(), i32> {
        Ok(())
    }

    fn fill_all_histograms(&mut self, histograms : &mut Vec<u32>) -> MultiHarpResult<()> {
        self._internal_buffer.clear();
        self._n_photons_in_hist = 0;
        Ok(())
    }

    fn get_resolution(&self) -> MultiHarpResult<f64> {
        Ok(self._base_resolution)
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

impl Drop for DebugMultiHarp150 {
    fn drop(&mut self) {
        unsafe { OCCUPIED_DEBUG_DEVICES.retain(|&x| x != self.index); }
    }
}