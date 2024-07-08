//! Code for interfacing with a MultiHarp 150

use core::time;
use std::ffi::*;
use crate::error::{MultiHarpError, PatinaError, mh_to_result};
use crate::{mhconsts, TriggerEdge};
use crate::mhlib::*;
use crate::{available_devices, MHDeviceIterator};

/// A trait for MultiHarp devices -- must implement
/// all of the below methods.
pub trait MultiHarpDevice : Sized {
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

    fn clear_histogram(&mut self) -> Result<(), MultiHarpError> {
        Ok(())
    }

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

    fn fill_histogram(&self, histogram : &mut Vec<u32>, channel : i32) -> Result<(), PatinaError<i32>> {
        Ok(())
    }

    fn get_histogram_by_copy(&self, channel : i32) -> Result<Vec<u32>, PatinaError<i32>> {
        Ok(vec![0; 65536])
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
        if mh_result != 0 {
            return Err(PatinaError::from(MultiHarpError::from(mh_result)));
        }
        let mut num_channels = 0i32;
        let channels_result = unsafe{ MH_GetNumOfInputChannels(index, &mut num_channels) };

        if channels_result != 0 {

            return Err(PatinaError::from(MultiHarpError::from(channels_result)));
        }

        Ok(MultiHarp150 {
            index,
            serial: unsafe { CString::from_raw(serial.as_mut_ptr()) }.to_str().unwrap().to_string(),
            initialized: false,
            num_channels,
        })
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


    //////// SETTERS //////////////

    /// Sets the divider of the sync signal, should be used to keep the
    /// effective sync rate below 78 MHz. The larger the divider, the greater
    /// the jitter in estimated timing of the sync signals. The output of
    /// `get_count_rate` is internally corrected for the sync divider, and should
    /// not be adjusted by this value.
    /// 
    /// # Arguments
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
    /// # Arguments
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
    /// # Arguments
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

    /// Sets the dead time of the sync signal. This function is used to suppress
    /// afterpulsing artifacts in some detectors. The dead time is in picoseconds
    /// 
    /// # Arguments
    /// 
    /// * `on` - Whether to turn the dead time on or off. 0 is off, 1 is on.
    /// 
    /// * `deadtime` - The dead time to set in picoseconds.
    fn set_sync_dead_time(&mut self, on : bool, deadtime : i32) -> Result<(), PatinaError<i32>> {
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
    /// # Arguments
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
    /// # Arguments
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

    /// Set the dead time of the input channel. Used to suppress afterpulsing artifacts
    /// in some detectors. The dead time is in picoseconds.
    /// 
    /// # Arguments
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
    /// with long pulse shape artifacts.
    /// 
    /// # Arguments
    /// 
    /// * `hystcode` - The hysteresis code to set. Must be 0 (for 3 mV) or 1 (for 35 mV).
    fn set_input_hysteresis(&mut self, hystcode : bool) -> Result<(), PatinaError<i32>> {
        let mh_result = unsafe { MH_SetInputHysteresis(self.index, hystcode as i32) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Determines if a measurement will stop when the histogram overflows.
    /// 
    /// # Arguments
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
    /// # Arguments
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
    /// ## See also
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
    /// # Arguments
    /// 
    /// * `lencode` - The length code to set. Must be between 0 and 6. Actual length
    /// will be 1024*(2^lencode).
    /// 
    /// # Returns
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
    /// # Arguments
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
    /// # Arguments
    /// 
    /// * `period` - The period to set in units of 100 ns.
    fn set_trigger_output(&mut self, period : i32) -> Result<(), PatinaError<i32>>{
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
    /// # Arguments
    /// 
    /// * `acquisition_time` - The acquisition time to set in milliseconds. Must be between 1 and 3600000 ms = 10 hours.
    /// 
    /// ## See also
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
    /// # Arguments
    /// 
    /// * `channel` - The channel to get the histogram for. Must be an available channel for the device.
    /// 
    /// # Returns
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

    /// Fills an existing buffer with the arrival time histogram from the device.
    /// TODO check if the buffer is the right size.
    /// 
    /// # Arguments
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