//! Code for interfacing with a MultiHarp 150

use std::ffi::*;
use std::str::FromStr;
#[cfg(feature = "async")]
use async_trait::async_trait;
#[cfg(feature = "async")]
use crate::error::AsyncCheckedResult;

use crate::error::{MultiHarpError, PatinaError, mh_to_result, CheckedResult, MultiHarpResult};
use crate::{mhconsts, TriggerEdge, WRMode, ROWIDXMAX, ROWIDXMIN};
use crate::mhlib::*;
use crate::MultiHarpConfig;
use crate::{available_devices, MHDeviceIterator};


#[allow(dead_code)]
#[inline]
/// Sync rollover or marker bit
pub fn photon_special(photon : u32) -> bool {
    (photon & mhconsts::SPECIAL) != 0
}

#[allow(dead_code)]
#[inline]
/// six highest bits other than the overflow bit
pub fn photon_to_channel(photon : u32) -> u8 {
    (photon & mhconsts::CHANNEL) as u8
}

#[allow(dead_code)]
#[inline]
/// 7th to 32nd bit from high bits, 25 bit output
pub fn photon_to_arrival_t2(photon : u32) -> u32 {
    (photon & mhconsts::HISTOTAG_T2) as u32
}

#[allow(dead_code)]
#[inline]
/// 7th to 7+15 = 22nd bit from high bits, 15 bit output
pub fn photon_to_arrival_t3(photon : u32) -> u16 {
    (photon & mhconsts::HISTOTAG_T3) as u16
}


#[allow(dead_code)]
#[inline]
/// Last 10 bits
pub fn photon_to_sync_counter(photon : u32) -> u16 {
    (photon & mhconsts::SYNCTAG) as u16
}

/// A trait for MultiHarp devices -- must implement
/// all of the below methods.
#[allow(unused_variables)]
pub trait MultiHarpDevice : Sized {

    /// Calls many `set_` functions to set the device with
    /// Err val will contain a vector of strings reporting each configuration parameter that
    /// was set with an error, if any.
    fn set_from_config(&mut self, config : &MultiHarpConfig) -> Result<(), Vec<String>> {

        let mut err_vals = Vec::<String>::new();
        if let Some(sync_div) = config.sync_div {
            let _ = self.set_sync_div(sync_div)
            .map_err(|e| err_vals.push(format!("Error setting sync divider: {:?}", e)));
        }
        if let Some(sync_trigger_edge) = config.sync_trigger_edge {
            let _ = self.set_sync_edge_trigger(sync_trigger_edge.0, sync_trigger_edge.1)
            .map_err(|e| err_vals.push(format!("Error setting sync trigger: {:?}", e)));
        }

        if let Some(sync_offset) = config.sync_channel_offset {
            let _ = self.set_sync_channel_offset(sync_offset)
            .map_err(|e| err_vals.push(format!("Error setting sync channel offset: {:?}", e)));
        }

        #[cfg(feature = "MHLv3_1_0")]
        if let Some(sync_enable) = config.sync_channel_enable {
            self.set_sync_channel_enable(sync_enable)
            .map_err(|e| err_vals.push(format!("Error setting sync channel enable: {:?}", e)));
        }

        if let Some(sync_deadtime) = config.sync_dead_time {
            let _ = self.set_sync_dead_time(sync_deadtime.0, sync_deadtime.1)
            .map_err(|e| err_vals.push(format!("Error setting sync dead time: {:?}", e)));
        }

        if let Some(input_edges) = &config.input_edges {
            for (i, level, edge) in input_edges.iter() {
                let _ = self.set_input_edge_trigger(*i, *level, *edge)
                .map_err(|e| err_vals.push(format!("Error setting input edge trigger: {:?}", e)));
            }
        }

        if let Some(input_offsets) = &config.input_offsets {
            for (i, offset) in input_offsets.iter() {
                let _ = self.set_input_channel_offset(*i, *offset)
                .map_err(|e| err_vals.push(format!("Error setting input channel offset: {:?}", e)));
            }
        }

        if let Some(input_enable) = &config.input_enables {
            for (i, enable) in input_enable.iter() {
                let _ =self.set_input_channel_enable(*i, *enable)
                .map_err(|e| err_vals.push(format!("Error setting input channel enable: {:?}", e)));
            }
        }

        if let Some(input_deadtimes) = &config.input_dead_times {
            for (i, on, deadtime) in input_deadtimes.iter() {
                let _ = self.set_input_dead_time(*i, *on, *deadtime)
                .map_err(|e| err_vals.push(format!("Error setting input dead time: {:?}", e)));
            }
        }

        #[cfg(feature = "MHLv3_0_0")]
        if let Some(input_hysteresis) = config.input_hysteresis {
            let _ = self.set_input_hysteresis(input_hysteresis)
            .map_err(|e| err_vals.push(format!("Error setting input hysteresis: {:?}", e)));
        }

        if let Some(stop_overflow) = config.stop_overflow {
            let _ = self.set_stop_overflow(stop_overflow.0, stop_overflow.1)
            .map_err(|e| err_vals.push(format!("Error setting stop overflow: {:?}", e)));
        }

        if let Some(binning) = config.binning {
            let _ = self.set_binning(binning)
            .map_err(|e| err_vals.push(format!("Error setting binning: {:?}", e)));
        }

        if let Some(offset) = config.offset {
            let _ = self.set_offset(offset)
            .map_err(|e| err_vals.push(format!("Error setting offset: {:?}", e)));
        }

        if let Some(histo_len) = config.histo_len {
            let _ = self.set_histogram_len(histo_len)
            .map_err(|e| err_vals.push(format!("Error setting histogram length: {:?}", e)));
        }

        if let Some(meas_control) = config.meas_control {
            let _ = self.set_measurement_control_mode(meas_control.0, meas_control.1, meas_control.2)
            .map_err(|e| err_vals.push(format!("Error setting measurement control mode: {:?}", e)));
        }

        if let Some(trigger_output) = config.trigger_output {
            let _ = self.set_trigger_output(trigger_output)
            .map_err(|e| err_vals.push(format!("Error setting trigger output: {:?}", e)));
        }

        #[cfg(feature = "MHLv3_1_0")]
        if let Some(ofl_compression) = config.ofl_compression {
            let _ = self.set_overflow_compression(ofl_compression)
            .map_err(|e| err_vals.push(format!("Error setting overflow compression: {:?}", e)));
        }

        if let Some(marker_edges) = config.marker_edges {
            let _ = self.set_marker_edges(marker_edges[0], marker_edges[1], marker_edges[2], marker_edges[3])
            .map_err(|e| err_vals.push(format!("Error setting marker edges: {:?}", e)));
        }

        if let Some(marker_enable) = config.marker_enable {
            let _ = self.set_marker_enable(marker_enable[0], marker_enable[1], marker_enable[2], marker_enable[3])
            .map_err(|e| err_vals.push(format!("Error setting marker enable: {:?}", e)));
        }

        if let Some(marker_holdoff) = config.marker_holdoff {
            let _ = self.set_marker_holdoff_time(marker_holdoff)
            .map_err(|e| err_vals.push(format!("Error setting marker holdoff time: {:?}", e)));
        }
        
        if err_vals.len() > 0 { return Err(err_vals) }
        Ok(())
    }

    // Open a MultiHarp device by index.
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
    /// 
    /// - `PatinaError::MultiHarpError(DeviceBusy)` if the
    /// device is already in use.
    /// 
    /// - `PatinaError::MultiHarpError(DeviceOpenFail)` if the
    /// the MHLib call itself fails.
    /// 
    /// - `PatinaError::NoDeviceAvailable` if there are either
    /// no connected `MultiHarp` devices or no available multiple
    /// harp devices when `None` is passed as an argument.
    fn open(index : Option<i32>) -> CheckedResult<Self, i32>;

    /// Iterate over MultiHarp device indices until the provided serial number
    /// is found, then open that device.
    /// 
    /// ## Arguments
    /// 
    /// * `serial` - The serial number of the device to open.
    /// 
    /// ## Returns
    /// 
    /// A `Result` containing the opened MultiHarp device
    /// or an error.
    /// 
    /// ## Errors
    /// 
    /// - `PatinaError::ArgumentError` if the serial number is not 8 characters or less
    /// (leading zeros are trimmed _after_ comparing to length 8 but can
    /// be provided pretrimed, e.g. '00035321' and '35321' refer to the
    /// same device, but '000000000000035321' returns an error).
    /// 
    /// - All errors of `MultiHarp150::open`
    /// 
    /// ## See also
    /// 
    /// - `open` - Open a MultiHarp device by index.
    fn open_by_serial(serial : &str) -> CheckedResult<Self, i32>;

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
    fn init(&mut self, mode : mhconsts::MeasurementMode, reference_clock : mhconsts::ReferenceClock) -> MultiHarpResult<()>;

    /// Returns the model code of the MultiHarp device, its part number, and its version.
    /// 
    /// ## Returns
    /// 
    /// * `(Model, PartNumber, Version)`
    fn get_hardware_info(&self) -> MultiHarpResult<(String, String, String)>{
        Ok(("".to_string(), "".to_string(), "".to_string()))
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
    fn get_base_resolution(&self) -> MultiHarpResult<(f64, i32)>{
        Ok((5.0,0))
    }

    /// Returns the number of input channels in the device.
    fn num_input_channels(&self) -> MultiHarpResult<i32> { Ok(4) }

    /// Returns an informative error message by querying the MultiHarp.
    /// Should be called on a `MultiHarpError` to get more information.
    fn get_debug_info(&self) -> MultiHarpResult<String> { Ok ("No debug info".to_string()) }

    /// Sets the divider of the sync signal, should be used to keep the
    /// effective sync rate below 78 MHz. The larger the divider, the greater
    /// the jitter in estimated timing of the sync signals. The output of
    /// `get_count_rate` is internally corrected for the sync divider, and should
    /// not be adjusted by this value.
    /// 
    /// ## Arguments
    /// 
    /// * `sync_div` - The sync divider to set. Must be between 1 and 16. 
    fn set_sync_div(&mut self, sync_div : i32) -> CheckedResult<(), i32>{
        if sync_div < mhconsts::SYNCDIVMIN || sync_div > mhconsts::SYNCDIVMAX {
            return Err(PatinaError::ArgumentError(
                "sync_div".to_string(),
                sync_div,
                format!("Sync divider must be between {} and {}", mhconsts::SYNCDIVMIN, mhconsts::SYNCDIVMAX))
            );
        }
        Ok(())
    }

    /// Sets the level and edge of the sync signal to trigger on.
    /// 
    /// ## Arguments
    /// 
    /// * `level` - The level of the sync signal to trigger on (in millivolts). Must be between -1200 and 1200 mV.
    ///  (note, the hardware uses a 10 bit DAC, and so this is only set to within 2.34 mV)
    /// 
    /// * `edge` - The edge of the sync signal to trigger on.
    fn set_sync_edge_trigger(&mut self, level : i32, edge : mhconsts::TriggerEdge) -> CheckedResult<(), i32>{
        if level < mhconsts::TRGLVLMIN || level > mhconsts::TRGLVLMAX {
            return Err(PatinaError::ArgumentError(
                "level".to_string(),
                level,
                format!("Level must be between {} and {}", mhconsts::TRGLVLMIN, mhconsts::TRGLVLMAX))
            );
        }
        Ok(())
    }

    /// Sets the timing offset of the sync channel in picoseconds.
    /// 
    /// ## Arguments
    /// 
    /// * `offset` - The offset to set in picoseconds. Must be between -99999 and 99999 ps.
    fn set_sync_channel_offset(&mut self, offset : i32) -> CheckedResult<(), i32>{
        if offset < mhconsts::CHANNEL_OFFS_MIN || offset > mhconsts::CHANNEL_OFFS_MAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Channel offset must be between {} and {}", mhconsts::CHANNEL_OFFS_MIN, mhconsts::CHANNEL_OFFS_MAX))
            );
        }
        Ok(())
    }

    /// Enables or disables the sync channel. Only useful in T2 mode
    #[cfg(feature = "MHLv3_1_0")]
    fn set_sync_channel_enable(&mut self, enable : bool) -> CheckedResult<(), i32>{
        Ok(())
    }

    /// Sets the dead time of the sync signal. This function is used to suppress
    /// afterpulsing artifacts in some detectors. The dead time is in picoseconds
    /// 
    /// ## Arguments
    /// 
    /// * `on` - Whether to turn the dead time on or off. 0 is off, 1 is on.
    /// 
    /// * `deadtime` - The dead time to set in picoseconds.
    fn set_sync_dead_time(&mut self, on : bool, deadtime : i32) -> CheckedResult<(), i32>{
        if deadtime < mhconsts::EXTDEADMIN || deadtime > mhconsts::EXTDEADMAX {
            return Err(PatinaError::ArgumentError(
                "deadtime".to_string(),
                deadtime,
                format!("Dead time must be between {} and {}", mhconsts::EXTDEADMIN, mhconsts::EXTDEADMAX))
            );
        }
        Ok(())    
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
    fn set_input_edge_trigger(&mut self, channel : i32, level : i32, edge : mhconsts::TriggerEdge) -> CheckedResult<(), i32>{
        if level < mhconsts::TRGLVLMIN || level > mhconsts::TRGLVLMAX {
            return Err(PatinaError::ArgumentError(
                "level".to_string(),
                level,
                format!("Level must be between {} and {}", mhconsts::TRGLVLMIN, mhconsts::TRGLVLMAX))
            );
        }
        Ok(())
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
    fn set_input_channel_offset(&mut self, channel : i32, offset : i32) -> CheckedResult<(), i32>{
        if offset < mhconsts::CHANNEL_OFFS_MIN || offset > mhconsts::CHANNEL_OFFS_MAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Channel offset must be between {} and {}", mhconsts::CHANNEL_OFFS_MIN, mhconsts::CHANNEL_OFFS_MAX))
            );
        }
        Ok(())
    }

    /// Enables or disables the input channel.
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to set the enable for. Must be an available channel for the device.
    /// 
    /// * `enable` - Whether to enable the channel. 0 is off, 1 is on.
    fn set_input_channel_enable(&mut self, channel : i32, enable : bool) -> CheckedResult<(), i32>{
        Ok(())
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
    fn set_input_dead_time(&mut self, channel : i32, on : bool, deadtime : i32) -> CheckedResult<(), i32> {
        if deadtime < mhconsts::EXTDEADMIN || deadtime > mhconsts::EXTDEADMAX {
            return Err(PatinaError::ArgumentError(
                "deadtime".to_string(),
                deadtime,
                format!("Dead time must be between {} and {}", mhconsts::EXTDEADMIN, mhconsts::EXTDEADMAX))
            );
        }
        Ok(())
    }

    /// Used to accommodate hysteresis on the input and sync channels for detectors
    /// with long pulse shape artifacts. New in firmware version 3.0
    /// 
    /// ## Arguments
    /// 
    /// * `hystcode` - The hysteresis code to set. Must be 0 (for 3 mV) or 1 (for 35 mV).    
    #[cfg(feature = "MHLv3_0_0")]
    fn set_input_hysteresis(&mut self, hystcode : bool) -> CheckedResult<(), i32> {
        Ok(())
    }

    /// Determines if a measurement will stop when the histogram overflows.
    /// 
    /// ## Arguments
    /// 
    /// * `stop_overflow` - Whether to stop on overflow. 0 is off, 1 is on.
    /// 
    /// * `stopcount` - The number of counts to stop on. Must be between 1 and 4294967295.
    fn set_stop_overflow(&mut self, stop_overflow : bool, stopcount : u32) -> CheckedResult<(), u32> {
        if stopcount < mhconsts::STOPCNTMIN {
            return Err(PatinaError::ArgumentError(
                "stopcount".to_string(),
                stopcount,
                format!("Stop count must be between {} and {}", mhconsts::STOPCNTMIN, mhconsts::STOPCNTMAX))
            );
        }

        Ok(())
    }

    /// Only applies in Histogramming or T3 mode. The binning corresponds to repeated
    /// doubling, i.e. 0 is no binning, 1 is 2x binning, 2 is 4x binning, etc.
    /// 
    /// ## Arguments
    /// 
    /// * `binning` - The binning to set. Must be between 0 and 24 (corresponding to
    /// pooling 2^0 to 2^24 bins).
    fn set_binning(&mut self, binning : i32) -> CheckedResult<(), i32> {
        if binning < 0 || binning > mhconsts::BINSTEPSMAX {
            return Err(PatinaError::ArgumentError(
                "binning".to_string(),
                binning,
                format!("Binning must be between 0 and {}", mhconsts::BINSTEPSMAX))
            );
        }
        Ok(())
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
    fn set_offset(&mut self, offset : i32) -> CheckedResult<(), i32> {
        if offset < mhconsts::OFFSETMIN || offset > mhconsts::OFFSETMAX {
            return Err(PatinaError::ArgumentError(
                "offset".to_string(),
                offset,
                format!("Offset must be between {} and {}", mhconsts::OFFSETMIN, mhconsts::OFFSETMAX))
            );
        }
        Ok(())
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
    /// * `CheckedResult<i32, i32>` - The actual length of the histogram.
    fn set_histogram_len(&mut self, lencode : i32) -> CheckedResult<i32, i32> {
        if lencode < mhconsts::MINLENCODE || lencode > mhconsts::MAXLENCODE {
            return Err(PatinaError::ArgumentError(
                "lencode".to_string(),
                lencode,
                format!("Length code must be between {} and {}", mhconsts::MINLENCODE, mhconsts::MAXLENCODE))
            );
        }
        Ok(65536)
    }

    /// Clears the histogram of the device. Does nothing if in T2 or T3 mode
    fn clear_histogram(&mut self) -> MultiHarpResult<()> {Ok(())}

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
    ) -> Result<(), PatinaError::<String>>{
        Err(PatinaError::NotImplemented)
    }

    /// Sets the period of the programmable trigger output. Setting the
    /// period to 0 switches it off. The period is set in units of 100 nanoseconds.
    /// 
    /// ## Arguments
    /// 
    /// * `period` - The period to set in units of 100 ns.
    fn set_trigger_output(&mut self, period : i32) -> CheckedResult<(), i32>{
        if period < mhconsts::TRIGOUTMIN || period > mhconsts::TRIGOUTMAX {
            return Err(PatinaError::ArgumentError(
                "period".to_string(),
                period,
                format!("Period must be between {} and {}", mhconsts::TRIGOUTMIN, mhconsts::TRIGOUTMAX))
            );
        }
        Ok(())
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
    fn start_measurement(&mut self, acquisition_time : i32) -> CheckedResult<(), i32>;

    /// Stops the current measurement. Must be called after `start_measurement`, even
    /// if it expires due to the `acquisition_time` parameter.
    fn stop_measurement(&mut self) -> MultiHarpResult<()>;

    /// Reports whether there is an ongoing measurement.
    /// 
    /// ## Returns
    /// 
    /// * `bool` - Whether there is an ongoing measurement.
    /// True if measurement is ongoing, false if not.  
    fn ctc_status(&self) -> MultiHarpResult<bool>;

    /// Fills an existing buffer with the arrival time histogram from the device.
    /// TODO check if the buffer is the right size.
    /// 
    /// ## Arguments
    /// 
    /// * `histogram` - The buffer to fill with the histogram. Must be at least as long
    /// as the setting's histogram length. TODO check this arg!
    /// 
    /// * `channel` - The channel to get the histogram for. Must be an available channel for the device.
    fn fill_histogram<'a, 'b>(&'a mut self, histogram : &'b mut [u32], channel : i32) -> CheckedResult<(), i32> {Ok(())}

    /// Populates an existing buffer with all histograms from the device. Expects
    /// a buffer for all channels, so the buffer must be at least `num_channels * histogram_length`
    /// long. TODO: actually provide checking!
    /// 
    /// ## Arguments
    /// 
    /// * `histograms` - The buffer to fill with all histograms. Must be at least as long
    /// as the setting's histogram length times the number of channels. TODO check this arg!
    fn fill_all_histograms<'a, 'b>(&'a mut self, histograms : &'b mut [u32]) -> MultiHarpResult<()> {Ok(())}

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
    fn get_histogram_by_copy(&mut self, channel : i32) -> CheckedResult<Vec<u32>, i32> {Ok(vec![0; 65536])}
    
    /// Returns all histograms from the device. This makes a copy, rather
    /// than filling an existing buffer.
    fn get_all_histograms_by_copy(&mut self) -> MultiHarpResult<Vec<u32>> {Ok(vec![0; 65536 * 4])}

    /// Returns the resolution of the bins in the histogram in picoseconds. Not meaningful
    /// in T2 mode.
    fn get_resolution(&self) -> MultiHarpResult<f64> {Ok(5.0)}

    /// Returns the sync rate in Hz. Requires at least 100 ms of data to be collected
    fn get_sync_rate(&self) -> MultiHarpResult<i32> {Ok(78e6 as i32)}

    /// Returns the sync period in seconds. Resolution is the
    /// same as the device's resolution. Accuracy is determined by
    /// single shot jitter and clock stability.
    fn get_sync_period(&self) -> MultiHarpResult<f64> {Ok(1.0 / 78e6)}

    /// Returns the count rate of the specified channel in photons per second
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to get the count rate for. Must be an available channel for the device.
    fn get_count_rate(&self, channel : i32) -> CheckedResult<i32, i32> {Ok(1e5 as i32)}

    /// Returns the count rates of all channels in photons per second and the sync rate
    /// in Hz.
    fn get_all_count_rates(&self) -> MultiHarpResult<(i32, Vec<i32>)> {Ok((78e6 as i32, vec![1e5 as i32; 4]))}

    /// Returns the set flags of the device, interpretable using
    /// the bitmasks in `mhconsts`.
    /// 
    /// ### See also
    /// 
    /// - `get_warnings` - To get the warning flags.
    fn get_flags(&self) -> MultiHarpResult<i32> {Ok(0)}

    /// Returns the set warnings of the device, interpretable using
    /// the bitmasks in `mhconsts`. Prior to this call, you must call
    /// `get_all_count_rates` or `get_sync_rate` and `get_count_rate` for
    /// all channels, otherwise at least some warnings will not be meaningful.
    /// 
    /// ### See also
    /// 
    /// - `get_flags`
    /// - `get_warnings_text`
    fn get_warnings(&self) -> MultiHarpResult<i32> {Ok(0)}


    /// Returns a human-readable string to interpret the device warnings
    /// 
    /// ### See also
    /// - `get_warnings`
    /// - `get_flags`
    fn get_warnings_text(&self) -> MultiHarpResult<String> {Ok("No warnings".to_string())}

    /// Returns the elapsed measurement time in milliseconds. When
    /// using the `SwStartSwStop` mode, these results will be less accurate.
    fn get_elapsed_measurement_time(&self) -> MultiHarpResult<f64> {Ok(0.0)}

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
    fn get_start_time(&self) -> MultiHarpResult<(u32, u32, u32)> {Ok((0, 0, 0))}

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
    /// * `CheckedResult<i32, u32>` - The actual number of counts read. Data
    /// after this value is undefined.
    fn read_fifo<'a, 'b>(&'a self, buffer : &'b mut [u32]) -> CheckedResult<i32, u32> {
        Ok(0)
    }

    /// Sets the detection edges for each of the four marker channels (set simultaneously). Only
    /// meaningful in TTTR mode.
    fn set_marker_edges(&mut self, me1 : TriggerEdge, me2 : TriggerEdge, me3 : TriggerEdge, me4 : TriggerEdge) -> MultiHarpResult<()> {Ok(())}

    /// Used to enable or disable individual TTL marker inputs. Only meaningful in TTTR mode.
    fn set_marker_enable(&mut self, en1 : bool, en2 : bool, en3 : bool, en4 : bool) -> MultiHarpResult<()> {Ok(())}

    /// Sets the holdoff time for the markers in nanoseconds. This is not normally required,
    /// but it can be useful to deal with marker line issues. The holdoff time sets the
    /// minimum time between markers. Only meaningful in TTTR mode.
    /// 
    /// ## Arguments
    /// 
    /// * `holdoff_time` - The holdoff time to set in nanoseconds. Must be between 0 and 25500 ns
    /// (25.5 microseconds)
    fn set_marker_holdoff_time(&mut self, holdofftime : i32) -> CheckedResult<(), i32> {
        if holdofftime < mhconsts::HOLDOFFMIN || holdofftime > mhconsts::HOLDOFFMAX {
            return Err(PatinaError::ArgumentError(
                "holdofftime".to_string(),
                holdofftime,
                format!("Holdoff time must be between {} and {}", mhconsts::HOLDOFFMIN, mhconsts::HOLDOFFMAX))
            );
        }
        Ok(())
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
    fn set_overflow_compression(&mut self, holdtime : i32) -> CheckedResult<(), i32> {
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

#[cfg(feature = "async")]
#[async_trait]
pub trait AsyncMultiHarpDevice {
    // Loads from the FIFO buffer asynchronously.
    // async fn read_fifo_async<'a, 'b>(&'a self, buffer : &'b mut Vec<u32>) -> AsyncCheckedResult<i32, u32>;
}

/// A more object-oriented way to
/// interface with the MultiHarp. A new MultiHarp150
/// is created with the `open` method.
/// 
/// Each method calls the corresponding `MHLib` function
/// with the device index of that `MultiHarp` instance.
/// 
/// Successful creation of a `MultiHarp` instance guarantees
/// that the device has been opened, and the device is
/// closed when the instance is dropped.
/// 
/// The MultiHarp does _not_ implement Copy or Clone. This
/// prevents multiple simultaneous attempts to access a MultiHarp
/// from within a thread. When using across threads, be careful
/// to guard the MultiHarp with a Mutex or other synchronization
/// primitive.
#[cfg(feature = "MHLib")]
pub struct MultiHarp150 {
    index : i32,
    serial : String,
    initialized : bool,
    num_channels : i32,
    features : i32, // marks which features are available on this device.
}

#[cfg(feature = "MHLib")]
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
    /// 
    /// - `PatinaError::MultiHarpError(DeviceBusy)` if the
    /// device is already in use.
    /// 
    /// - `PatinaError::MultiHarpError(DeviceOpenFail)` if the
    /// the MHLib call itself fails.
    /// 
    /// - `PatinaError::NoDeviceAvailable` if there are either
    /// no connected `MultiHarp` devices or no available multiple
    /// harp devices when `None` is passed as an argument.
    fn open(index : Option<i32>) -> CheckedResult<Self, i32> {
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

        let init_result = unsafe { MH_Initialize(index, mhconsts::MeasurementMode::T3 as i32, mhconsts::ReferenceClock::Internal as i32) };
        if init_result != 0 {
            return Err(PatinaError::from(MultiHarpError::from(init_result)));
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
                serial: unsafe { CStr::from_ptr(serial.as_mut_ptr()) }.to_str().unwrap().to_string(),
                initialized: false,
                num_channels,
                features,
            }
        )
    }

    /// Iterate over MultiHarp device indices until the provided serial number
    /// is found, then open that device.
    /// 
    /// ## Arguments
    /// 
    /// * `serial` - The serial number of the device to open.
    /// 
    /// ## Returns
    /// 
    /// A `Result` containing the opened MultiHarp device
    /// or an error.
    /// 
    /// ## Errors
    /// 
    /// - `PatinaError::ArgumentError` if the serial number is not 8 characters or less
    /// (leading zeros are trimmed _after_ comparing to length 8 but can
    /// be provided pretrimed, e.g. '00035321' and '35321' refer to the
    /// same device, but '000000000000035321' returns an error).
    /// 
    /// - All errors of `MultiHarp150::open`
    /// 
    /// ## See also
    /// 
    /// - `open` - Open a MultiHarp device by index.
    fn open_by_serial(serial : &str) -> CheckedResult<Self, i32> {
        if serial.len() > 8 {
            return Err(PatinaError::ArgumentError(
                "serial".to_string(),
                serial.len() as i32,
                "Serial number must be 8 characters or less".to_string())
            );
        }

        // Trim leading zeros in serial number
        let serial = serial.trim_start_matches('0');

        // Trim leading zeros in serial number
        let serial = serial.trim_start_matches('0');

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
    fn init(&mut self, mode : mhconsts::MeasurementMode, reference_clock : mhconsts::ReferenceClock) -> MultiHarpResult<()> {
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
    fn get_hardware_info(&self) -> MultiHarpResult<(String, String, String)> {
        let mut model_code = [0 as c_char; 24];
        let mut part_number = [0 as c_char; 8];
        let mut version = [0 as c_char; 8];

        mh_to_result!(
            unsafe { MH_GetHardwareInfo(self.index, model_code.as_mut_ptr(), part_number.as_mut_ptr(), version.as_mut_ptr()) },
            (
                unsafe { CStr::from_ptr(model_code.as_mut_ptr()) }.to_str().unwrap().to_string(),
                unsafe { CStr::from_ptr(part_number.as_mut_ptr()) }.to_str().unwrap().to_string(),
                unsafe { CStr::from_ptr(version.as_mut_ptr()) }.to_str().unwrap().to_string()
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
    fn get_base_resolution(&self) -> MultiHarpResult<(f64, i32)> {
        let mut base_resolution: f64 = 0.0;
        let mut bin_steps = 0;
        mh_to_result!(
            unsafe { MH_GetBaseResolution(self.index, &mut base_resolution, &mut bin_steps) },
            (base_resolution, bin_steps)
        )
    }

    /// Returns the number of input channels in the device.
    fn num_input_channels(&self) -> MultiHarpResult<i32> {
        let mut num_channels = 0;
        mh_to_result!(
            unsafe { MH_GetNumOfInputChannels(self.index, &mut num_channels) },
            num_channels
        )
    }

    /// Returns an informative error message by querying the MultiHarp.
    /// Should be called on a `MultiHarpError` to get more information.
    fn get_debug_info(&self) -> MultiHarpResult<String> {
        let debug_string = [0 as c_char; mhconsts::DEBUGSTRLEN];
        let mh_result = unsafe { MH_GetErrorString(debug_string.as_ptr() as *mut c_char, self.index) };
        mh_to_result!(
            mh_result,
            unsafe { CStr::from_ptr(debug_string.as_ptr() as *mut c_char) }.to_str().unwrap().to_string()
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
    fn set_sync_div(&mut self, sync_div : i32) -> CheckedResult<(), i32> {
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
    fn set_sync_edge_trigger(&mut self, level : i32, edge : mhconsts::TriggerEdge) -> CheckedResult<(), i32> {
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
    fn set_sync_channel_offset(&mut self, offset : i32) -> CheckedResult<(), i32> {
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
    #[cfg(feature = "MHLv3_1_0")]
    fn set_sync_channel_enable(&mut self, enable : bool) -> CheckedResult<(), i32> {
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
    fn set_sync_dead_time(&mut self, on : bool, deadtime : i32) -> CheckedResult<(), i32> {
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
    fn set_input_edge_trigger(&mut self, channel : i32, level : i32, edge : mhconsts::TriggerEdge) -> CheckedResult<(), i32> {
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
    fn set_input_channel_offset(&mut self, channel : i32, offset : i32) -> CheckedResult<(), i32> {
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
    fn set_input_channel_enable(&mut self, channel : i32, enable : bool) -> CheckedResult<(), i32> {
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
    fn set_input_dead_time(&mut self, channel : i32, on : bool, deadtime : i32) -> CheckedResult<(), i32> {
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
    #[cfg(feature = "MHLv3_0_0")]
    fn set_input_hysteresis(&mut self, hystcode : bool) -> CheckedResult<(), i32> {
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
    fn set_stop_overflow(&mut self, stop_overflow : bool, stopcount : u32) -> CheckedResult<(), u32> {

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
    fn set_binning(&mut self, binning : i32) -> CheckedResult<(), i32> {
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
    fn set_offset(&mut self, offset : i32) -> CheckedResult<(), i32> {
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
    /// * `CheckedResult<i32, i32>` - The actual length of the histogram.
    fn set_histogram_len(&mut self, lencode : i32) -> CheckedResult<i32, i32> {
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
    fn clear_histogram(&mut self) -> MultiHarpResult<()> {
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
    ) -> CheckedResult<(), String> {

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
            // #[cfg(feature = "MHLv_3_1_0")]
            // mhconsts::MeasurementControlMode::SwStartSwStop => {
            //     let mh_result = unsafe { MH_SetMeasControl(self.index, mode as c_int, 0, 0) };
            //     return mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
            // }
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
    fn set_trigger_output(&mut self, period : i32) -> CheckedResult<(), i32>{
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
    fn start_measurement(&mut self, acquisition_time : i32) -> CheckedResult<(), i32> {
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
    fn stop_measurement(&mut self) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_StopMeas(self.index) };
        mh_to_result!(mh_result, ())
    }

    /// Reports whether there is an ongoing measurement.
    /// 
    /// ## Returns
    /// 
    /// * `bool` - Whether there is an ongoing measurement.
    /// True if measurement is ongoing, false if not.
    fn ctc_status(&self) -> Result<bool, MultiHarpError> {
        let mut ctc_status = 0;
        let mh_result = unsafe { MH_CTCStatus(self.index, &mut ctc_status) };
        mh_to_result!(mh_result, ctc_status == 0)
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
    fn get_histogram_by_copy(&mut self, channel : i32) -> Result<Vec<u32>, PatinaError<i32>> {
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
    fn get_all_histograms_by_copy(&mut self) -> MultiHarpResult<Vec<u32>> {
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
    fn fill_histogram<'a, 'b>(&'a mut self, histogram : &'b mut [u32], channel : i32) -> CheckedResult<(), i32> {
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
    // fn fill_all_histograms<'a, 'b>(&'a mut self, histograms : &'b mut [u32]) -> MultiHarpResult<()> {
    fn fill_all_histograms<'a, 'b>(&'a mut self, histograms : &'b mut [u32] ) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_GetAllHistograms(self.index, histograms.as_mut_ptr()) };
        mh_to_result!(mh_result, ())
    }

    /// Returns the resolution of the bins in the histogram in picoseconds. Not meaningful
    /// in T2 mode.
    fn get_resolution(&self) -> MultiHarpResult<f64> {
        let mut resolution = 0.0;
        let mh_result = unsafe { MH_GetResolution(self.index, &mut resolution) };
        mh_to_result!(mh_result, resolution)
    }

    /// Returns the sync rate in Hz. Requires at least 100 ms of data to be collected
    fn get_sync_rate(&self) -> MultiHarpResult<i32> {
        let mut sync_rate = 0;
        let mh_result = unsafe { MH_GetSyncRate(self.index, &mut sync_rate) };
        mh_to_result!(mh_result, sync_rate)
    }

    /// Returns the count rate of the specified channel in photons per second
    /// 
    /// ## Arguments
    /// 
    /// * `channel` - The channel to get the count rate for. Must be an available channel for the device.
    fn get_count_rate(&self, channel : i32) -> CheckedResult<i32, i32> {
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
    fn get_all_count_rates(&self) -> MultiHarpResult<(i32, Vec<i32>)> {
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
    fn get_flags(&self) -> MultiHarpResult<i32> {
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
    fn get_warnings(&self) -> MultiHarpResult<i32> {
        let mut warnings = 0;
        let mh_result = unsafe { MH_GetWarnings(self.index, &mut warnings) };
        mh_to_result!(mh_result, warnings)
    }

    /// Returns a human-readable string to interpret the device warnings
    /// 
    /// ### See also
    /// - `get_warnings`
    /// - `get_flags`
    fn get_warnings_text(&self) -> MultiHarpResult<String> {
        let warnings = self.get_warnings()?;
        let mut warnings_text = [0 as c_char; mhconsts::WARNLEN];
        let mh_result = unsafe { MH_GetWarningsText(self.index, warnings_text.as_mut_ptr(), warnings) };
        mh_to_result!(mh_result, unsafe { CStr::from_ptr(warnings_text.as_mut_ptr()) }.to_str().unwrap().to_string())
    }

    /// Returns the sync period in seconds. Resolution is the
    /// same as the device's resolution. Accuracy is determined by
    /// single shot jitter and clock stability.
    fn get_sync_period(&self) -> MultiHarpResult<f64> {
        let mut sync_period = 0.0;
        let mh_result = unsafe { MH_GetSyncPeriod(self.index, &mut sync_period) };
        mh_to_result!(mh_result, sync_period)
    }

    /// Returns the elapsed measurement time in milliseconds. When
    /// using the `SwStartSwStop` mode, these results will be less accurate.
    fn get_elapsed_measurement_time(&self) -> MultiHarpResult<f64> {
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
    fn get_start_time(&self) -> MultiHarpResult<(u32, u32, u32)> {
        let (mut dword2, mut dword1, mut dword0) = (0u32, 0u32, 0u32);
        let mh_result = unsafe { MH_GetStartTime(self.index, &mut dword2, &mut dword1, &mut dword0) };
        mh_to_result!(mh_result, (dword2, dword1, dword0))
    }

    /// Loads a buffer with the arrival time data from the device. Returns the actual
    /// number of counts read. Only meaningful in TTTR mode. Note: this call actually
    /// runs _faster_ when the read count is high, counterintuitively! The manual seems
    /// to suggest otherwise, but in my testing it will crank up the read speed as the
    /// photon counts increase.
    /// 
    /// ## Arguments
    /// 
    /// * `buffer` - The buffer to fill with the arrival time data. Must be at least
    /// `TTREADMAX` long.
    /// 
    /// ## Returns
    /// 
    /// * `CheckedResult<i32, u32>` - The actual number of counts read. Data
    /// after this value is undefined.
    fn read_fifo<'a, 'b>(&'a self, buffer : &'b mut [u32]) -> CheckedResult<i32, u32> {
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
    fn set_marker_edges(&mut self, marker1 : TriggerEdge, marker2 : TriggerEdge, marker3 : TriggerEdge, marker4 : TriggerEdge) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_SetMarkerEdges(self.index, marker1 as c_int, marker2 as c_int, marker3 as c_int, marker4 as c_int) };
        mh_to_result!(mh_result, ())
    }

    /// Used to enable or disable individual TTL marker inputs. Only meaningful in TTTR mode.
    fn set_marker_enable(&mut self, enable1 : bool, enable2 : bool, enable3: bool, enable4 : bool) -> MultiHarpResult<()> {
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
    fn set_marker_holdoff_time(&mut self, holdoff_time : i32) -> CheckedResult<(), i32> {
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
    #[cfg(feature = "v3_1")]
    fn set_overflow_compression(&mut self, hold_time : i32) -> CheckedResult<(), i32> {
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

/// Event filtering functionality
#[cfg(feature = "MHLib_v3_1_0")]
#[allow(dead_code)]
impl MultiHarp150 {
    /// This sets the parameters for one Row Filter implemented
    /// in the local FPGA processing that row of input channels.
    /// Each Row Filter can act only on the input channels within
    /// its own row and never on the sync channel. The value
    /// timerange de- termines the time window the filter is
    /// acting on. The parameter matchcnt specifies how many
    /// other events must fall into the chosen time window for
    /// the filter condition to act on the event at hand. The
    /// parameter inverse inverts the filter action, i.e. when
    /// the filter would regularly have eliminated an event it
    /// will then keep it and vice versa. For the typical case,
    /// let it be not inverted. Then, if matchcnt is 1 we will
    /// obtain a simple ‘singles filter’. This is the most
    /// straightforward and most useful filter in typical quantum
    /// optics experiments. It will suppress all events that do
    /// not have at least one coincident event within the chosen
    /// time range, be this in the same or any other channel
    /// marked as ‘use’ in this row. The bitfield passchannels
    /// is used to indicate if a channel is to be passed through
    /// the filter unconditionally, whether it is marked as ‘use’
    /// or not. The events on a channel that is marked neither as
    /// ‘use’ nor as ‘pass’ will not pass the filter, provided
    /// the filter is enabled. The parameter settings are
    /// irrelevant as long as the filter is not enabled.
    /// The output from the Row Filters is fed to the Main Filter.
    /// The overall filtering result depends on their combined
    /// action. Only the Main Filter can act on all channels of
    /// the MutiHarp device includ - ing the sync channel. It is
    /// usually sufficient and easier to use the Main Filter alone.
    /// 
    /// The only reasons for using the Row Filter(s) are early data
    /// reduction, so as to not overload the Main Filter, and the
    /// possible need for more complex filters, e.g. with different
    /// time ranges.
    /// 
    /// ## Arguments
    /// 
    /// * `row` - The row to set the filter for. Must be between 0 and 8.
    /// 
    /// * `time_range` - Time distance in picoseconds to other events
    /// to meet filter condition
    /// 
    /// * `match_cnt` - Number of other events to meet filter condition
    /// 
    /// * `inverse` - Whether to invert the filter action. 0 is normal,
    /// 1 is inverse filter
    /// 
    /// * `use_channels` - Bitfield of channels to use in the filter, with
    /// bit 7 as the rightmost input channel and bit 0 as the leftmost channel.
    /// Setting a bit to high means to use the channel in the filter.
    /// 
    /// * `pass_channels` - Bitfield of channels to pass through the
    /// filter unconditionally. If a bit is high, it is passed unconditionally.
    fn set_row_event_filter(
        &self, row : i32, time_range : i32,
        match_cnt : i32, inverse : bool, use_channels : i32,
        pass_channels : i32,
    ) -> CheckedResult<(), i32>{
        if (row < ROWIDXMIN || row > ROWIDXMAX) {
            return Err(PatinaError::ArgumentError(
                "row".to_string(),
                row,
                format!("Row must be between {} and {}", ROWIDXMIN, ROWIDXMAX))
            );
        }

        if (time_range < TIME_RANGEMIN || time_range > TIME_RANGEMAX) {
            return Err(PatinaError::ArgumentError(
                "time_range".to_string(),
                time_range,
                format!("Time range must be between {} and {}", TIME_RANGEMIN, TIME_RANGEMAX))
            );
        }

        if (match_cnt < MATCHCNTMIN || match_cnt > MATCHCNTMAX) {
            return Err(PatinaError::ArgumentError(
                "match_cnt".to_string(),
                match_cnt,
                format!("Match count must be between {} and {}", MATCHCNTMIN, MATCHCNTMAX))
            );
        }

        let mh_result = unsafe { MH_SetRowFilter(
            self.index, row, time_range, match_cnt, inverse as i32, use_channels, pass_channels
        ) };

        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// When the filter is disabled, all events are passed.
    fn enable_row_event_filter(&self, row : i32, enable : bool) -> CheckedResult<(), i32> {
        if (row < ROWIDXMIN || row > ROWIDXMAX) {
            return Err(PatinaError::ArgumentError(
                "row".to_string(),
                row,
                format!("Row must be between {} and {}", ROWIDXMIN, ROWIDXMAX))
            );
        }

        let mh_result = unsafe { MH_EnableRowFilter(self.index, row, enable as i32) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// This sets the parameters for the Main Filter implemented in the
    /// main FPGA processing the aggregated events arriving from the row FPGAs.
    /// The Main Filter can therefore act on all channels of the MutiHarp device
    /// including the sync channel. The value timerange determines the time
    /// window the filter is acting on. The parameter matchcnt specifies how
    /// many other events must fall into the chosen time window for the filter
    /// condition to act on the event at hand. The parameter inverse inverts the
    /// filter action, i.e. when the filter would regularly have eliminated an
    /// event it will then keep it and vice versa. For the typical case, let it
    /// be not inverted. Then, if matchcnt is 1 we obtain a simple
    /// ‘singles filter’. This is the most straight forward and most useful 
    /// filter in typical quantum optics experiments. It will suppress all
    /// events that do not have at least one coincid - ent event within the
    /// chosen time range, be this in the same or any other channel. In order
    /// to mark individual channel as ‘use’ and/or ‘pass’
    /// please use MH_SetMainEventFilterChannels.The parameter settings are
    /// irrelevant as long as the filter is not enabled. Note that the Main
    /// Filter only receives event data that passes the Row Filters (if they
    /// are enabled). The overall fil- tering result therefore depends on the
    /// combined action of both filters. It is usually sufficient and easier
    /// to use the Main Filter alone. The only reasons for using the Row
    /// Filters are early data reduction, so as to not overload the Main
    /// Filter, and the pos- sible need for more complex filters, e.g. with
    /// different time ranges.
    fn set_main_event_filter_params(&self, time_range : i32, match_cnt : i32, inverse : bool)
    -> CheckedResult<(), i32> {
        if (time_range < TIME_RANGEMIN || time_range > TIME_RANGEMAX) {
            return Err(PatinaError::ArgumentError(
                "time_range".to_string(),
                time_range,
                format!("Time range must be between {} and {}", TIME_RANGEMIN, TIME_RANGEMAX))
            );
        }

        if (match_cnt < MATCHCNTMIN || match_cnt > MATCHCNTMAX) {
            return Err(PatinaError::ArgumentError(
                "match_cnt".to_string(),
                match_cnt,
                format!("Match count must be between {} and {}", MATCHCNTMIN, MATCHCNTMAX))
            );
        }

        let mh_result = unsafe { MH_SetMainFilterParams(self.index, time_range, match_cnt, inverse as i32) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    fn set_main_event_filter_channels(&self, row : i32, use_channels : i32, pass_channels : i32)
    -> CheckedResult<(), i32> {
        if (row < ROWIDXMIN || row > ROWIDXMAX) {
            return Err(PatinaError::ArgumentError(
                "row".to_string(),
                row,
                format!("Row must be between {} and {}", ROWIDXMIN, ROWIDXMAX))
            );
        }

        let mh_result = unsafe { MH_SetMainFilterChannels(self.index, row, use_channels, pass_channels) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    fn enable_main_event_filter(&self, enable : bool) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_EnableMainFilter(self.index, enable as i32) };
        mh_to_result!(mh_result, ())
    }

    /// One important purpose of the event filters is to reduce USB load.
    /// When the input data rates are higher than the USB bandwith,
    /// there will at some point be a FiFo overrun. It may under such
    /// conditions be difficult to empirically optimize the filter settings.
    /// Setting filter test mode disables all data transfers into the FiFo
    /// so that a test measurement can be run without interruption by a
    /// FiFo overrun. The library routines MH_GetRowFilteredRates and
    /// MH_GetMainFilteredRates can then be used to monitor the count rates
    /// after the Row Filter and after the Main Filter. When the filtering
    /// effect is satisfactory the test mode can be switched off again to
    /// perform the regular measurement.
    /// 
    /// ## Arguments
    /// 
    /// * `test_mode` - Whether to enable or disable the filter test mode.
    /// If true, the filter test mode is enabled. If false, the filter test
    /// mode is disabled.
    fn set_filter_test_mode(&self, test_mode : bool) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_SetFilterTestMode(self.index, enable as i32) };
        mh_to_result!(mh_result, ())
    }

    ///This call retrieves the count rates after the Row Filters before
    /// entering the Main Filter. A measurement must be running to obtain
    /// valid results. Allow at least 100 ms to get a new reading. This is
    /// the gate time of the rate counters.
    /// 
    /// ## Returns
    /// 
    /// * `i32` - The sync rate after the filter
    /// * `Vec<i32>` - The count rates of all channels after the filter
    fn get_row_filtered_rates(&self) -> MultiHarpResult<(i32, Vec<i32>)> {
        let mut sync_rate : i32 = 0;
        let mut count_rates = vec![0i32; self.num_channels as usize];
        let mh_result = unsafe { MH_GetRowFilteredRates(self.index, &mut sync_rate, count_rates.as_mut_ptr()) };
        mh_to_result!(mh_result, (sync_rate, count_rates))
    }

    /// This call retrieves the count rates after the Main Filter. A measurement
    /// must be running to obtain valid results. Allow at least 100 ms to get a
    /// new reading. This is the gate time of the rate counters.
    /// 
    /// ## Returns
    /// 
    /// * `i32` - The sync rate after the filter
    /// * `Vec<i32>` - The count rates of all channels after the filter
    fn get_main_filtered_rates(&self) -> MultiHarpResult<(i32, Vec<i32>)> {
        let mut sync_rate : i32 = 0;
        let mut count_rates = vec![0i32; self.num_channels as usize];
        let mh_result = unsafe { MH_GetMainFilteredRates(self.index, &mut sync_rate, count_rates.as_mut_ptr()) };
        mh_to_result!(mh_result, (sync_rate, count_rates))
    }
}

/// WhiteRabbit functionality -- not
/// implemented for debug tools.
#[cfg(feature = "MHLib")]
#[allow(dead_code)]
impl MultiHarp150 {
    /// Returns the MAC address of the device as a string of length 6.
    fn wrabbit_get_mac(&self) -> MultiHarpResult<String> {
        let mut mac = [0 as c_char; mhconsts::WR_MAC_LEN];
        let mh_result = unsafe { MH_WRabbitGetMAC(self.index, mac.as_mut_ptr()) };
        mh_to_result!(mh_result, unsafe { CStr::from_ptr(mac.as_mut_ptr()) }.to_str().unwrap().to_string())
    }

    /// Set the MAC address of the device. Must be a string of length 6.
    /// 
    /// Note: The MAC address must be unique within the network you are using
    fn wrabbit_set_mac(&self, mac : &str) -> CheckedResult<(), usize> {
        if mac.len() != mhconsts::WR_MAC_LEN {
            return Err(
                PatinaError::ArgumentError(
                "mac".to_string(),
                mac.len() as usize,
                format!("MAC address must be {} characters long", mhconsts::WR_MAC_LEN))
            );
        }
        let mac = CString::new(mac).unwrap();
        let mh_result = unsafe { MH_WRabbitSetMAC(self.index, mac.as_ptr()) };
        mh_to_result!(mh_result, ()).map_err(|e| PatinaError::from(e))
    }

    /// Retrieves the White Rabbit initialization script from the MultiHarp's EEPROM.
    fn wrabbit_get_init_script(&self) -> MultiHarpResult<String> {
        let mut script = [0 as c_char; mhconsts::WR_SCRIPT_LEN];
        let mh_result = unsafe { MH_WRabbitGetInitScript(self.index, script.as_mut_ptr()) };
        mh_to_result!(mh_result, unsafe { CStr::from_ptr(script.as_mut_ptr()) }.to_str().unwrap().to_string())
    }

    /// Sets the White Rabbit initialization script in the MultiHarp's EEPROM.
    /// Lines are separated by a newline character.
    fn wrabbit_set_init_script(&self, script : &str) -> MultiHarpResult<()> {
        let script = CString::new(script).unwrap();
        let mh_result = unsafe { MH_WRabbitSetInitScript(self.index, script.as_ptr()) };
        mh_to_result!(mh_result, ())
    }

    /// Used to retrieve SFP module calibration data (if any) from EEPROM.
    /// 
    /// ## Returns
    /// 
    /// - A tuple of the SFP serial number, dTx, dRx, and alpha values
    /// for each of 4 SFPs
    fn wrabbit_get_sfp_data(&self) -> [(String, i32, i32, i32); 4]{
        let mut sfp_names = [0 as c_char; 4*20];
        let mut dtxs = [0i32; 4];
        let mut drxs = [0i32; 4];
        let mut alphas = [0i32; 4];
        
        unsafe { 
            MH_WRabbitGetSFPData(
             self.index, 
            sfp_names.as_mut_ptr(),
                dtxs.as_mut_ptr(),
                drxs.as_mut_ptr(),
            alphas.as_mut_ptr()
        ) };

        [
            (
                unsafe { CStr::from_ptr(sfp_names.as_mut_ptr()).to_str().unwrap().to_string() },
                dtxs[0], drxs[0], alphas[0]
            ),
            (
                unsafe { CStr::from_ptr(sfp_names.as_mut_ptr().add(20)).to_str().unwrap().to_string() },
                dtxs[1], drxs[1], alphas[1]
            ),
            (
                unsafe { CStr::from_ptr(sfp_names.as_mut_ptr().add(40)).to_str().unwrap().to_string() },
                dtxs[2], drxs[2], alphas[2]
            ),
            (
                unsafe { CStr::from_ptr(sfp_names.as_mut_ptr().add(60)).to_str().unwrap().to_string() },
                dtxs[3], drxs[3], alphas[3]
            )
        ]  
    }

    /// Used to set SFP module calibration data in EEPROM.
    fn wrabbit_set_sfp_data(
        &self,
        sfp_names : [String; 4],
        dtxs : [i32; 4],
        drxs : [i32; 4],
        alphas : [i32; 4]
    ) -> MultiHarpResult<()> {
        // create a single string of all the sfp names
        let mut sfp_names_str = String::new();

        for name in sfp_names.iter() {
            sfp_names_str.push_str(name);
        }

        let sfp_names = CString::new(sfp_names_str).unwrap();
        let mh_result = unsafe { MH_WRabbitSetSFPData(
            self.index,
            sfp_names.as_ptr(),
            dtxs.as_ptr(),
            drxs.as_ptr(),
            alphas.as_ptr()
        ) };
        mh_to_result!(mh_result, ())
    }

    /// Set WhiteRabbit link on or off.
    fn set_wrabbit_link(&self, on : bool) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_WRabbitInitLink(self.index, on as i32) };
        mh_to_result!(mh_result, ())
    }

    /// Set how the White Rabbit core boots.
    /// 
    /// ## Arguments
    /// 
    /// * `boot_from_script` - Whether to boot from the script.
    /// If true, boots from script in EEPROM (set with `wrabbit_set_init_script`).
    /// 
    /// * `reinit_with_mode` - Whether to reinitialize with a new mode
    /// (provided in the third argument)
    /// 
    /// * `mode` - The mode to set the WRabbit to. Must be between 0 and 3.
    /// 0 : Off, 1 : Slave, 2 : Master, 3 : GrandMaster
    fn set_wrabbit_mode(&self, boot_from_script : bool, reinit_with_mode : bool, mode : WRMode) -> MultiHarpResult<()> {
        let mh_result = unsafe { 
            MH_WRabbitSetMode(
        self.index,
!boot_from_script as i32,
                reinit_with_mode as i32,
                mode as i32)
            };
        mh_to_result!(mh_result, ())
    }

    /// Used to set the current UTC time of a White Rabbit code for
    /// a device configured as a WR master. If a slave is connected,
    /// it will be set to the same time.
    fn set_wrabbit_time(&self, time_high_dw : u32, time_low_dw : u32) -> MultiHarpResult<()> {
        let mh_result = unsafe { MH_WRabbitSetTime(self.index, time_high_dw, time_low_dw) };
        mh_to_result!(mh_result, ())
    }

    /// Retrieve the UTC time of a MultiHarp's WR core.
    /// 
    /// ## Returns
    /// 
    /// * (time_high_dw, time_low_dw, subsec_16_ns) - The time in 3 parts:
    ///    - `time_high_dw` - The most significant 32 bits of the time in secsonds since epoch
    ///    - `time_low_dw` - The lowest 32 bits of the time in seconds since epoch
    ///    - `subsec_16_ns` - The subsecond part of the time in 16 ns units.
    fn get_wrabbit_time(&self) -> MultiHarpResult<(u32, u32, u32)> {
        let mut time_high_dw = 0u32;
        let mut time_low_dw = 0u32;
        let mut subsec_16_ns = 0u32;
        let mh_result = unsafe { MH_WRabbitGetTime(self.index, &mut time_high_dw, &mut time_low_dw, &mut subsec_16_ns) };
        mh_to_result!(mh_result, (time_high_dw, time_low_dw, subsec_16_ns))
    }

    /// Get the status of the WRabbit core. Interpreted as a
    /// bitfield, using the masks in `mhconsts`.
    fn get_wrabbit_status(&self) -> MultiHarpResult<i32> {
        let mut status = 0;
        let mh_result = unsafe { MH_WRabbitGetStatus(self.index, &mut status) };
        mh_to_result!(mh_result, status)
    }

    /// When the MultiHarp’s WR core has received the command gui
    /// (should be the last line of the init script) it sends terminal
    /// output describing its state. 
    /// This needs to be done repeatedly.
    /// The output will contain escape sequences for control of
    /// text color, screen refresh, etc. In order to present it
    /// correctly these escape sequences must be interpreted and
    /// translated to the corresponding control mechanisms of
    /// the chosen display scheme. To take care of this the data can
    /// be sent to a terminal emulator. Note that this is read-only.
    /// There is currently no way of injecting commands to the WR
    /// core’s console prompt.
    fn get_wrabbit_term_output(&self) -> MultiHarpResult<String> {
        let mut buffer = [0 as c_char; mhconsts::WR_TERM_LEN];
        let mut term_output_chars = 0;
        let mh_result = unsafe { MH_WRabbitGetTermOutput(self.index, buffer.as_mut_ptr(), &mut term_output_chars) };

        // Take only the `term_output_chars` from `buffer` and
        // copy them to a string to return

        // Maybe a bad implementation...
        let mut term_output = String::new();
        for i in 0..term_output_chars {
            term_output.push(buffer[i as usize] as u8 as char);
        }

        mh_to_result!(mh_result, term_output)
    }
}

// #[cfg(feature = "async")]
// impl AsyncMultiHarpDevice for MultiHarp150 {
//     /// Implements an asynchronous version of `read_fifo` for the MultiHarp150 for
//     /// single-threaded asynchronous programming models.
//     async fn read_fifo_async<'a, 'b>(&'a self, buffer : &'b mut Vec<u32>) -> AsyncCheckedResult<i32, u32> { 
//         async move {
//             self.read_fifo(buffer).map_err(|e| e.into_future())
//         }.await
//     }
// }

#[cfg(feature = "MHLib")]
impl Drop for MultiHarp150 {
    fn drop(&mut self) {
        let mh_return = unsafe { MH_CloseDevice(self.index) };
        if mh_return != 0 {
            eprintln!("Error closing device {}: {}", self.index, error_to_string(mh_return as i32).unwrap());
        }
    }
}