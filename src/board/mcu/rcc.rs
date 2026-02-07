use super::gpio::GpioId;
use super::{
    RCC_AHBENR_ADDR, MCUError, register::{Address, Register}
};

/** 
 * RCC related utility functions with private default implementation.
 */
pub(super) trait RCCUtil {
    unsafe fn enable_peripheral_clock(&self, port: GpioId) -> Result<(), MCUError> {
        let register: Register = Register::new(RCC_AHBENR_ADDR);
        let bit_pos = match port {
            GpioId::A => 17,
            GpioId::B => 18,
            GpioId::C => 19,
            GpioId::D => 20,
            GpioId::E => 21,
            GpioId::F => 22,
        };

        unsafe {
            register.set_bit(bit_pos)?;
        }

        Ok(())
    }
}
