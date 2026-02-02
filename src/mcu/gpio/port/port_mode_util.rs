use crate::mcu::{
    MCUError,
    bitwise::Bitwise,
    gpio::{GPIO, gpio_mode::GPIOMode},
    register::Register,
};

/**
 * Set the GPIOMode for a given port. This is a shared utility with a
 * private default implementation.
 */
pub(super) trait PortModeUtil {
    unsafe fn set_mode(
        &self,
        moder: &Register,
        gpio: GPIO,
        mode: GPIOMode,
    ) -> Result<(), MCUError> {
        let (bits, bit_len) = GPIOMode::Input.as_bit_pattern();
        let offset_in_reg = gpio.pin * bit_len;
        unsafe {
            moder.set_bits(bits, offset_in_reg, bit_len)?;
        }

        Ok(())
    }
}
