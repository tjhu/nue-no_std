#![deny(missing_docs)]
#![no_std]


//! A collection of tools for working with binary data, I/O, and POD structs.
//!
//! Re-exports items from the `pod` and `nue_io` crates. See `nue_macros`
//! for more examples and usage.

extern crate packed as nue_packed;
extern crate pod;

pub use pod::*;
pub use nue_packed::*;
