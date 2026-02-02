use crate::mcu::{MCUError, bitwise::Bitwise, gpio::{GPIO, gpio_output_type::GPIOOutputType}, register::Register};



pub(super) trait OutputTypeUtil {
    unsafe fn set_output_type(otyper: &Register, gpio: GPIO, otype: GPIOOutputType) -> Result<(), MCUError> {
        let (bits, bit_len) = otype.as_bit_pattern();
        let offset_in_reg = gpio.pin * bit_len;
        unsafe {
            otyper.set_bits(bits, offset_in_reg, bit_len)?;
        }
    
        Ok(())
    }
}