//! For testing functions without a physical MultiHarp connected
use crate::multiharp::MultiHarpDevice;

#[cfg(feature = "async")]
use crate::multiharp::AsyncMultiHarpDevice;
use crate::TTREADMAX;

use std::sync::{Arc, RwLock};
use crate::error::{PatinaError, MultiHarpError, MultiHarpResult, CheckedResult};
use crate::mhconsts::{self, TriggerEdge, MeasurementControlMode, MeasurementMode};

use rand_distr::{Distribution, Poisson, Exp};

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
    /// Units of nanoseconds
    _taus : Vec<f64>,
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

    // This is not technically correct! The _interal_buffer
    // ends up getting owned by threads that can outlive
    // the `DebugMultiHarp150` in principle. In practice
    // those threads are joined before the `DebugMultiHarp150`
    // is dropped, but it is potentially dangerous.
    _internal_buffer : Arc<RwLock<(Vec<u32>, usize)>>,
    _last_tick : std::time::SystemTime,
    _acq_thread : Option<std::thread::JoinHandle<()>>,
    _start_time : std::time::SystemTime,
    _acquisition_time : i32,
    _acquiring : Arc<std::sync::atomic::AtomicBool>,
    
    /// Generation method should be `Send` so that the
    /// `MultiHarp` can be passed around between threads.
    _generation_method : Box<dyn Fn(std::time::Duration, &mut Vec<u32>) -> u16 + Send>,
    
    // This method seems smarter, and doesn't rely on dynamic types,
    // but I made a mistake implementing it so I'll have to revisit
    // the question later.
    // _generation_method : F,
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
            _taus : vec![2.0; 1],
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
            _last_tick : std::time::SystemTime::now(),
            // Big buffer with lots of space.
            _internal_buffer : Arc::new(RwLock::new(
                (Vec::<u32>::with_capacity(500*mhconsts::TTREADMAX), 0)
            )),
            // _generation_method : F
            _generation_method : Box::new(Self::_default_tick),
            _acq_thread : None,
            _start_time : std::time::SystemTime::now(),
            _acquisition_time : 0,
            _acquiring : Arc::new(std::sync::atomic::AtomicBool::new(false)),
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

    pub fn get_mean_count_rate(&self) -> f64 {
        self._mean_count_rate
    }

    /// Create a new DebugMultiHarp150 with a mean count rate and sync rate
    /// defined in seconds and the exponential(s) from which the photons are
    /// drawn.
    /// 
    /// # Arguments
    /// 
    /// * `mean_count_rate` - The mean photon count rate in Hz
    /// 
    /// * `sync_rate` - The sync rate in Hz
    /// 
    /// * `taus` - The exponential decay times in nanoseconds. If
    /// `None` then the default is `[2.0]`
    pub fn new(mean_count_rate : f64, sync_rate : f64, taus : Option<Vec<f64>>) -> Self {
        let taus = taus.unwrap_or(vec![2.0]);
        DebugMultiHarp150 {
            _mean_count_rate: mean_count_rate,
            _sync_rate: sync_rate,
            _taus : taus,
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
            _last_tick : std::time::SystemTime::now(),
            _internal_buffer : Arc::new(RwLock::new(
                (Vec::<u32>::with_capacity(500*mhconsts::TTREADMAX), 0)
            )),
            _generation_method : Box::new(Self::_default_tick),
            _acq_thread : None,
            _start_time : std::time::SystemTime::now(),
            _acquisition_time : 0,
            _acquiring : Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Set the exponential(s) from which the photon arrival times
    /// are drawn. Units are in nanoseconds.
    pub fn set_taus(&mut self, taus : Vec<f64>) -> () {
        self._taus = taus;
    }


    /// Create a new histogrma_tick_method
    pub fn set_histogram_tick_method(&mut self, f : Box<dyn Fn(std::time::Duration, &mut Vec<u32>) -> u16 + Send>)
    -> () {
        self._generation_method = f;
    }

    /// Populates randomly -- returns number of photons added
    fn _default_tick(tick_interval : std::time::Duration, hist : &mut Vec<u32>) -> u16 {
        0
        // let n_photons = rand::random::<u16>();
        // for _ in 0..n_photons {
        //     let arrival_time = rand::random::<u16>() % (1<<14);
        //     let channel = rand::random::<u8>() % 4;
        //     let syncs = rand::random::<u16>() % (1<<10);
        //     hist.push(
        //         ((channel as u32) << 26)
        //         | ((arrival_time as u32) << 10)
        //         | (syncs as u32)
        //     );
        // }

        // n_photons 
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
            _taus : vec![2.0],
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

            _last_tick : std::time::SystemTime::now(),
            _base_resolution : 5.0,
            _resolution : 5.0,
            _ctc_status : false,
            _internal_buffer : Arc::new(RwLock::new(
                (Vec::<u32>::with_capacity(500*mhconsts::TTREADMAX),0)
            )),
            _generation_method : Box::new(Self::_default_tick),
            _acq_thread : None,
            _start_time : std::time::SystemTime::now(),
            _acquisition_time : 0,
            _acquiring : Arc::new(std::sync::atomic::AtomicBool::new(false)),
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
            _taus : vec![2.0],
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

            _last_tick : std::time::SystemTime::now(),
            _base_resolution : 5.0,
            _resolution : 5.0,
            _ctc_status : false,
            _internal_buffer : Arc::new(RwLock::new(
                (Vec::<u32>::with_capacity(500*mhconsts::TTREADMAX), 0)
            )),
            _generation_method : Box::new(Self::_default_tick),
            _acq_thread : None,
            _start_time : std::time::SystemTime::now(),
            _acquisition_time : 0,
            _acquiring : Arc::new(std::sync::atomic::AtomicBool::new(false)),
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

    fn start_measurement(&mut self, acquisition_time : i32) -> Result<(), PatinaError<i32>> {
        self._ctc_status = true;
        self._last_tick = std::time::SystemTime::now();
        self._acquisition_time = acquisition_time;
        self._acquiring.store(true, std::sync::atomic::Ordering::SeqCst);

        let acq_pt = Arc::clone(&self._acquiring);

        // Reset the internal buffer pointer
        let mut internal = self._internal_buffer.as_ref().write().unwrap();
        internal.0.clear();
        internal.1 = 0;
        
        // Create cloned variables for the thread
        let buf = Arc::clone(&self._internal_buffer);
        let mean_rate = self._mean_count_rate.clone();
        let exponentials = self._taus.iter().map(|tau| Exp::new(1.0/tau).unwrap())
        .to_owned();

        let mut last_tick = std::time::Instant::now();

        // Define the acquisition function here -- TODO use
        // the _generation_method attribute, though it's tricky because
        // it needs to be cloned -- along with its necessary arguments -- somehow.
        self._acq_thread = Some(std::thread::spawn(move || {

            let start_time = std::time::SystemTime::now();
            let mut rng = rand::thread_rng();

            while acq_pt.load(std::sync::atomic::Ordering::SeqCst)
            && start_time.elapsed().unwrap().as_millis() < acquisition_time as u128 {

                let mut guard = buf.as_ref().write().unwrap();

                let tick = std::time::Instant::now();
                // println!("Expected {} photons for an interval of {}", expected_photons, tick.duration_since(last_tick).as_secs_f64());
                let n_photons = Poisson::new(
                    mean_rate * tick.duration_since(last_tick).as_secs_f64()
                ).unwrap().sample(&mut rng) as usize;
                
                for _ in 0..n_photons as usize {
                    let arrival_time = rand::random::<u16>() % (1<<14);
                    let channel = rand::random::<u8>() % 4;
                    let syncs = rand::random::<u16>() % (1<<10);
                    guard.0.push(
                        ((channel as u32) << 26)
                        | ((arrival_time as u32) << 10)
                        | (syncs as u32)
                    );
                }
                guard.1 += n_photons as usize;
                last_tick = tick;
            }
        }));

        Ok(())
    }

    fn stop_measurement(&mut self) -> Result<(), MultiHarpError> {
        self._ctc_status = false;
        self._acquiring.store(false, std::sync::atomic::Ordering::SeqCst);
        self._acq_thread.take()
            .ok_or(MultiHarpError::NotInitialized)?.join().unwrap();
        Ok(())
    }

    fn read_fifo<'a, 'b>(&'a self, buffer : &'b mut Vec<u32>) -> CheckedResult<i32, u32> {
        if buffer.len() < mhconsts::TTREADMAX {
            return Err(PatinaError::ArgumentError(
                "buffer".to_string(),
                buffer.len() as u32,
                format!("Buffer must be at least {} long", mhconsts::TTREADMAX))
            );
        }
        let mut read = self._internal_buffer.as_ref().write()
        .map_err(|e| 
            PatinaError::MultiHarpError(MultiHarpError::ThreadStateFail)
        )?;
        
        if read.1 > TTREADMAX {
            return Err(PatinaError::MultiHarpError(MultiHarpError::FIFOResetFail));
        }

        buffer[..read.1].clone_from_slice(&read.0[..read.1]);
        let returned = read.1;
        read.1 = 0;
        Ok(returned as i32)
    } 

    fn get_histogram_by_copy(&mut self, channel : i32) -> CheckedResult<Vec<u32>, i32> {
        Ok(vec![0])
    }

    fn get_all_histograms_by_copy(&mut self) -> MultiHarpResult<Vec<u32>>{
        Ok(vec![0])
    }

    fn fill_histogram<'a, 'b>(&'a mut self, histogram : &'b mut Vec<u32>, channel : i32) -> CheckedResult<(), i32> {
        Ok(())
    }

    fn fill_all_histograms<'a, 'b>(&'a mut self, histograms : &'b mut Vec<u32>) -> MultiHarpResult<()> {
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
        self._acquiring.store(false, std::sync::atomic::Ordering::SeqCst);
        self._acq_thread.take().map(|t| t.join().unwrap());
        unsafe { OCCUPIED_DEBUG_DEVICES.retain(|&x| x != self.index); }
    }
}

#[cfg(test)]
mod tests {
    use crate::MultiHarpDevice;

    use super::DebugMultiHarp150;

    #[test]
    fn test_basic_debug_multiharp(){
        let mut mh = DebugMultiHarp150::new(5e5, 80e6, None);
        
        let mut buffer = vec![0u32; crate::TTREADMAX];
        // First stop the measurement with "stop_measurement"
        println!{"Starting read for 10 sec"}
        mh.start_measurement(3000).unwrap();
        std::thread::sleep(std::time::Duration::from_secs_f64(2.0));
        let n_measurements = mh.read_fifo(&mut buffer).unwrap();

        // Panic if it's an error.
        mh.stop_measurement().unwrap();
        
        assert!(
            (n_measurements as f64) < 11.0e5 
            && (n_measurements as f64) > 9e5
        );

        mh.set_mean_count_rate(8000.0);
        
        // Now stop it with the internal timer
        mh.start_measurement(1000).unwrap();
        std::thread::sleep(std::time::Duration::from_secs_f64(2.0));

        let n_measurements = mh.read_fifo(&mut buffer).unwrap();
        
        mh.stop_measurement().unwrap();

        assert!(
            (n_measurements as f64) < 9000.0 
            && (n_measurements as f64) > 7000.0
        );

    }
}