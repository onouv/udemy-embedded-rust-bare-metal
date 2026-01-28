use core::ptr;

use crate::{
    mcu::{error::{MCU_ERR_INVALID_BITLEN, MCU_ERR_INVALID_OFFSET}, register},
    utils::bits,
};

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

pub unsafe fn set_bits(
    address: RegisterAddress,
    value: u32,
    offset: u32,
    value_bit_length: u32,
) -> Result<(), MCUErrorCode> {
    if offset > 31 {
        return Err(MCU_ERR_INVALID_OFFSET);
    }

    if !(1..=32).contains(&value_bit_length) {
        return Err(MCU_ERR_INVALID_BITLEN);
    }

    unsafe {
        let old = read(address);
        let mask = ((1 << value_bit_length) - 1) << offset;
        let update = bits::set(bits::clear(old, mask), mask);
        write(address, update);
    }

    Ok(())
}

pub unsafe fn set_bit(address: RegisterAddress, bit_position: u32, bit_is_high: bool) -> Result<(), MCUErrorCode> {
    if bit_position > 31 {
        return Err(MCU_ERR_INVALID_OFFSET);
    }

    unsafe {
        let register_val = read(address);
        let update_val = if bit_is_high {
            register_val | (1 << bit_position) 
        } else {
            register_val & !(1 << bit_position) 
        };

        write(address, update_val);
    }

    Ok(())
}