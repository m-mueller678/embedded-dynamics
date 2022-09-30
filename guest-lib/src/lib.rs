#![no_std]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub unsafe fn syscall(r0:u32)->u32{
    core::arch::asm!{"\
        svc 1\
    "};
    42
}