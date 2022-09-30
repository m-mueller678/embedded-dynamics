
#![no_std]
#![no_main]


// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    __cortex_m_rt_main()
}
#[doc = " Entry point to our bare-metal application."]
#[doc = ""]
#[doc =
" The `#[entry]` macro ensures the Cortex-M start-up code calls this function"]
#[doc = " as soon as all global variables are initialised."]
#[doc = ""]
#[doc =
" The function configures the RP2040 peripherals, then echoes any characters"]
#[doc = " received over USB Serial."]
fn __cortex_m_rt_main() -> ! {
    loop{}
}
