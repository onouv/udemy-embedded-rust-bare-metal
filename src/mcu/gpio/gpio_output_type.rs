use super::gpio_bits::GPIOBits;

pub enum GPIOOutputType {
    PushPull,
    OpenDrain,
}

impl GPIOBits for GPIOOutputType {
    fn as_bit_value(&self) -> u32 {
        match self {
            GPIOOutputType::OpenDrain => 0x1,
            GPIOOutputType::PushPull => 0x0,
        }
    }
}