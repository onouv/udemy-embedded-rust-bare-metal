use super::gpio::GPIOId;
use super::{
    RCC_AHBENR_ADDR, MCUError, register::{Address, Register}
};

/** 
 * RCC related utility functions with private default implementation.
 */
pub(super) trait RCCUtil {
    unsafe fn enable_peripheral_clock(&self, port: GPIOId) -> Result<(), MCUError> {
        let register: Register = Register::new(RCC_AHBENR_ADDR);
        let bit_pos = match port {
            GPIOId::A => 17,
            GPIOId::B => 18,
            GPIOId::C => 19,
            GPIOId::D => 20,
            GPIOId::E => 21,
            GPIOId::F => 22,
        };

        unsafe {
            register.set_bit(bit_pos)?;
        }

        Ok(())
    }
}
