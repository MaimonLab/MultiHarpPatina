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

    let mh = open_first_device::<MultiHarp150>();
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
            return ();
        }
    }

    let mut mh = mh.unwrap();
    mh.init(MeasurementMode::T3, ReferenceClock::Internal)
    .map_err(|e| {println!("Error initializing device: {:?}", e); return ();})
    .unwrap();

    let (model, partno, ver) = mh.get_hardware_info()
    .map_err(|e| {println!("Error getting hardware info: {:?}", e); return ();}).unwrap();
    
    println!("Model: {}, Part number: {}, Version: {}", model, partno, ver);

    let config = MultiHarpConfig {
        binning : Some(0) ,
        sync_channel_offset : Some(10),
        sync_div : Some(2),
        sync_trigger_edge : Some((-80, TriggerEdge::Falling)),
        input_edges: Some(vec![
            (0, -100, TriggerEdge::Falling),
            (1, -100, TriggerEdge::Falling),
            (2, -100, TriggerEdge::Falling),
            (3, -100, TriggerEdge::Falling),
        ]),
        input_enables: Some(
            vec![
                (0, true),
                (1, true),
                (2, false),
                (3, false),
            ]
        ),
        ..Default::default()
    };

    mh.set_from_config(&config);

    mh.get_resolution().map(|r| println!("Resolution: {} picoseconds", r)).unwrap();

    mh.get_all_count_rates().map(|(sync, countrates)| {
        println!("Sync rate: {} Hz", sync);
        for (i, c) in countrates.iter().enumerate() {
            println!("Channel {} count rate: {} Hz", i, c);
        }
    }).unwrap();

    println!("{}", mh.get_warnings_text().unwrap());

    mh.start_measurement(4000)
    .map_err(|e| {
        println!("Error starting measurement: {:?}", e); return ();
    }).unwrap();

    let mut buf = vec![0u32; multi_harp_patina::TTREADMAX];
    while let Ok(x) = mh.ctc_status() {
        if x {
            break;
        }
        // We'll time the read while we're at it
        let time = std::time::Instant::now();
        let n_reads = mh.read_fifo(&mut buf)
        .map_err(|e| {
            println!("Error reading FIFO: {:?}", e); return ();
        }).unwrap();

        println!("Read {} records in {} us", n_reads, time.elapsed().as_micros());
        

        // Do something with the data,
        // send it to another thread,
        // send it to a friend,
        // write it to a file??
        /* - snip - */
//        println!("Read {} records", n_reads);
    }

    mh.stop_measurement().map_err(|e| {
        println!("Error stopping measurement: {:?}", e); return ();
    }).unwrap();

    // when mh goes out of scope, it will call `CloseDevice`

}