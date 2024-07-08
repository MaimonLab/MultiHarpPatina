//! Interface for the external `MHLib` library calls

use std::ffi::*;
use crate::error::MultiHarpError;

/// Rust FFI for the MHLib
//#[link(name = "mhlib")]
#[cfg_attr(windows, link(name = "mhlib64", kind = "dylib"))]
#[cfg_attr(unix, link(name = "mhlib", kind = "dylib"))]
extern "C" {
    pub fn MH_GetLibraryVersion(vers : *mut c_char) -> c_int;
    pub fn MH_GetErrorString(errstring : *mut c_char, errcode : c_int) -> c_int;
    
    pub fn MH_OpenDevice(devidx : c_int, serial : *mut c_char) -> c_int;
    pub fn MH_CloseDevice(devidx : c_int) -> c_int;
    pub fn MH_Initialize(devidx : c_int, mode : c_int, refsource : c_int) -> c_int;

    // Only usable after MH_Initialize

    pub fn MH_GetHardwareInfo(devidx : c_int, model : *mut c_char, partno : *mut c_char, version : *mut c_char) -> c_int;
    pub fn MH_GetSerialNumber(devidx : c_int, serial : *mut c_char) -> c_int;
    pub fn MH_GetFeatures(devidx : c_int, features : *mut c_int) -> c_int;
    pub fn MH_GetBaseResolution(devidx : c_int, resolution : *mut c_double, binsteps: *mut c_int) -> c_int;
    pub fn MH_GetNumOfInputChannels(devidx : c_int, n_channels : *mut c_int) -> c_int;

    pub fn MH_SetSyncDiv(devidx : c_int, sync_div : c_int) -> c_int;
    pub fn MH_SetSyncEdgeTrg(devidx : c_int, level : c_int, sync_edge : c_int) -> c_int;
    pub fn MH_SetSyncChannelOffset(devidx : c_int, offset : c_int) -> c_int;
    /// New in v3.1
    pub fn MH_SetSyncChannelEnable(devidx : c_int, enable : c_int) -> c_int;
    /// New in v1.1
    pub fn MH_SetSyncDeadTime(devidx : c_int, on : c_int, deadtime : c_int) -> c_int;

    pub fn MH_SetInputEdgeTrg(devidx : c_int, channel : c_int, level : c_int, edge : c_int) -> c_int;
    pub fn MH_SetInputChannelOffset(devidx : c_int, channel : c_int, offset : c_int) -> c_int;
    /// New in v1.1
    pub fn MH_SetInputDeadTime(devidx : c_int, channel : c_int, on : c_int, deadtime : c_int) -> c_int;
    /// New in v3.0
    pub fn MH_SetInputHysteresis(devidx : c_int, hystcode : c_int) -> c_int;
    pub fn MH_SetInputChannelEnable(devidx : c_int, channel : c_int, enable : c_int) -> c_int;

    pub fn MH_SetStopOverflow(devidx : c_int, stop_overflow : c_int, stopcount : c_uint) -> c_int;
    pub fn MH_SetBinning(devidx : c_int, binning : c_int) -> c_int;
    pub fn MH_SetOffset(devidx : c_int, offset : c_int) -> c_int;
    pub fn MH_SetHistoLen(devidx : c_int, len_code : c_int, actual_len : *mut c_int) -> c_int;
    pub fn MH_SetMeasControl(devidx : c_int, control : c_int, startedge : c_int, stop_edge : c_int) -> c_int;
    pub fn MH_SetTriggerOutput(devidx : c_int, period : c_int) -> c_int;

    pub fn MH_ClearHistMem(devidx : c_int) -> c_int;
    pub fn MH_StartMeas(devidx : c_int, tacq : c_int) -> c_int;
    pub fn MH_StopMeas(devidx : c_int) -> c_int;
    pub fn MH_CTCStatus(devidx : c_int, ctc : *mut c_int) -> c_int;

    pub fn MH_GetHistogram(devidx : c_int, chcount : *mut c_uint, channel : c_int) -> c_int;
    pub fn MH_GetAllHistograms(devidx : c_int, chcount : *mut c_uint) -> c_int;
    pub fn MH_GetResolution(devidx : c_int, resolution : *mut c_double) -> c_int;
    pub fn MH_GetSyncPeriod(devidx : c_int, period : *mut c_double) -> c_int;
    pub fn MH_GetSyncRate(devidx : c_int, sync_rate : *mut c_int) -> c_int;
    pub fn MH_GetCountRate(devidx : c_int, channel : c_int, count_rate: *mut c_int) -> c_int;
    pub fn MH_GetAllCountRates(devidx : c_int, sync_rate : *mut c_int, count_rates : *mut c_int) -> c_int;
    pub fn MH_GetFlags(devidx : c_int, flags : *mut c_int) -> c_int;
    pub fn MH_GetElapsedMeasTime(devidx : c_int, elapsed_time : *mut c_double) -> c_int;
    pub fn MH_GetStartTime(devidx : c_int, time_dw2 : *mut c_uint, time_dw1 : *mut c_uint, time_dw0 : *mut c_uint) -> c_int;

    pub fn MH_GetWarnings(devidx : c_int, warnings : *mut c_int) -> c_int;
    pub fn MH_GetWarningsText(devidx : c_int, text : *mut c_char, warnings : c_int) -> c_int;

    // Time tagging functions only
    
    /// New in v3.1
    pub fn MH_SetOflCompression(devidx : c_int, holdtime : c_int) -> c_int;
    pub fn MH_SetMarkerHoldoffTime(devidx : c_int, holdofftime : c_int) -> c_int;
    pub fn MH_SetMarkerEdges(devidx : c_int, me1 : c_int, me2 : c_int, me3 : c_int, me4: c_int) -> c_int;
    pub fn MH_SetMarkerEnable(devidx : c_int, en1 : c_int, en2: c_int, en3: c_int, en4: c_int) -> c_int;
    pub fn MH_ReadFiFo(devidx : c_int, buffer : *mut c_uint, n_actual : *mut c_int) -> c_int;

    // Eventing filtering, time-tagging only, new in v3.1
    pub fn MH_SetRowEventFilter(devidx : c_int, rowidx : c_int, timerange : c_int, matchcnt : c_int, inverse : c_int, usechannels : c_int, passchannels : c_int) -> c_int;
    pub fn MH_EnableRowEventFilter(devidx : c_int, rowidx : c_int, enable : c_int) -> c_int;
    pub fn MH_SetMainEventFilterParams(devidx : c_int, timerange : c_int, matchcnt : c_int, inverse : c_int) -> c_int;
    pub fn MH_SetMainEventFilterChannels(devidx : c_int, rowidx : c_int, usechannels : c_int, passchannels : c_int) -> c_int;
    pub fn MH_EnableMainEventFilter(devidx : c_int, enable : c_int) -> c_int;
    pub fn MH_SetFilterTestMode(devidx : c_int, testmode : c_int) -> c_int;
    pub fn MH_GetRowFilteredRates(devidx : c_int, syncrate : *mut c_int, cntrates : *mut c_int) -> c_int;
    pub fn MH_GetMainFilteredRates(devidx : c_int, syncrate : *mut c_int, cntrates : *mut c_int) -> c_int;

    // Debugging only
    pub fn MH_GetDebugInfo(devidx : c_int, debuginfo : *mut c_char) -> c_int;
    pub fn MH_GetNumOfModules(devidx : c_int, nummod : *mut c_int) -> c_int;
    pub fn MH_GetModuleInfo(devidx : c_int, modidx : c_int, modelcode : *mut c_int, versioncode : *mut c_int) -> c_int;

    // White Rabbit only
    pub fn MH_WRabbitGetMAC(devidx : c_int, mac_addr : *mut c_char) -> c_int;
    pub fn MH_WRabbitSetMAC(devidx : c_int, mac_addr : *mut c_char) -> c_int;
    pub fn MH_WRabbitGetInitScript(devidx : c_int, initscript : *mut c_char) -> c_int;
    pub fn MH_WRabbitSetInitScript(devidx : c_int, initscript : *mut c_char) -> c_int;
    pub fn MH_WRabbitGetSFPData(devidx : c_int, sfpnames : *mut c_char, dTxs : *mut c_int, dRxs : *mut c_int, alphas : *mut c_int) -> c_int;
    pub fn MH_WRabbitSetSFPData(devidx : c_int, sfpnames : *mut c_char, dTxs : *mut c_int, dRxs : *mut c_int, alphas : *mut c_int) -> c_int;
    pub fn MH_WRabbitInitLink(devidx : c_int, link_on : c_int) -> c_int;
    pub fn MH_WRabbitSetMode(devidx : c_int, bootfromscript : c_int, reinit_with_mode : c_int, mode : c_int) -> c_int;
    pub fn MH_WRabbitSetTime(devidx : c_int, timehidw : c_uint, timelodw : c_uint) -> c_int;
    pub fn MH_WRabbitGetTime(devidx : c_int, timehidw : *mut c_uint, timelodw : *mut c_uint, subsec16ns : *mut c_uint) -> c_int;
    pub fn MH_WRabbitGetStatus(devidx : c_int, wrstatus : *mut c_int) -> c_int;
    pub fn MH_WRabbitGetTermOutput(devidx : c_int, buffer : *mut c_char, nchar : *mut c_int) -> c_int;

    // MultiHarp 160 with external FPGA only, all new since v3.0
    pub fn MH_ExtFPGAInitLink(devidx : c_int, linknumber : c_int, on : c_int) -> c_int;
    pub fn MH_ExtFPGAGetLinkStatus(devidx : c_int, linknumber : c_int, status : *mut c_uint) -> c_int;
    pub fn MH_ExtFPGASetMode(devidx : c_int, mode : c_int, loopback : c_int) -> c_int;
    pub fn MH_ExtFPGAResetStreamFifos(devidx : c_int) -> c_int;
    pub fn MH_ExtFPGAUserCommand(devidx : c_int, write : c_int, addr : c_uint, data : *mut c_uint) -> c_int;
}

/// Calls the MultiHarp library to convert an error to a string version of the error.
pub fn error_to_string(errcode : c_int) -> Result<String, MultiHarpError> {
    if errcode < -100 {
        return Err(MultiHarpError::InvalidError);
    }
    let mut errstring = [0 as c_char; 40];
    let result = unsafe { MH_GetErrorString(errstring.as_mut_ptr(), errcode) };
    if result == 0 {
        Ok(unsafe { CString::from_raw(errstring.as_mut_ptr()) }.to_str().unwrap().to_string())
    } else {
        Err(MultiHarpError::from(result))
    }
}