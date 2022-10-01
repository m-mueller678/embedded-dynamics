use core::ops::Range;

pub const RAM: Range<u32> = 0x20000000..0x20000000 + (256 << 10);
pub const GUEST_RAM_RANGE: Range<u32> = 0x20000000 + (128 << 10)..0x20000000 + (256 << 10);

pub const GUEST_INITIAL_STACK_POINTER:u32 = GUEST_RAM_RANGE.end;

pub fn assert_range_in_guest_range(start: u32, count: u32, size: usize) -> Result<(), ()> {
    let end = start.checked_add(count.checked_mul(size.try_into().map_err(|_|())?).ok_or(())?).ok_or(())?;
    if GUEST_RAM_RANGE.contains(&start) && GUEST_RAM_RANGE.contains(&end) {
        Ok(())
    } else { Err(()) }
}