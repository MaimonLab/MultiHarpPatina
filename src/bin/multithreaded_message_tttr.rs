//! Multithreading is hard -- but not with Rust!
//! 
//! Here's some example code you can use to exchange
//! MultiHarp measurement data across threads -- safely!
//! This example uses the `flume` crate for fast message
//! passing. This is faster than the Mutexed histogram approach.
//! 
//! We also include a demo where the MultiHarp is itself in a
//! Mutex for comparison.

use std::sync::{
    Arc, Mutex, atomic::{AtomicBool,Ordering},
};

use flume;

use multi_harp_patina::*;


/// This is a simple example of how to use the `MultiHarp150` struct
/// in a multithreaded environment, sending copies of the buffer histogram
/// that is updated by the `MultiHarp150` struct in one thread to a second for
/// offloading
fn main() {

    #[cfg(feature = "MHLib")]
    let mh = open_first_device::<MultiHarp150>();
    #[cfg(feature = "nolib")]
    let mh = open_first_device::<DebugMultiHarp150>();

    match &mh {
        Ok(m) => {
            println!("Opened device with serial number {}", m.get_serial());
            println!("Number of channels: {}", m.num_input_channels().unwrap());
            println!("Index : {}", m.get_index());
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

    let count_rate = mh.get_all_count_rates()
    .map_err(|e| {println!("Count rate call failure: {:?}", e); return;}).unwrap();

    println!("Count rates: {:?}", count_rate);
    let photons_per_sec = count_rate.1.iter().sum::<i32>();
    let test_duration = 10; // seconds
    println!("That's {} photons per second. You should expect {} in this test",
        photons_per_sec, photons_per_sec * test_duration
    );
 
    mh.start_measurement(ACQTMAX)
    .map_err(|e| {println!("Error starting measurement: {:?}", e); return ();}).unwrap();
    
    // protect the histogram and the number stored in the tuple
    // Stored in an RwLock so that other threads can read from this
    // to do other things with the data while it's in transit if they want
    let (sender, receiver) = flume::unbounded::<(Vec<u32>, usize)>();

    // Allows us to tell the `ReadFifo` thread to stop
    // referred to by multiple threads through a new Arc for
    // each thread -- just to hold the reference to this one.
    let acquiring = Arc::new(AtomicBool::new(true));
    let acq_ptr = Arc::clone(&acquiring);

    let load_stored_thread = std::thread::spawn(move || {
        load_stored_histogram(mh, sender, acq_ptr)
    });

    let handle_stored_thread = std::thread::spawn(move ||
        {offload_data(receiver);}
    );

    // how long to run it
    std::thread::sleep(std::time::Duration::from_secs(test_duration as u64));
    acquiring.store(false, Ordering::Relaxed);
    let mh = load_stored_thread.join().map_err(|e| {println!("Error joining load thread: {:?}", e); return ();}).unwrap();
    handle_stored_thread.join().map_err(|e| {println!("Error joining offload thread: {:?}", e); return ();}).unwrap();

    println!{"Now we'll do the same thing with a mutexed MultiHarp150"};
    // let's try the same thing with a mutexed MultiHarp150
    let mh = Arc::new(Mutex::new(mh));
    let (sender, receiver) = flume::unbounded::<(Vec<u32>, usize)>();

    let acquiring = Arc::new(AtomicBool::new(true));
    let acq_ptr = Arc::clone(&acquiring);

    let load_stored_thread = std::thread::spawn(move || {
        load_stored_histogram_with_mutex(mh, sender, acq_ptr);
    });

    let handle_stored_thread = std::thread::spawn(move ||
        {offload_data(receiver);}
    );

    std::thread::sleep(std::time::Duration::from_secs(test_duration as u64));
    acquiring.store(false, Ordering::Relaxed);
    load_stored_thread.join().map_err(|e| {println!("Error joining load thread: {:?}", e); return ();}).unwrap();
    handle_stored_thread.join().map_err(|e| {println!("Error joining offload thread: {:?}", e); return ();}).unwrap();   
}

fn load_default_config<M : MultiHarpDevice>(multiharp : &mut M) {
    let config = MultiHarpConfig {
        binning : Some(0) ,
        sync_channel_offset : Some(10),
        sync_div : Some(2),
        sync_trigger_edge : Some((-60, TriggerEdge::Falling)),
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
                (2, true),
                (3, true),
            ]
        ),
        ..Default::default()
    };

    match multiharp.set_from_config(&config) {
        Ok(_) => {},
        Err(e) => {
            for err in e {
                println!("{}", err);
            }
        }
    }
}

/// Checks whether the histogram has been updated
/// and then offloads the data, hopefully for other uses
/// (saving? analysis? plotting? drawing an image?)
fn offload_data(receiver : flume::Receiver<(Vec<u32>, usize)>) {
    
    let mut total_processed : usize = 0;
    let mut overflow: usize = 0;
    // Keeps calling until the sender is dropped or some other error in the
    // channel occurs. Blocks while waiting for data.
    while let Ok((histo, counts)) = receiver.recv() {
        
        // println!("Histogram has {} entries", counts);
        
        // Do something with histo here!
        if counts > 0 {
            overflow += histo[0..counts].iter().fold(0, |acc, x| acc + ((x & SPECIAL) >> 31) as usize);
            // println!(
            //     "{} overflow or special markers",
            //     overflow
            // );
                
            // println!("First 10 entries: {:?}", &histo[0..10]);
        }

        total_processed += counts;
    }
    println!{"Total reads processed : {}", total_processed};
    println!{"Total photons : {}", total_processed-overflow};
}

/// Called as often as possible, this method just
/// reads the MultiHarp150 FIFO and shoots it off
/// to the other thread. Returns the MultiHarp when
/// it's done.
fn load_stored_histogram<M : MultiHarpDevice>(
    multiharp : M,
    sender : flume::Sender<(Vec<u32>, usize)>,
    acquire : Arc<AtomicBool>
    ) -> M {
    
    // this one stores the reads from the MultiHarp
    let mut read_histogram = vec![0u32; TTREADMAX];
    
    while let Ok(x) = multiharp.ctc_status(){
        if !x || !acquire.load(Ordering::Relaxed) {break;}

        let read_time = std::time::Instant::now();
        // println!("{:?}",multiharp.get_all_count_rates().unwrap());
        match multiharp.read_fifo(&mut read_histogram) {
            Ok(ncount) => {
                if ncount > 0 {
                    println!{"Loaded {} reads in {} milliseconds", ncount, read_time.elapsed().as_micros() as f64 / 1000.0};
                }

                sender.send((read_histogram.clone(), ncount as usize)).unwrap();
                
            },
            Err(e) => {
                println!{"Error reading FIFO: {:?}", e};
                return multiharp;
            }
        }

        if multiharp.get_warnings().is_ok_and(|x| x > 0) {
            println!("Warnings: {:?}", multiharp.get_warnings().unwrap());
        }
    }
    println!("Exiting histo thread");
    multiharp
}

fn load_stored_histogram_with_mutex<M : MultiHarpDevice>(
    multiharp : Arc<Mutex<M>>,
    sender : flume::Sender<(Vec<u32>, usize)>,
    acquire : Arc<AtomicBool> 
) {
    // this one stores the reads from the MultiHarp
    let mut read_histogram = vec![0u32; TTREADMAX];

    while acquire.load(Ordering::Relaxed) {

        let mh = multiharp.lock().unwrap();

        if !mh.ctc_status().unwrap() {break;}

        let read_time = std::time::Instant::now();
        // println!("{:?}",multiharp.get_all_count_rates().unwrap());
        match mh.read_fifo(&mut read_histogram) {
            Ok(ncount) => {
                if ncount > 0 {
                    println!{"Loaded {} reads in {} milliseconds", ncount, read_time.elapsed().as_micros() as f64 / 1000.0};
                }

                sender.send((read_histogram.clone(), ncount as usize)).unwrap();
                
            },
            Err(e) => {
                println!{"Error reading FIFO: {:?}", e};
                return;
            }
        }

        if mh.get_warnings().is_ok_and(|x| x > 0) {
            println!("Warnings: {:?}", mh.get_warnings().unwrap());
        }
    }
    println!("Exiting histo thread");
}