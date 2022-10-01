use cortex_m_rt::exception;
use crate::WriteUsb;
use core::fmt::Write;
use crate::usb::write_usb;

#[export_name = "SVCall"]
pub unsafe extern "C" fn sv_call_extern() {
    let a:u32;
    let b:u32;
    core::arch::asm!{
        "",
        out("r4") a,
        out("r5") b,
    };
    let c=match sv_call(a,b){
        Ok(c)=>{ c }
        Err(())=>{
            unimplemented!()
        }
    };
    core::arch::asm!{
        "",
        in("r4") c,
    };
    write_usb(b"exret\n").unwrap();
}

unsafe fn sv_call(a:u32,b:u32)->Result<u32,()>{
    use guest_lib::syscall_id;
    Ok(match a{
        WRITE=>{
            let args=decode_mem_range::<u32>(b,2,&())?;
            let data=decode_mem_range::<u8>(args[0],args[1],&())?;
            write_usb(data).is_ok() as u32
        }
    })
}

unsafe fn decode_mem_range<T>(start:u32,len:u32,lifetime:& ())->Result<& [T],()>{
    // TODO verify range is inside guest memory and aligned
    Ok(core::slice::from_raw_parts(start as *mut T,len as usize))
}