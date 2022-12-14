//! # Pico USB Serial Example
//!
//! Creates a USB Serial device on a Pico board, with the USB driver running in
//! the main thread.
//!
//! This will create a USB Serial device echoing anything it receives. Incoming
//! ASCII characters are converted to upercase, so you can tell it is working
//! and not just local-echo!
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() -> ! {
    guest_lib::sys::call(0xABCDABCD,0xDEEDAAAA);
    //guest_lib::sys::write(b"hello, world 1!\n").unwrap();
    //guest_lib::sys::write(b"hello, world 2!\n").unwrap();
    //guest_lib::sys::write(b"hello, world 3!\n").unwrap();
}
