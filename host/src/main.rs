#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::asm::delay;
use cortex_m::delay::Delay;
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

mod usb;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    write!(WriteUsb,"{}\n",info).ok();
    hal::rom_data::reset_to_usb_boot(0,0);
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
   // let pins = Pins::new(pac.IO_BANK0,pac.PADS_BANK0,sio.gpio_bank0, &mut pac.RESETS);
   // pins.led.into_push_pull_output().set_high().unwrap();

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
    let timer=Timer::new(pac.TIMER,&mut resets);
    delay_ms(&timer,2000);
    writeln!(WriteUsb,"usb init complete!").unwrap();
    panic!();
}

fn delay_ms(timer:&Timer,d:u32){
    let mut cd = timer.count_down();
    cd.start(d.millis());
    nb::block!(cd.wait());
}

fn try_dyn_load()->bool{
    let flash_start = 0x20030000;
    let bytes= &[];//&include_bytes!("../../guest/target/thumbv6m-none-eabi/debug/host")[0x010000..][..0x10];

   unsafe{
        core::ptr::copy(bytes.as_ptr(), flash_start as *mut u8, bytes.len());

        let guest_fn: unsafe extern "C" fn(*mut u8) = core::mem::transmute(0x20030001 );
        let mut data:u8 = 0;
        guest_fn(&mut data);
        data==42
    }
}
