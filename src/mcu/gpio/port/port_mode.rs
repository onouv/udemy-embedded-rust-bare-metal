use crate::mcu::{GPIO, MCUError, Register, bitwise::Bitwise};

#[derive(Clone, Copy)]
pub enum PortMode {
    Output,
    Input,
    AlternateFunction,
    AnalogMode,
}

impl Bitwise for PortMode {
    fn as_bit_pattern(&self) -> (u32, u32) {
        match self {
            PortMode::Input => (0x00, 2),
            PortMode::Output => (0x01, 2),
            PortMode::AlternateFunction => (0x10, 2),
            PortMode::AnalogMode => (0x11, 2),
        }
    }
}

/**
 * Set the GPIOMode for a given port. This is a shared utility with a
 * private default implementation.
 */
pub(crate) trait PortModeUtil {
    unsafe fn set_mode(
        &self,
        moder: &Register,
        gpio: GPIO,
        mode: PortMode,
    ) -> Result<(), MCUError> {
        let (bits, bit_len) = PortMode::Input.as_bit_pattern();
        let offset_in_reg = gpio.pin * bit_len;
        unsafe {
            moder.set_bits(bits, offset_in_reg, bit_len)?;
        }

        Ok(())
    }
}
