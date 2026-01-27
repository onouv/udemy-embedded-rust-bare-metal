use core::ptr;

use super::error::MCUErrorCode;


pub type RegisterAddress = *mut u32;

pub unsafe fn read(address: *const u32) -> u32 {
    unsafe { ptr::read_volatile(address) }
}

pub unsafe fn write(address: RegisterAddress, value: u32) {
    unsafe {
        ptr::write_volatile(address, value);
    }
}

pub fn set_bits() -> Result<(), MCUErrorCode> {
    Ok(())
}