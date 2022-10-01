#![no_std]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod syscall_id{
    pub const WRITE:u32 = 0; // [ptr,len] -> is_ok
}

pub mod sys {
    use crate::syscall_id::*;

    fn call(mut id:u32, arg:u32) ->u32{
        unsafe{
            core::arch::asm! {
                "svc 0",
                inout("r4") id,
                in("r5") arg,
            };
        };
        id
    }

    pub fn write(d: &[u8]) -> Result<(), ()> {
        if call(WRITE as u32, [d.as_ptr() as u32,d.len() as u32].as_ptr() as u32)!=0{
            Ok(())
        }else{
            Err(())
        }
    }
}
