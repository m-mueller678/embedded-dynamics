use cortex_m::interrupt::free as critical_section;
use rp_pico::hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usbd_serial::SerialPort;
use usb_device::device::{UsbDevice, UsbDeviceBuilder, UsbVidPid};
use crate::{hal, pac, RESETS, USBCTRL_DPRAM, USBCTRL_REGS,pac::interrupt};
use crate::hal::clocks::UsbClock;

static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

fn assert_single_core(){
    //TODO
}

pub fn init_usb(mut resets: &mut RESETS, usb_clock: UsbClock, usbctrl_dpram: USBCTRL_DPRAM, usbctrl_regs: USBCTRL_REGS) {
    // static accesses are safe because interrupts are not active yet and this function can not be called more than once
    // because it takes UsbClock

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        usbctrl_regs,
        usbctrl_dpram,
        usb_clock,
        true,
        &mut resets,
    ));
    let bus_ref= unsafe {
        USB_BUS = Some(usb_bus);
        USB_BUS.as_ref().unwrap()
    };
    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    unsafe {
        USB_DEVICE = Some(usb_dev);
    }
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };
}

pub struct WriteUsb;

impl core::fmt::Write for WriteUsb {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        assert_single_core();
        let mut data = s.as_bytes();
        while data.len() > 0 {
            critical_section(|_cs| {
                let written =unsafe{
                    USB_SERIAL.as_mut().unwrap().write(data).map_err(|_| core::fmt::Error)?
                };
                data = &data[written..];
                Ok(())
            })?;
        }
        Ok(())
    }
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    use core::sync::atomic::{AtomicBool, Ordering};

    /// Note whether we've already printed the "hello" message.
    static SAID_HELLO: AtomicBool = AtomicBool::new(false);

    critical_section(|_|{
        let usb_dev = USB_DEVICE.as_mut().unwrap();
        let serial = USB_SERIAL.as_mut().unwrap();
        if usb_dev.poll(&mut [serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(_) => {
                    panic!();
                }
            }
        }
    });
}
