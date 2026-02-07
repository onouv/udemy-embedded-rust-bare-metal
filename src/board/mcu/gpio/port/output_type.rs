use crate::mcu::{GPIO, MCUError, Register, bitwise::Bitwise};

#[derive(Clone, Copy)]
pub enum OutputType {
    PushPull,
    OpenDrain,
}

impl Bitwise for OutputType {
    fn as_bit_pattern(&self) -> (u32, u32) {
        match self {
            OutputType::OpenDrain => (0x1, 0x1),
            OutputType::PushPull => (0x0, 0x1),
        }
    }
}

pub(crate) trait OutputTypeUtil {
    unsafe fn set_output_type(
        otyper: &Register,
        gpio: GPIO,
        otype: OutputType,
    ) -> Result<(), MCUError> {
        let (bits, bit_len) = otype.as_bit_pattern();
        let offset_in_reg = gpio.pin * bit_len;
        unsafe {
            otyper.set_bits(bits, offset_in_reg, bit_len)?;
        }

        Ok(())
    }
}
