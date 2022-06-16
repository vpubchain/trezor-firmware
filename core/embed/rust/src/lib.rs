#![cfg_attr(not(test), no_std)]
#![deny(clippy::all)]
#![allow(clippy::new_without_default)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

mod error;
#[cfg(feature = "micropython")]
#[macro_use]
mod micropython;
#[cfg(feature = "protobuf")]
mod protobuf;
mod time;
#[cfg(feature = "ui_debug")]
mod trace;
#[macro_use]
mod trezorhal;

#[cfg(feature = "ui")]
#[macro_use]
pub mod ui;

#[cfg(not(test))]
#[cfg(any(not(feature = "test"), feature = "clippy"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: as of 2022, ignoring the `_info` parameter does not help with saving
    // flash space -- the `fmt` machinery still gets compiled in.
    // We can avoid that by using unstable Cargo arguments:
    //   -Zbuild-std=core -Zbuild-std-features=panic_immediate_abort
    // Doing that will compile every panic!() to a single udf instruction which raises
    // a Hard Fault on hardware.
    //
    // For debugging, we might still want to look into the PanicInfo to get some data.
    //
    // Otherwise, use `unwrap!` macro from trezorhal.

    // TODO: Ideally we would take the file and line info out of
    // `PanicInfo::location()`.
    trezorhal::common::__fatal_error("", "rs", "", 0, "");
}
