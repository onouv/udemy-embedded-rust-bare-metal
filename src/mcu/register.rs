use core::ptr;
use super::MCUError;

pub type Address = *mut u32;

pub struct Register {
    addr: Address
}

impl Register {
    pub const fn new(addr: Address) -> Self {
        Self {
            addr
        }
    }

    unsafe fn read(&self) -> u32 {
        unsafe { ptr::read_volatile(self.addr) }
    }

    unsafe fn write(&self, value: u32) {
        unsafe {
            ptr::write_volatile(self.addr, value);
        }
    }

    pub unsafe fn set_bits(
        &self,
        value: u32,
        offset: u32,
        value_bit_length: u32,
    ) -> Result<(), MCUError> {
        if offset > 31 {
            return Err(MCUError::InvalidOffset);
        }

        if !(1..=32).contains(&value_bit_length) {
            return Err(MCUError::InvalidOffset);
        }

        unsafe {
            let reg_value = self.read();
            let mask = ((1 << value_bit_length) - 1) << offset;

            // Clear the relevant bits in the register and set the new value
            let update = (reg_value & !mask) | ((value << offset) & mask);

            self.write(update);
        }

        Ok(())
    }
    /**
     * Set the bit indicated by bit_position (starting bit 0) to '1', 
     * and return Ok wih the resulting value or an Err with an error code.
     */
    pub unsafe fn set_bit(&self, bit_position: u32) -> Result<(), MCUError> {
        if bit_position > 31 {
            return Err(MCUError::InvalidOffset);
        }

        unsafe {
            let reg_val = self.read();
            let update_val = reg_val | (1 << bit_position);
            self.write(update_val);
        }

        Ok(())
    }

    /**
     * Set the bit indicated by bit_position (starting bit 0) to '0', 
     * and return Ok wih the resulting value or an Err with an error code.
     */
    pub unsafe fn clear_bit(&self, bit_position: u32) -> Result<(), MCUError> {
        if bit_position > 31 {
            return Err(MCUError::InvalidOffset);
        }

        unsafe {
            let reg_val = self.read();
            let update = reg_val & !(1 << bit_position);
            self.write(update);
        }

        Ok(())
    }
}