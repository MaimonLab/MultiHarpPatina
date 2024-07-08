//! Code for interfacing with a MultiHarp 150

use std::ffi::*;
use crate::error::{MultiHarpError, PatinaError, mh_to_result};
use crate::{mhconsts, TriggerEdge};
use crate::mhlib::*;
use crate::MultiHarpConfig;
use crate::{available_devices, MHDeviceIterator};

/// A trait for MultiHarp devices -- must implement
/// all of the below methods.
#[allow(unused_variables)]
pub trait MultiHarpDevice : Sized {

    /// Calls many `set_` functions to set the device with
    /// the configuration provided. TODO make this report failures!
    fn set_from_config(&mut self, config : &MultiHarpConfig) -> () {

        if let Some(sync_div) = config.sync_div {
            self.set_sync_div(sync_div);
        }
        if let Some(sync_trigger_edge) = config.sync_trigger_edge {
            self.set_sync_edge_trigger(sync_trigger_edge.0, sync_trigger_edge.1);
        }

        if let Some(sync_offset) = config.sync_channel_offset {
            self.set_sync_channel_offset(sync_offset);
        }

        if let Some(sync_enable) = config.sync_channel_enable {
            self.set_sync_channel_enable(sync_enable);
        }

        if let Some(sync_deadtime) = config.sync_dead_time {
            self.set_sync_dead_time(sync_deadtime.0, sync_deadtime.1);
        }

        if let Some(input_edges) = &config.input_edges {
            for (i, (level, edge)) in input_edges.iter().enumerate() {
                self.set_input_edge_trigger(i as i32, *level, *edge);
            }
        }

        if let Some(input_offsets) = &config.input_offsets {
            for (i, offset) in input_offsets.iter().enumerate() {
                self.set_input_channel_offset(i as i32, *offset);
            }
        }

        if let Some(input_enable) = &config.input_enables {
            for (i, enable) in input_enable.iter().enumerate() {
                self.set_input_channel_enable(i as i32, *enable);
            }
        }

        if let Some(input_deadtimes) = &config.input_dead_times {
            for (i, (on, deadtime)) in input_deadtimes.iter().enumerate() {
                self.set_input_dead_time(i as i32, *on, *deadtime);
            }
        }

        if let Some(input_hysteresis) = config.input_hysteresis {
            self.set_input_hysteresis(input_hysteresis);
        }

        if let Some(stop_overflow) = config.stop_overflow {
            self.set_stop_overflow(stop_overflow.0, stop_overflow.1);
        }

        if let Some(binning) = config.binning {
            self.set_binning(binning);
        }

        if let Some(offset) = config.offset {
            self.set_offset(offset);
        }

        if let Some(histo_len) = config.histo_len {
            self.set_histogram_len(histo_len);
        }

        if let Some(meas_control) = config.meas_control {
            self.set_measurement_control_mode(meas_control.0, meas_control.1, meas_control.2);
        }

        if let Some(trigger_output) = config.trigger_output {
            self.set_trigger_output(trigger_output);
        }

        if let Some(ofl_compression) = config.ofl_compression {
            self.set_overflow_compression(ofl_compression);
        }

        if let Some(marker_edges) = config.marker_edges {
            self.set_marker_edges(marker_edges[0], marker_edges[1], marker_edges[2], marker_edges[3]);
        }

        if let Some(marker_enable) = config.marker_enable {
            self.set_marker_enable(marker_enable[0], marker_enable[1], marker_enable[2], marker_enable[3]);
        }

        if let Some(marker_holdoff) = config.marker_holdoff {
            self.set_marker_holdoff_time(marker_holdoff);
        }
    }

    fn open(index : Option<i32>) -> Result<Self, PatinaError<i32>>;
    fn open_by_serial(serial : &str) -> Result<Self, PatinaError<i32>>;
    fn init(&mut self, mode : mhconsts::MeasurementMode, reference_clock : mhconsts::ReferenceClock) -> Result<(), MultiHarpError>;
    fn get_hardware_info(&self) -> Result<(String, String, String), MultiHarpError>{
        Ok(("".to_string(), "".to_string(), "".to_string()))
    }
    fn get_base_resolution(&self) -> Result<(f64, i32), MultiHarpError>{
        Ok((5.0,0))
    }
    fn num_input_channels(&self) -> Result<i32, MultiHarpError>{ Ok(4) }
    fn get_debug_info(&self) -> Result<String, MultiHarpError>{ Ok ("No debug info".to_string()) }
    
    fn set_sync_div(&mut self, sync_div : i32) -> Result<(), PatinaError<i32>>{
        if sync_div < mhconsts::SYNCDIVMIN || sync_div > mhconsts::SYNCDIVMAX {
            return Err(PatinaError::ArgumentError(
                "sync_div".to_string(),
                sync_div,
                format!("Sync divider must be between {} and {}", mhconsts::SYNCDIVMIN, mhconsts::SYNCDIVMAX))
            );
        }
        Ok(())
    }

    fn set_sync_edge_trigger(&mut self, level : i32, edge : mhconsts::TriggerEdge) -> Result<(), PatinaError<i32>>{
        if level < mhconsts::TRGLVLMIN || level > mhconsts::TRGLVLMAX {
            return Err(PatinaError::ArgumentError(
                "level".to_string(),
                level,
                format!("Level must be between {} and {}", mhconsts::TRGLVLMIN, mhconsts::TRGLVLMAX))
            );
        }
        Ok(())
    }

    fn set_sync_channel_offset(&mut self, offset : i32) -> Result<(), PatinaError<i32>>{
        if offset < mhconsts::CHANNEL_OFFS_MIN || offset > mhconsts::CHANNEL_OFFS_MAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Channel offset must be between {} and {}", mhconsts::CHANNEL_OFFS_MIN, mhconsts::CHANNEL_OFFS_MAX))
            );
        }
        Ok(())
    }

    fn set_sync_channel_enable(&mut self, enable : bool) -> Result<(), PatinaError<i32>>{
        Ok(())
    }

    fn set_sync_dead_time(&mut self, on : bool, deadtime : i32) -> Result<(), PatinaError<i32>>{
        if deadtime < mhconsts::EXTDEADMIN || deadtime > mhconsts::EXTDEADMAX {
            return Err(PatinaError::ArgumentError(
                "deadtime".to_string(),
                deadtime,
                format!("Dead time must be between {} and {}", mhconsts::EXTDEADMIN, mhconsts::EXTDEADMAX))
            );
        }
        Ok(())    
    }

    fn set_input_edge_trigger(&mut self, channel : i32, level : i32, edge : mhconsts::TriggerEdge) -> Result<(), PatinaError<i32>>{
        if level < mhconsts::TRGLVLMIN || level > mhconsts::TRGLVLMAX {
            return Err(PatinaError::ArgumentError(
                "level".to_string(),
                level,
                format!("Level must be between {} and {}", mhconsts::TRGLVLMIN, mhconsts::TRGLVLMAX))
            );
        }
        Ok(())
    }

    fn set_input_channel_offset(&mut self, channel : i32, offset : i32) -> Result<(), PatinaError<i32>>{
        if offset < mhconsts::CHANNEL_OFFS_MIN || offset > mhconsts::CHANNEL_OFFS_MAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Channel offset must be between {} and {}", mhconsts::CHANNEL_OFFS_MIN, mhconsts::CHANNEL_OFFS_MAX))
            );
        }
        Ok(())
    }

    fn set_input_channel_enable(&mut self, channel : i32, enable : bool) -> Result<(), PatinaError<i32>>{
        Ok(())
    }

    fn set_input_dead_time(&mut self, channel : i32, on : bool, deadtime : i32) -> Result<(), PatinaError<i32>> {
        if deadtime < mhconsts::EXTDEADMIN || deadtime > mhconsts::EXTDEADMAX {
            return Err(PatinaError::ArgumentError(
                "deadtime".to_string(),
                deadtime,
                format!("Dead time must be between {} and {}", mhconsts::EXTDEADMIN, mhconsts::EXTDEADMAX))
            );
        }
        Ok(())
    }

    fn set_input_hysteresis(&mut self, hystcode : bool) -> Result<(), PatinaError<i32>> {
        Ok(())
    }

    fn set_stop_overflow(&mut self, stop_overflow : bool, stopcount : u32) -> Result<(), PatinaError<u32>> {
        if stopcount < mhconsts::STOPCNTMIN || stopcount > mhconsts::STOPCNTMAX {
            return Err(PatinaError::ArgumentError(
                "stopcount".to_string(),
                stopcount,
                format!("Stop count must be between {} and {}", mhconsts::STOPCNTMIN, mhconsts::STOPCNTMAX))
            );
        }

        Ok(())
    }

    fn set_binning(&mut self, binning : i32) -> Result<(), PatinaError<i32>> {
        if binning < 0 || binning > mhconsts::BINSTEPSMAX {
            return Err(PatinaError::ArgumentError(
                "binning".to_string(),
                binning,
                format!("Binning must be between 0 and {}", mhconsts::BINSTEPSMAX))
            );
        }
        Ok(())
    }

    fn set_offset(&mut self, offset : i32) -> Result<(), PatinaError<i32>> {
        if offset < mhconsts::OFFSETMIN || offset > mhconsts::OFFSETMAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Offset must be between {} and {}", mhconsts::OFFSETMIN, mhconsts::OFFSETMAX))
            );
        }
        Ok(())
    }

    fn set_histogram_len(&mut self, lencode : i32) -> Result<i32, PatinaError<i32>> {
        if lencode < mhconsts::MINLENCODE || lencode > mhconsts::MAXLENCODE {
            return Err(PatinaError::ArgumentError(
                "lencode".to_string(),
                lencode,
                format!("Length code must be between {} and {}", mhconsts::MINLENCODE, mhconsts::MAXLENCODE))
            );
        }
        Ok(65536)
    }

    fn clear_histogram(&mut self) -> Result<(), MultiHarpError> {Ok(())}

    fn set_measurement_control_mode(
        &mut self,
        mode : mhconsts::MeasurementControlMode,
        start_edge : Option<TriggerEdge>,
        stop_edge : Option<TriggerEdge>,
    ) -> Result<(), PatinaError::<String>>{
        Err(PatinaError::NotImplemented)
    }

    fn set_trigger_output(&mut self, period : i32) -> Result<(), PatinaError<i32>>{
        if period < mhconsts::TRIGOUTMIN || period > mhconsts::TRIGOUTMAX {
            return Err(PatinaError::ArgumentError(
                "period".to_string(),
                period,
                format!("Period must be between {} and {}", mhconsts::TRIGOUTMIN, mhconsts::TRIGOUTMAX))
            );
        }
        Ok(())
    }

    fn start_measurement(&mut self, acquisition_time : i32) -> Result<(), PatinaError<i32>>;
    fn stop_measurement(&mut self) -> Result<(), MultiHarpError>;
    fn ctc_status(&self) -> Result<bool, MultiHarpError>;

    fn fill_histogram(&self, histogram : &mut Vec<u32>, channel : i32) -> Result<(), PatinaError<i32>> {Ok(())}

    fn fill_all_histograms(&self, histograms : &mut Vec<u32>) -> Result<(), MultiHarpError> {Ok(())}

    fn get_histogram_by_copy(&self, channel : i32) -> Result<Vec<u32>, PatinaError<i32>> {Ok(vec![0; 65536])}

    fn get_all_histograms_by_copy(&self) -> Result<Vec<u32>, MultiHarpError> {Ok(vec![0; 65536 * 4])}

    fn get_resolution(&self) -> Result<f64, MultiHarpError> {Ok(5.0)}

    fn get_sync_rate(&self) -> Result<i32, MultiHarpError> {Ok(78e6 as i32)}

    fn get_sync_period(&self) -> Result<f64, MultiHarpError> {Ok(1.0 / 78e6)}

    fn get_count_rate(&self, channel : i32) -> Result<i32, PatinaError<i32>>{Ok(1e5 as i32)}

    fn get_all_count_rates(&self) -> Result<(i32, Vec<i32>), MultiHarpError> {Ok((78e6 as i32, vec![1e5 as i32; 4]))}

    fn get_flags(&self) -> Result<i32, MultiHarpError> {Ok(0)}

    fn get_warnings(&self) -> Result<i32, MultiHarpError> {Ok(0)}

    fn get_warnings_text(&self) -> Result<String, MultiHarpError> {Ok("No warnings".to_string())}

    fn get_elapsed_measurement_time(&self) -> Result<f64, MultiHarpError> {Ok(0.0)}

    fn get_start_time(&self) -> Result<(u32, u32, u32), MultiHarpError> {Ok((0, 0, 0))}

    fn read_fifo(&self, buffer : &mut Vec<u32>) -> Result<i32, PatinaError<u32>> {
        Ok(0)
    }

    fn set_marker_edges(&mut self, me1 : TriggerEdge, me2 : TriggerEdge, me3 : TriggerEdge, me4 : TriggerEdge) -> Result<(), MultiHarpError> {Ok(())}

    fn set_marker_enable(&mut self, en1 : bool, en2 : bool, en3 : bool, en4 : bool) -> Result<(), MultiHarpError> {Ok(())}

    fn set_marker_holdoff_time(&mut self, holdofftime : i32) -> Result<(), PatinaError<i32>> {
        if holdofftime < mhconsts::HOLDOFFMIN || holdofftime > mhconsts::HOLDOFFMAX {
            return Err(PatinaError::ArgumentError(
                "holdofftime".to_string(),
                holdofftime,
                format!("Holdoff time must be between {} and {}", mhconsts::HOLDOFFMIN, mhconsts::HOLDOFFMAX))
            );
        }
        Ok(())
    }

    fn set_overflow_compression(&mut self, holdtime : i32) -> Result<(), PatinaError<i32>> {
        if holdtime < mhconsts::HOLDTIMEMIN || holdtime > mhconsts::HOLDTIMEMAX {
            return Err(PatinaError::ArgumentError(
                "holdtime".to_string(),
                holdtime,
                format!("Hold time must be between {} and {}", mhconsts::HOLDTIMEMIN, mhconsts::HOLDTIMEMAX))
            );
        }
        Ok(())
    }

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
    initialized : bool,
    num_channels : i32,
    features : i32, // marks which features are available on this device.
}

impl MultiHarpDevice for MultiHarp150 {
    /// Open a MultiHarp device by index.
    /// 
    /// ## Arguments
    /// 
    /// * `index` - The index of the device to open (0..7).
    /// If no index is provided, will open the first `MultiHarp`
    /// encountered.
    /// 
    /// ## Returns
    /// 
    /// A `Result` containing the opened MultiHarp device
    /// or an error.
    /// 
    /// ## Errors
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
        if mh_result != 0 {
            return Err(PatinaError::from(MultiHarpError::from(mh_result)));
        }
        let mut num_channels = 0i32;
        let channels_result = unsafe{ MH_GetNumOfInputChannels(index, &mut num_channels) };

        if channels_result != 0 {

            return Err(PatinaError::from(MultiHarpError::from(channels_result)));
        }

        let mut features = 0i32;
        let features_result = unsafe { MH_GetFeatures(index, &mut features) };

        if features_result != 0 {
            return Err(PatinaError::from(MultiHarpError::from(features_result)));
        }

        Ok(
            MultiHarp150 {
                index,
                serial: unsafe { CString::from_raw(serial.as_mut_ptr()) }.to_str().unwrap().to_string(),
                initialized: false,
                num_channels,
                features,
            }
        )
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
    /// ## Arguments
    /// 
    /// * `mode` - The measurement mode to initialize the device in.
    /// 
    /// * `reference_clock` - The reference clock to use for the device.
    /// 
    /// ## Returns
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
    /// ## Returns
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
    /// ## Returns
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


    //////// SETTERS //////////////

    /// Sets the divider of the sync signal, should be used to keep the
    /// effective sync rate below 78 MHz. The larger the divider, the greater
    /// the jitter in estimated timing of the sync signals. The output of
    /// `get_count_rate` is internally corrected for the sync divider, and should
    /// not be adjusted by this value.
    /// 
    /// ## Arguments
    /// 
    /// * `sync_div` - The sync divider to set. Must be between 1 and 16.
    fn set_sync_div(&mut self, sync_div : i32) -> Result<(), PatinaError<i32>> {
        if sync_div < mhconsts::SYNCDIVMIN || sync_div > mhconsts::SYNCDIVMAX {
            return Err(PatinaError::ArgumentError(
                "sync_div".to_string(),
                sync_div,
                format!("Sync divider must be between {} and {}", mhconsts::SYNCDIVMIN, mhconsts::SYNCDIVMAX))
            );
        } 
        let mh_result = unsafe { MH_SetSyncDiv(self.index, sync_div) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the level and edge of the sync signal to trigger on.
    /// 
    /// ## Arguments
    /// 
    /// * `level` - The level of the sync signal to trigger on (in millivolts). Must be between -1200 and 1200 mV.
    ///  (note, the hardware uses a 10 bit DAC, and so this is only set to within 2.34 mV)
    /// 
    /// * `edge` - The edge of the sync signal to trigger on.
    fn set_sync_edge_trigger(&mut self, level : i32, edge : mhconsts::TriggerEdge) -> Result<(), PatinaError<i32>> {
        if level < mhconsts::TRGLVLMIN || level > mhconsts::TRGLVLMAX {
            return Err(PatinaError::ArgumentError(
                "level".to_string(),
                level,
                format!("Level must be between {} and {}", mhconsts::TRGLVLMIN, mhconsts::TRGLVLMAX))
            );
        }
        let mh_result = unsafe { MH_SetSyncEdgeTrg(self.index, level as c_int, edge as c_int) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the timing offset of the sync channel in picoseconds.
    /// 
    /// ## Arguments
    /// 
    /// * `offset` - The offset to set in picoseconds. Must be between -99999 and 99999 ps.
    fn set_sync_channel_offset(&mut self, offset : i32) -> Result<(), PatinaError<i32>> {
        if offset < mhconsts::CHANNEL_OFFS_MIN || offset > mhconsts::CHANNEL_OFFS_MAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Offset must be between {} and {}", mhconsts::CHANNEL_OFFS_MIN, mhconsts::CHANNEL_OFFS_MAX))
            );
        }
        let mh_result = unsafe { MH_SetSyncChannelOffset(self.index, offset) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Enables or disables the sync channel. Only useful in T2 mode
    fn set_sync_channel_enable(&mut self, enable : bool) -> Result<(), PatinaError<i32>> {
        let mh_result = unsafe { MH_SetSyncChannelEnable(self.index, enable as i32) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the dead time of the sync signal. This function is used to suppress
    /// afterpulsing artifacts in some detectors. The dead time is in picoseconds
    /// 
    /// ## Arguments
    /// 
    /// * `on` - Whether to turn the dead time on or off. 0 is off, 1 is on.
    /// 
    /// * `deadtime` - The dead time to set in picoseconds.
    fn set_sync_dead_time(&mut self, on : bool, deadtime : i32) -> Result<(), PatinaError<i32>> {
        if (self.features & (mhconsts::FeatureMasks::ProgTd as i32)) == 0 {
            return Err(PatinaError::FeatureNotAvailable("Programmable dead time".to_string()));
        }
        if deadtime < mhconsts::EXTDEADMIN || deadtime > mhconsts::EXTDEADMAX {
            return Err(PatinaError::ArgumentError(
                "deadtime".to_string(),
                deadtime,
                format!("Dead time must be between {} and {}", mhconsts::EXTDEADMIN, mhconsts::EXTDEADMAX))
            );
        }

        let mh_result = unsafe { MH_SetSyncDeadTime(self.index, on as i32, deadtime) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the level and edge for photon detection of the channel specified.
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to set the input edge trigger for. Must be an available channel for
    ///  the device.
    /// 
    /// * `level` - The level of the input signal to trigger on (in millivolts). Must be between -1200 and 1200 mV.
    /// 
    /// * `edge` - The edge of the input signal to trigger on.
    /// 
    fn set_input_edge_trigger(&mut self, channel : i32, level : i32, edge : mhconsts::TriggerEdge) -> Result<(), PatinaError<i32>> {
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }
        
        if level < mhconsts::TRGLVLMIN || level > mhconsts::TRGLVLMAX {
            return Err(PatinaError::ArgumentError(
                "level".to_string(),
                level,
                format!("Level must be between {} and {}", mhconsts::TRGLVLMIN, mhconsts::TRGLVLMAX))
            );
        }
        let mh_result = unsafe { MH_SetInputEdgeTrg(self.index, channel, level, edge as c_int) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the offset of the input channel in picoseconds. This is equivalent to
    /// changing the cable delay on the chosen input. The actual offset resolution
    /// is in the device's base resolution.
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to set the offset for. Must be an available channel for the device.
    /// 
    /// * `offset` - The offset to set in picoseconds. Must be between -99999 and 99999 ps.
    fn set_input_channel_offset(&mut self, channel : i32, offset : i32) -> Result<(), PatinaError<i32>> {
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }

        if offset < mhconsts::CHANNEL_OFFS_MIN || offset > mhconsts::CHANNEL_OFFS_MAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Offset must be between {} and {}", mhconsts::CHANNEL_OFFS_MIN, mhconsts::CHANNEL_OFFS_MAX))
            );
        }
        let mh_result = unsafe { MH_SetInputChannelOffset(self.index, channel, offset) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Enables or disables the input channel.
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to set the enable for. Must be an available channel for the device.
    /// 
    /// * `enable` - Whether to enable the channel. 0 is off, 1 is on.
    fn set_input_channel_enable(&mut self, channel : i32, enable : bool) -> Result<(), PatinaError<i32>> {
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }
        let mh_result = unsafe { MH_SetInputChannelEnable(self.index, channel, enable as i32) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Set the dead time of the input channel. Used to suppress afterpulsing artifacts
    /// in some detectors. The dead time is in picoseconds.
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to set the dead time for. Must be an available channel for the device.
    /// 
    /// * `on` - Whether to turn the dead time on or off. 0 is off, 1 is on.
    /// 
    /// * `deadtime` - The dead time to set in picoseconds.
    fn set_input_dead_time(&mut self, channel : i32, on : bool, deadtime : i32) -> Result<(), PatinaError<i32>> {
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }
        
        if deadtime < mhconsts::EXTDEADMIN || deadtime > mhconsts::EXTDEADMAX {
            return Err(PatinaError::ArgumentError(
                "deadtime".to_string(),
                deadtime,
                format!("Dead time must be between {} and {}", mhconsts::EXTDEADMIN, mhconsts::EXTDEADMAX))
            );
        }
        let mh_result = unsafe { MH_SetInputDeadTime(self.index, channel, on as i32,  deadtime) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Used to accommodate hysteresis on the input and sync channels for detectors
    /// with long pulse shape artifacts. New in firmware version 3.0
    /// 
    /// ## Arguments
    /// 
    /// * `hystcode` - The hysteresis code to set. Must be 0 (for 3 mV) or 1 (for 35 mV).
    fn set_input_hysteresis(&mut self, hystcode : bool) -> Result<(), PatinaError<i32>> {
        if (self.features & (mhconsts::FeatureMasks::ProgHyst as i32)) == 0 {
            return Err(PatinaError::FeatureNotAvailable("Hysteresis".to_string()));
        }
        let mh_result = unsafe { MH_SetInputHysteresis(self.index, hystcode as i32) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Determines if a measurement will stop when the histogram overflows.
    /// 
    /// ## Arguments
    /// 
    /// * `stop_overflow` - Whether to stop on overflow. 0 is off, 1 is on.
    /// 
    /// * `stopcount` - The number of counts to stop on. Must be between 1 and 4294967295.
    fn set_stop_overflow(&mut self, stop_overflow : bool, stopcount : u32) -> Result<(), PatinaError<u32>> {

        if stopcount < mhconsts::STOPCNTMIN || stopcount > mhconsts::STOPCNTMAX {
            return Err(PatinaError::ArgumentError(
                "stopcount".to_string(),
                stopcount,
                format!("Stop count must be between {} and {}", mhconsts::STOPCNTMIN, mhconsts::STOPCNTMAX))
            );
        }

        let mh_result = unsafe { MH_SetStopOverflow(self.index, stop_overflow as i32, stopcount) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Only applies in Histogramming or T3 mode. The binning corresponds to repeated
    /// doubling, i.e. 0 is no binning, 1 is 2x binning, 2 is 4x binning, etc.
    /// 
    /// ## Arguments
    /// 
    /// * `binning` - The binning to set. Must be between 0 and 24 (corresponding to
    /// pooling 2^0 to 2^24 bins).
    fn set_binning(&mut self, binning : i32) -> Result<(), PatinaError<i32>> {
        if binning < 0 || binning > mhconsts::BINSTEPSMAX {
            return Err(PatinaError::ArgumentError(
                "binning".to_string(),
                binning,
                format!("Binning must be between 0 and {}", mhconsts::BINSTEPSMAX))
            );
        }
        let mh_result = unsafe { MH_SetBinning(self.index, binning) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the overall offset subtracted from the difference between stop and start,
    /// intended for situations where the range of the histogram is not long enough
    /// to look at "late" data. This offset shifts teh "window of view" of the histogram.
    /// This is NOT the same as changing or compensating for cable delays!
    /// 
    /// ### See also
    /// 
    /// - `set_input_channel_offset`
    /// - `set_sync_channel_offset`
    fn set_offset(&mut self, offset : i32) -> Result<(), PatinaError<i32>> {
        if offset < mhconsts::OFFSETMIN || offset > mhconsts::OFFSETMAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Offset must be between {} and {}", mhconsts::OFFSETMIN, mhconsts::OFFSETMAX))
            );
        }
        let mh_result = unsafe { MH_SetOffset(self.index, offset) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Sets the number of bins of the histograms collected. The histogram length
    /// obtained with `MAXLENCODE` = 6 is `65536` bins, calculated as 1024*(2^LENCODE).
    /// Returns the current length of histograms (e.g 65536 for `MAXLENCODE` = 6).
    /// 
    /// ## Arguments
    /// 
    /// * `lencode` - The length code to set. Must be between 0 and 6. Actual length
    /// will be 1024*(2^lencode).
    /// 
    /// ## Returns
    /// 
    /// * `Result<i32, PatinaError<i32>>` - The actual length of the histogram.
    fn set_histogram_len(&mut self, lencode : i32) -> Result<i32, PatinaError<i32>> {
        if lencode < mhconsts::MINLENCODE || lencode > mhconsts::MAXLENCODE {
            return Err(PatinaError::ArgumentError(
                "lencode".to_string(),
                lencode,
                format!("Length code must be between {} and {}", mhconsts::MINLENCODE, mhconsts::MAXLENCODE))
            );
        }
        let mut actual_lencode = 0;
        let mh_result = unsafe { MH_SetHistoLen(self.index, lencode, &mut actual_lencode) };
        mh_to_result!(mh_result, actual_lencode).map_err(|e| PatinaError::from(e))
    }

    /// Clears the histogram of the device. Does nothing if in T2 or T3 mode
    fn clear_histogram(&mut self) -> Result<(), MultiHarpError> {
        let mh_result = unsafe { MH_ClearHistMem(self.index) };
        mh_to_result!(mh_result, ())
    }

    /// Set the mode by which measurements are controlled. Default mode is
    /// `SingleShotCTC`, in which the software triggers a measurement which 
    /// lasts as long as `tacq`. In the `Gated` modes, a hardware trigger can
    /// be used to initiate and cease measurements. In the `SwStartSwStop` mode,
    /// `tacq` is bypassed, at the expense of a slightly less accurate timestamp
    /// for the elapsed measurement time (available in >=v3.1).
    /// 
    /// ## Arguments
    /// 
    /// * `mode` - The mode to set the measurement control to.
    /// 
    /// * `start_edge` - The edge to start the measurement on. Only required for `Gated` modes.
    /// 
    /// * `stop_edge` - The edge to stop the measurement on. Only required for `Gated` modes.
    fn set_measurement_control_mode(
        &mut self,
        mode : mhconsts::MeasurementControlMode,
        start_edge : Option<TriggerEdge>,
        stop_edge : Option<TriggerEdge>,
    ) -> Result<(), PatinaError<String>> {

        match mode {
            mhconsts::MeasurementControlMode::C1Gated => {
                if start_edge.is_none() || stop_edge.is_none() {
                    return Err(PatinaError::ArgumentError(
                        "mode".to_string(),
                        ( mode as i32 ).to_string(),
                        "Gated mode requires start and stop edges".to_string())
                    );
                }
                let start_edge = start_edge.unwrap();
                let stop_edge = stop_edge.unwrap();
                let mh_result = unsafe { MH_SetMeasControl(self.index, mode as c_int, start_edge as i32, stop_edge as i32) };
                return mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
            }

            mhconsts::MeasurementControlMode::C1StartCtcStop => {
                if start_edge.is_none(){
                    return Err(PatinaError::ArgumentError(
                        "mode".to_string(),
                        ( mode as i32 ).to_string(),
                        "C1StartCtcStop mode requires a start edge".to_string())
                    );
                }
                let start_edge = start_edge.unwrap();
                let stop_edge = 0;
                let mh_result = unsafe { MH_SetMeasControl(self.index, mode as c_int, start_edge as i32, stop_edge) };
                return mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
            }
            mhconsts::MeasurementControlMode::C1StartC2Stop => {
                if start_edge.is_none() || stop_edge.is_none() {
                    return Err(PatinaError::ArgumentError(
                        "mode".to_string(),
                        ( mode as i32 ).to_string(),
                        "C1StartC2Stop mode requires a start edge and a stop edge".to_string())
                    );
                }
                let start_edge = start_edge.unwrap();
                let stop_edge = stop_edge.unwrap();
                let mh_result = unsafe { MH_SetMeasControl(self.index, mode as c_int, start_edge as i32, stop_edge as i32) };
                return mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
            }
            _ => {
                let mh_result = unsafe { MH_SetMeasControl(self.index, mode as c_int, 0, 0) };
                return mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
            }
        }
    }

    /// Sets the period of the programmable trigger output. Setting the
    /// period to 0 switches it off. The period is set in units of 100 nanoseconds.
    /// 
    /// ## Arguments
    /// 
    /// * `period` - The period to set in units of 100 ns.
    fn set_trigger_output(&mut self, period : i32) -> Result<(), PatinaError<i32>>{
        if (self.features & (mhconsts::FeatureMasks::TrigOut as i32)) == 0 {
            return Err(PatinaError::FeatureNotAvailable("Trigger Output".to_string()));
        }
        if period < mhconsts::TRIGOUTMIN || period > mhconsts::TRIGOUTMAX {
            return Err(PatinaError::ArgumentError(
                "period".to_string(),
                period,
                format!("Period must be between {} and {}", mhconsts::TRIGOUTMIN, mhconsts::TRIGOUTMAX))
            );
        }
        let mh_result = unsafe { MH_SetTriggerOutput(self.index, period) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Starts a measurement with the given acquisition time in milliseconds
    /// 
    /// ## Arguments
    /// 
    /// * `acquisition_time` - The acquisition time to set in milliseconds. Must be between 1 and 3600000 ms = 100 hours.
    /// 
    /// ### See also
    /// 
    /// - `set_measurement_control_mode` - If the software library version is >3.1, this
    /// can be used to bypass the `acquistion_time` parameter entirely, permitting very
    /// very long acquisitions.
    fn start_measurement(&mut self, acquisition_time : i32) -> Result<(), PatinaError<i32>> {
        if acquisition_time < mhconsts::ACQTMIN || acquisition_time > mhconsts::ACQTMAX {
            return Err(PatinaError::ArgumentError(
                "acquisition_time".to_string(),
                acquisition_time,
                format!("Acquisition time must be between {} and {}", mhconsts::ACQTMIN, mhconsts::ACQTMAX))
            );
        }
        let mh_result = unsafe { MH_StartMeas(self.index, acquisition_time) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Stops the current measurement. Must be called after `start_measurement`, even
    /// if it expires due to the `acquisition_time` parameter.
    fn stop_measurement(&mut self) -> Result<(), MultiHarpError> {
        let mh_result = unsafe { MH_StopMeas(self.index) };
        mh_to_result!(mh_result, ())
    }

    /// Reports whether there is an ongoing measurement.
    fn ctc_status(&self) -> Result<bool, MultiHarpError> {
        let mut ctc_status = 0;
        let mh_result = unsafe { MH_CTCStatus(self.index, &mut ctc_status) };
        mh_to_result!(mh_result, ctc_status != 0)
    }

    /// Returns an arrival time histogram from the device. This makes a copy, rather
    /// than filling an existing buffer.
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to get the histogram for. Must be an available channel for the device.
    /// 
    /// ## Returns
    /// 
    /// * `Vec<u32>` - The histogram of arrival times, of length determined by the
    /// current histogram length TODO: make it actually determined, currently just MAXHISTLEN
    fn get_histogram_by_copy(&self, channel : i32) -> Result<Vec<u32>, PatinaError<i32>> {
        let mut histogram = vec![0u32; mhconsts::MAXHISTLEN];
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }

        let mh_result = unsafe { MH_GetHistogram(self.index, histogram.as_mut_ptr(), channel) };
        mh_to_result!(mh_result, histogram).map_err(|e| PatinaError::from(e))
    }

    /// Returns all histograms from the device. This makes a copy, rather
    /// than filling an existing buffer.
    fn get_all_histograms_by_copy(&self) -> Result<Vec<u32>, MultiHarpError> {
        let mut histograms = vec![0u32; mhconsts::MAXHISTLEN * self.num_channels as usize];
        let mh_result = unsafe { MH_GetAllHistograms(self.index, histograms.as_mut_ptr()) };
        mh_to_result!(mh_result, histograms)
    }

    /// Fills an existing buffer with the arrival time histogram from the device.
    /// TODO check if the buffer is the right size.
    /// 
    /// ## Arguments
    /// 
    /// * `histogram` - The buffer to fill with the histogram. Must be at least as long
    /// as the setting's histogram length. TODO check this arg!
    /// 
    /// * `channel` - The channel to get the histogram for. Must be an available channel for the device.
    fn fill_histogram(&self, histogram : &mut Vec<u32>, channel : i32) -> Result<(), PatinaError<i32>> {
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }

        let mh_result = unsafe { MH_GetHistogram(self.index, histogram.as_mut_ptr(), channel) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Populates an existing buffer with all histograms from the device. Expects
    /// a buffer for all channels, so the buffer must be at least `num_channels * histogram_length`
    /// long. TODO: actually provide checking!
    /// 
    /// ## Arguments
    /// 
    /// * `histograms` - The buffer to fill with all histograms. Must be at least as long
    /// as the setting's histogram length times the number of channels. TODO check this arg!
    fn fill_all_histograms(&self, histograms : &mut Vec<u32>) -> Result<(), MultiHarpError> {
        let mh_result = unsafe { MH_GetAllHistograms(self.index, histograms.as_mut_ptr()) };
        mh_to_result!(mh_result, ())
    }

    /// Returns the resolution of the bins in the histogram in picoseconds. Not meaningful
    /// in T2 mode.
    fn get_resolution(&self) -> Result<f64, MultiHarpError> {
        let mut resolution = 0.0;
        let mh_result = unsafe { MH_GetResolution(self.index, &mut resolution) };
        mh_to_result!(mh_result, resolution)
    }

    /// Returns the sync rate in Hz. Requires at least 100 ms of data to be collected
    fn get_sync_rate(&self) -> Result<i32, MultiHarpError> {
        let mut sync_rate = 0;
        let mh_result = unsafe { MH_GetSyncRate(self.index, &mut sync_rate) };
        mh_to_result!(mh_result, sync_rate)
    }

    /// Returns the count rate of the specified channel in photons per second
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to get the count rate for. Must be an available channel for the device.
    fn get_count_rate(&self, channel : i32) -> Result<i32, PatinaError<i32>> {
        if channel < 0 || channel >= self.num_channels {
            return Err(PatinaError::ArgumentError(
                "channel".to_string(),
                channel,
                format!("Channel must be between 0 and {}", self.num_channels - 1))
            );
        }
        let mut count_rate = 0;
        let mh_result = unsafe { MH_GetCountRate(self.index, channel, &mut count_rate) };
        mh_to_result!(mh_result, count_rate).map_err(|e| PatinaError::from(e))
    }

    /// Returns the count rates of all channels in photons per second and the sync rate
    /// in Hz.
    fn get_all_count_rates(&self) -> Result<(i32, Vec<i32>), MultiHarpError> {
        let mut sync_rate : i32 = 0;
        let mut count_rates = vec![0i32; self.num_channels as usize];
        let mh_result = unsafe { MH_GetAllCountRates(self.index, &mut sync_rate, count_rates.as_mut_ptr()) };
        mh_to_result!(mh_result, (sync_rate, count_rates))
    }

    /// Returns the set flags of the device, interpretable using
    /// the bitmasks in `mhconsts`.
    /// 
    /// ### See also
    /// 
    /// - `get_warnings` - To get the warning flags.
    fn get_flags(&self) -> Result<i32, MultiHarpError> {
        let mut flags = 0;
        let mh_result = unsafe { MH_GetFlags(self.index, &mut flags) };
        mh_to_result!(mh_result, flags)
    }

    /// Returns the set warnings of the device, interpretable using
    /// the bitmasks in `mhconsts`. Prior to this call, you must call
    /// `get_all_count_rates` or `get_sync_rate` and `get_count_rate` for
    /// all channels, otherwise at least some warnings will not be meaningful.
    /// 
    /// ### See also
    /// 
    /// - `get_flags`
    /// - `get_warnings_text`
    fn get_warnings(&self) -> Result<i32, MultiHarpError> {
        let mut warnings = 0;
        let mh_result = unsafe { MH_GetWarnings(self.index, &mut warnings) };
        mh_to_result!(mh_result, warnings)
    }

    /// Returns a human-readable string to interpret the device warnings
    /// 
    /// ### See also
    /// - `get_warnings`
    /// - `get_flags`
    fn get_warnings_text(&self) -> Result<String, MultiHarpError> {
        let warnings = self.get_warnings()?;
        let mut warnings_text = [0 as c_char; mhconsts::WARNLEN];
        let mh_result = unsafe { MH_GetWarningsText(self.index, warnings_text.as_mut_ptr(), warnings) };
        mh_to_result!(mh_result, unsafe { CString::from_raw(warnings_text.as_mut_ptr()) }.to_str().unwrap().to_string())
    }

    /// Returns the sync period in seconds. Resolution is the
    /// same as the device's resolution. Accuracy is determined by
    /// single shot jitter and clock stability.
    fn get_sync_period(&self) -> Result<f64, MultiHarpError> {
        let mut sync_period = 0.0;
        let mh_result = unsafe { MH_GetSyncPeriod(self.index, &mut sync_period) };
        mh_to_result!(mh_result, sync_period)
    }

    /// Returns the elapsed measurement time in milliseconds. When
    /// using the `SwStartSwStop` mode, these results will be less accurate.
    fn get_elapsed_measurement_time(&self) -> Result<f64, MultiHarpError> {
        let mut elapsed_time = 0.0;
        let mh_result = unsafe { MH_GetElapsedMeasTime(self.index, &mut elapsed_time) };
        mh_to_result!(mh_result, elapsed_time)
    }

    /// Returns the time of the last photon in the buffer in picoseconds since the
    /// epoch. It always relates to the start of the most recent measurement.
    /// With internal clocking, this is only as accurate as the PC clock itself.
    /// 
    /// Using 3 dwords provides a 96 bit timestamp, which is more than enough
    /// for most purposes.
    /// 
    /// ### Returns
    /// 
    /// * `dword2` - The most significant 32 bits of the time in picoseconds since epoch
    /// * `dword1` - The middle 32 bits of the time in picoseconds since epoch
    /// * `dword0` - The least significant 32 bits of the time in picoseconds since epoch
    /// 
    /// To convert to picoseconds since epoch, use the following formula:
    /// 
    /// (dword2 * 2^64) + (dword1 * 2^32) + dword0
    /// 
    /// which cannot be stored in a 64 bit uint or float, so be cautious!
    /// 
    fn get_start_time(&self) -> Result<(u32, u32, u32), MultiHarpError> {
        let (mut dword2, mut dword1, mut dword0) = (0u32, 0u32, 0u32);
        let mh_result = unsafe { MH_GetStartTime(self.index, &mut dword2, &mut dword1, &mut dword0) };
        mh_to_result!(mh_result, (dword2, dword1, dword0))
    }

    /// Loads a buffer with the arrival time data from the device. Returns the actual
    /// number of counts read. Only meaningful in TTTR mode.
    /// 
    /// ## Arguments
    /// 
    /// * `buffer` - The buffer to fill with the arrival time data. Must be at least
    /// `TTREADMAX` long.
    /// 
    /// ## Returns
    /// 
    /// * `Result<i32, PatinaError<u32>>` - The actual number of counts read. Data
    /// after this value is undefined.
    fn read_fifo(&self, buffer : &mut Vec<u32>) -> Result<i32, PatinaError<u32>> {
        if buffer.len() < mhconsts::TTREADMAX {
            return Err(PatinaError::ArgumentError(
                "buffer".to_string(),
                buffer.len() as u32,
                format!("Buffer must be at least {} long", mhconsts::TTREADMAX))
            );
        }
        let mut count = 0;
        let mh_result = unsafe { MH_ReadFiFo(self.index, buffer.as_mut_ptr(), &mut count) };
        mh_to_result!(mh_result, count).map_err(|e| PatinaError::from(e))
    }

    /// Sets the detection edges for each of the four marker channels (set simultaneously). Only
    /// meaningful in TTTR mode.
    fn set_marker_edges(&mut self, marker1 : TriggerEdge, marker2 : TriggerEdge, marker3 : TriggerEdge, marker4 : TriggerEdge) -> Result<(), MultiHarpError> {
        let mh_result = unsafe { MH_SetMarkerEdges(self.index, marker1 as c_int, marker2 as c_int, marker3 as c_int, marker4 as c_int) };
        mh_to_result!(mh_result, ())
    }

    /// Used to enable or disable individual TTL marker inputs. Only meaningful in TTTR mode.
    fn set_marker_enable(&mut self, enable1 : bool, enable2 : bool, enable3: bool, enable4 : bool) -> Result<(), MultiHarpError> {
        let mh_result = unsafe { MH_SetMarkerEnable(self.index, enable1 as i32, enable2 as i32, enable3 as i32, enable4 as i32) };
        mh_to_result!(mh_result, ())
    }

    /// Sets the holdoff time for the markers in nanoseconds. This is not normally required,
    /// but it can be useful to deal with marker line issues. The holdoff time sets the
    /// minimum time between markers. Only meaningful in TTTR mode.
    /// 
    /// ## Arguments
    /// 
    /// * `holdoff_time` - The holdoff time to set in nanoseconds. Must be between 0 and 25500 ns
    /// (25.5 microseconds)
    fn set_marker_holdoff_time(&mut self, holdoff_time : i32) -> Result<(), PatinaError<i32>> {
        if holdoff_time < 0 || holdoff_time > mhconsts::HOLDOFFMAX {
            return Err(PatinaError::ArgumentError(
                "holdoff_time".to_string(),
                holdoff_time,
                format!("Holdoff time must be between {} and {}", 0, mhconsts::HOLDOFFMAX))
            );
        }
        let mh_result = unsafe { MH_SetMarkerHoldoffTime(self.index, holdoff_time) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// The setting is useful when data rates are very low, so that the sync signals
    /// are far more common than photons (i.e. << 1 photon per 1000 pulses) and overflows
    /// happen regularly long before a useful amount of data arrives. The hardware will
    /// only transfer data to the FiFo when the holdtime has elapsed. By default, this is
    /// set to 2 ms (v 3.1 and later) or 0 (v 3.0 and earlier). If the hold time is too large,
    /// this may cause the data flow to "stutter", because it takes many milliseconds before the
    /// FiFo is read out. Overflow compression can be switched off by setting holdtime to 0.
    /// 
    /// New in v3.1
    /// 
    /// ## Arguments
    /// 
    /// * `hold_time` - The hold time to set in milliseconds. Must be between 0 and 255 ms.
    fn set_overflow_compression(&mut self, hold_time : i32) -> Result<(), PatinaError<i32>> {
        if hold_time < mhconsts::HOLDTIMEMIN || hold_time > mhconsts::HOLDTIMEMAX {
            return Err(PatinaError::ArgumentError(
                "hold_time".to_string(),
                hold_time,
                format!("Hold time must be between {} and {}",mhconsts::HOLDTIMEMIN, mhconsts::HOLDTIMEMAX))
            );
        }
        let mh_result = unsafe { MH_SetOflCompression(self.index, hold_time) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
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