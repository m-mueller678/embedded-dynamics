use core::mem;
use crate::usb::write_usb;
use crate::WriteUsb;
use core::fmt::Write;

#[export_name = "SVCall"]
pub unsafe extern "C" fn sv_call_extern() {
    let stack_ptr:u32;
    core::arch::asm!{
        "mrs {}, msp",
        out(reg) stack_ptr,
    };
    writeln!(WriteUsb, "msp = b{:x}",stack_ptr).unwrap();
    for i in -10..50{
        let data = (stack_ptr as *const u32).offset(i).read();
        writeln!(WriteUsb,"stack[{}] = {:x}",i,data).unwrap();
    }
}

unsafe fn sv_call(a:u32,b:u32)->Result<u32,()>{
    use guest_lib::syscall_id;
    Ok(match a{
        syscall_id::WRITE=>{
            let args=decode_mem_range::<u32>(b,2)?;
            let data=decode_mem_range::<u8>(args[0],args[1])?;
            write_usb(data).is_ok() as u32
        },
        _=>unimplemented!(),
    })
}

unsafe fn decode_mem_range<'a,T>(start:u32,len:u32)->Result<&'a [T],()>{
    guest_lib::memory_space::assert_range_in_guest_range(start,len,mem::size_of::<T>())?;
    Ok(core::slice::from_raw_parts(start as *mut T,len as usize))
}