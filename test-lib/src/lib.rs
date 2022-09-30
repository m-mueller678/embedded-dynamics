#![no_std]

use panic_halt as _;

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
	left + right+42
}
