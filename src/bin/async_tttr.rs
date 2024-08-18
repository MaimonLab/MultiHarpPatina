//! This module shows how to use the MultiHarp150 with asynchronous
//! single-threaded operation. TODO!
#[cfg(feature = "async")]
fn main(){}

#[cfg(not(feature = "async"))]
fn main(){
    println!("This example requires the 'async' feature to be enabled.");
    println!("To enable this feature, run the following command:");
    println!("cargo run --example async_tttr --features async");
}