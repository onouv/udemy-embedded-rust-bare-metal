use super::{ GpioId, MCUError, register::{ RCC_AHBENR, Register}};


/** 
 * RCC related utility functions with private default implementation.
 */
pub(super) trait RCCUtil {
    fn enable_peripheral_clock(&self, port: GpioId) -> Result<(), MCUError> {
        let bit_pos = match port {
            GpioId::A => 17,
            GpioId::B => 18,
            GpioId::C => 19,
            GpioId::D => 20,
            GpioId::E => 21,
            GpioId::F => 22,
        };

        unsafe {
            RCC_AHBENR.set_bit(bit_pos)?;
        }

        Ok(())
    }
}
