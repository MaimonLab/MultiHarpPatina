# Multi-Harp-Patina

A thin wrapper around the `MultiHarp` control library
for `Rust`. Provides a `DebugMultiHarp150` struct to
emulate the behavior of a `MultiHarp` without actually
being connected to one. Provides a few additional convenience
functions, such as a `Config` struct to set many parameters
in one call.

TODO
-----

- Make the debug simulation
more complete than it currently is.

- Implement FPGA functionality / MultiHarp 160

- Benchmarking and test suites.

- `async`

## Features

Not all functions are available on all MHLib versions.
Use the `features` flags to build for your installed driver.
If you are unsure which library you are using, install with the
`MHLib` flag, then use the `get_library_version()` function to
check.

* `nolib` - for use on machines that have no installed MultiHarp library,
just for debugging (e.g. if you are working on an application that reads from
a MultiHarp but on a system without one connected) The `C` API is not exposed
by this `Patina` but only the `DebugMultiHarp` structs will be built.

* `default` - the default flags for this library, which presume a user is
using `MHLib >=v3.0` (uses flags `MHLib` and `MHLv3_0_0`)

* `MHLib` - A flag for `MHLib <3` functionality only

* `MHLv3_0_0` - A flag for features compliant with `v3.0.0` but no further

* `MHLv3_1_0` - Builds `MHLibv3_0_0` functionality + functions specific to `v3.1.0`

**Warning!** Not all functionality has been tested yet.
The current developer does not use the White Rabbit and FPGA
functionality, and has not tested the event filtering.
Use at your own risk (and then send feedback
to thornquist@rockefeller.edu)!

## Usage

This library wraps the `C` API and provides `Rust`-style
access to the functionality. For example code, refer to
the source code in `src/bin`. This section provides only
a very minimal example.

```rust
use multi_harp_patina::*;

fn main(){

    // This line opens the first `MultiHarp150` it finds,
    // handling the various errors with a print line and
    // terminating the code without a panic.
    // Returns `None` if no devices available
    let multi_harp = open_first_device::<MultiHarp150>();

    match &multi_harp {
        Some(Ok(m)) => {
            println!("Opened device with serial number {}", m.get_serial());
        }
        Some(Err(e)) => {
            match e {
                CheckedError::ArgumentError(s, i, msg) => println!("Argument error: {} {} {}", s, i, msg),
                CheckedError::MultiHarpError(e) => println!("Error opening device: {:?}", e),
                _ => println!("Unknown error opening device"),
            }
            return ();
        }
        None => {
            println!("No devices available!")
            return ();
        }
    }

    let mut mh = multi_harp.unwrap().unwrap();

    mh.init(MeasurementMode::T3, ReferenceClock::Internal)
        .map_err(|e| {println!("Error initializing device: {:?}", e); return ();})
        .unwrap();
    
    // Run for 4 seconds
    mh.start_measurement(4000)
    .map_err(|e| {
        println!("Error starting measurement: {:?}", e); return ();
    }).unwrap();

    // Normally you'd handle this stuff with multi-threading,
    // allowing the buffer to be read and processed in parallel
    // for maximum efficiency. Check out the examples!
    let mut buf = vec![0u32; multi_harp_patina::TTREADMAX];
    while let Ok(x) = mh.ctc_status() {
        if !x {break;}
        let n_reads = mh.read_fifo(&mut buf)
        .map_err(|e| {
            println!("Error reading FIFO: {:?}", e); return ();
        }).unwrap();

        // Do something with the data,
        // send it to another thread,
        // send it to a friend,
        // write it to a file??
        /* - snip - */
    }

    mh.stop_measurement().map_err(|e| {
        println!("Error stopping measurement: {:?}", e); return ();
    }).unwrap();

    // when mh goes out of scope, it will call `CloseDevice` on its
    // own
}
```