use super::gpio_bits::GPIOBits;

pub enum GPIOMode {
    Output,
    Input,
    AlternateFunction,
    AnalogMode,
}

impl GPIOBits for GPIOMode {
    fn as_bit_value(&self) -> u32 {
        match self {
            GPIOMode::Input => 0x00,
            GPIOMode::Output => 0x01,
            GPIOMode::AlternateFunction => 0x10,
            GPIOMode::AnalogMode => 0x11,
        }
    }

    fn bit_mask() -> u32 {
        0x03
    }

    fn bit_len() -> u32 {
        2 // each MODER(pin) has 2 bits
    }
}