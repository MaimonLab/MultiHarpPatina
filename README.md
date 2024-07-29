# Multi-Harp-Patina

A thin wrapper around the `MultiHarp` control library
for `Rust`. Provides a `DebugMultiHarp150` struct to
emulate the behavior of a `MultiHarp` without actually
being connected to one.

TODO
-----

- Make the debug simulation
more complete than it currently is.

- Implement White Rabbit functionality

- Implement FPGA functionality

- Implement event filtering.

## Features

Not all functions are available on all MHLib versions.
Use the `features` flags to build for your installed driver.

* `nolib` - for use on machines that have no installed MultiHarp library,
just for debugging (e.g. if you are working on an application that reads from
a MultiHarp but on a system without one connected) The `C` API is not exposed
by this `Patina` but only the `DebugMultiHarp` structs will be built.

* `default` - the default flags for this library, which presume a user is
using `MHLib >=v3.0` (uses flags `MHLib` and `MHLv3_0_0`)

* `MHLib` - A flag for `MHLib <3` functionality only

* `MHLv3_0_0` - A flag for features compliant with `v3.0.0` but no further

* `MHLv3_1_0` - Builds `MHLibv3_0_0` functionality + functions specific to `v3.1.0`

## Usage

```rust
use multi_harp_patina;

/// TODO document!

```