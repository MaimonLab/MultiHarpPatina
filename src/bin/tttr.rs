//! Implements an example with a simple `main` function, just as in the
//! `MultiHarp` official documentation.
use multi_harp_patina::*;

fn main() {
    let libv = get_library_version();
    match libv {
        Ok(v) => println!("Library version: {}", v),
        Err(e) => println!("Error getting library version: {:?}", e),
    }

    println!("Searching for MultiHarp devices...");
    let devs = available_devices();
    println!("Available devices : {:?}", devs);

    let mut mh = open_first_device::<MultiHarp150>();
    match &mh {
        Ok(m) => {
            println!("Opened device with serial number {}", m.get_serial());
            println!("Number of channels: {}", m.num_input_channels().unwrap());
        }
        Err(e) => {
            match e {
                PatinaError::NoDeviceAvailable => println!("No devices available"),
                PatinaError::ArgumentError(s, i, msg) => println!("Argument error: {} {} {}", s, i, msg),
                PatinaError::MultiHarpError(e) => println!("Error opening device: {:?}", e),
                _ => println!("Unknown error opening device"),
            }
        }
    }

    let mut mh = mh.unwrap();
    mh.init(MeasurementMode::T3, ReferenceClock::Internal).unwrap();

    let (model, partno, ver) = mh.get_hardware_info().unwrap();
    println!("Model: {}, Part number: {}, Version: {}", model, partno, ver);

    

}