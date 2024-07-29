//! Multithreading is hard -- but not with Rust!
//! 
//! Here's some example code you can use to exchange
//! MultiHarp measurement data across threads -- safely!

use std::sync::{
     Arc, Mutex,
    atomic::{AtomicBool,Ordering},
};

use multi_harp_patina::*;

/// This is a simple example of how to use the `MultiHarp150` struct
/// in a multithreaded environment, sharing a single buffer histogram
/// that is updated by the `MultiHarp150` struct in one thread, and
/// offloaded by a second.
fn main() {

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

    load_default_config(&mut mh);

    let shared_info
        = (Vec::<u32>::with_capacity(TTREADMAX), 0 as usize);

    mh.start_measurement(ACQTMAX)
    .map_err(|e| {println!("Error starting measurement: {:?}", e); return ();}).unwrap();
    // protect the histogram and the number stored in the tuple
    let histo_ptr = Arc::new(Mutex::new(shared_info));

    // Allows us to tell the `ReadFifo` thread to stop
    let acquiring = Arc::new(AtomicBool::new(true));

    let histoptr = Arc::clone(&histo_ptr);
    let acqpt = Arc::clone(&acquiring);

    let load_stored_thread = std::thread::spawn(move || {
        load_stored_histogram(&mut mh, histoptr, acqpt);
    });

    let acqpt = Arc::clone(&acquiring);
    let histoptr = Arc::clone(&histo_ptr);

    let handle_stored_thread = std::thread::spawn(move ||
        {offload_data(histoptr, acqpt)}
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    acquiring.store(false, Ordering::Relaxed);
    load_stored_thread.join();
    
}

fn load_default_config(multiharp : &mut MultiHarp150) {
    let config = MultiHarpConfig {
        binning : Some(0) ,
        sync_channel_offset : Some(10),
        sync_div : Some(2),
        sync_trigger_edge : Some((-350, TriggerEdge::Falling)),
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

    multiharp.set_from_config(&config);
}

fn offload_data(
    histo_ptr : Arc<Mutex<(Vec<u32>, usize)>>,
    acquire : Arc<AtomicBool>
    ) {
    while acquire.load(Ordering::Relaxed) {
        let mut histo = histo_ptr.lock().unwrap();
        if histo.1 != 0 {
            println!("Histogram has {} entries", histo.1);
            histo.0.clear();
            histo.1 = 0;
        }
    }
}

/// Called as often as possible, this method just
/// reads the MultiHarp150 FIFO and stores the data
/// in the shared histogram memory.
fn load_stored_histogram(
    multiharp : &mut MultiHarp150,
    histo_ptr : Arc<Mutex<(Vec<u32>, usize)>>,
    acquire : Arc<AtomicBool>
    ) {
    
    let mut read_histogram = vec![0u32; TTREADMAX];
    
    while let Ok(x) = multiharp.ctc_status(){
        if !x || !acquire.load(Ordering::Relaxed) {break;}

        let read_time = std::time::Instant::now();
        match multiharp.read_fifo(&mut read_histogram) {
            Ok(ncount) => {
                // lock the shared memory
                let mut histo = histo_ptr.lock().unwrap();
        
                histo.1 += ncount as usize;
                histo.0.extend(read_histogram.iter().take(ncount as usize));    
                println!{"Loaded {} reads in {} milliseconds", ncount, read_time.elapsed().as_millis()};
            },
            Err(e) => {println!{"Error reading FIFO: {:?}", e}; return;}
        }
    }
    println!("Exiting histo thread");
}