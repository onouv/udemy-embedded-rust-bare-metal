use super::gpio_bits::GPIOBits;

pub enum GPIOPinState {
    High,
    Low,
}

impl GPIOBits for GPIOPinState {
    fn as_bit_value(&self) -> u32 {
        match self {
            GPIOPinState::High => 0x01,
            GPIOPinState::Low => 0x00,
        }
    }
}

pub enum GPIOPinStateRequest {
    Set(GPIOPinState),
    Toggle,
}