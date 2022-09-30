use cortex_m_rt::exception;
use crate::WriteUsb;
use core::fmt::Write;

#[allow(non_snake_case)]
#[exception]
unsafe fn SVCall() {
    write!(WriteUsb,"svc").unwrap();
    panic!();
}
