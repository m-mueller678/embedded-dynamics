#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::register::control::{Control, Npriv, Spsel};
use cortex_m::register::{control, psp};
use embedded_hal::timer::CountDown;
use rp_pico::hal;
use hal::{
    pac,
    Sio,
};
use crate::hal::Timer;

use crate::pac::{RESETS, USBCTRL_DPRAM, USBCTRL_REGS};
use crate::usb::WriteUsb;
use fugit::ExtU32;
use guest_lib::memory_space::GUEST_INITIAL_STACK_POINTER;
use crate::generated_guest::ENTRY_POINT;

mod usb;
mod syscall;
mod generated_guest;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    write!(WriteUsb, "{}\n", info).ok();
    hal::rom_data::reset_to_usb_boot(0, 0);
    loop {}
}

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then echoes any characters
/// received over USB Serial.
#[rp_pico::entry]
fn main() -> ! {
    // Grab our singleton objects
    let pac = pac::Peripherals::take().unwrap();

    let _sio = Sio::new(pac.SIO);

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let mut resets = pac.RESETS;
    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut resets,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let usb_clock = clocks.usb_clock;
    let usbctrl_dpram = pac.USBCTRL_DPRAM;
    let usbctrl_regs = pac.USBCTRL_REGS;
    // Set up the USB driver
    usb::init_usb(&mut resets, usb_clock, usbctrl_dpram, usbctrl_regs);
    let timer = Timer::new(pac.TIMER, &mut resets);
    delay_ms(&timer, 3000);
    writeln!(WriteUsb, "usb init complete!").unwrap();
    try_dyn_load();
    writeln!(WriteUsb, "dyn end").unwrap();
    panic!();
}

fn delay_ms(timer: &Timer, d: u32) {
    let mut cd = timer.count_down();
    cd.start(d.millis());
    nb::block!(cd.wait()).unwrap();
}

fn try_dyn_load() {
    unsafe {
        for ph in generated_guest::PROGRAMM_HEADERS {
            writeln!(WriteUsb,"{:x}, {:x}, {:x}",ph.0,ph.1.len(),ph.0+ph.1.len() as u32).unwrap();
            writeln!(WriteUsb,"guest: {:x?}",guest_lib::memory_space::GUEST_RAM_RANGE).unwrap();
            guest_lib::memory_space::assert_range_in_guest_range(ph.0,1,ph.1.len()).unwrap();
            core::ptr::copy(ph.1.as_ptr(), ph.0 as *mut u8, ph.1.len());
        }
        psp::write(GUEST_INITIAL_STACK_POINTER);
        cortex_m::interrupt::free(|_|{
            let mut control_register = control::read();
            control_register.set_spsel(Spsel::Psp);
            control_register.set_npriv(Npriv::Unprivileged);
            control::write(control_register);
        });
        core::mem::transmute::<u32,extern "C" fn() -> !>(ENTRY_POINT)();
        loop{}
    }
}