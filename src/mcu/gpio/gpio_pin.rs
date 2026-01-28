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

pub struct GPIOPin {
    no: u32,
    state: GPIOState,
}

impl GPIOPin {
    pub fn new(no: u32, state: GPIOPinState) -> Self {
        Self { no, state }
    }

    pub fn to_low(self) -> Self {
        Self {
            no: self.no,
            state: GPIOPinState::Low,
        }
    }

    pub fn to_high(self) -> Self {
        Self {
            no: self.no,
            state: GPIOPinState::High,
        }
    }

    pub fn toggle(self) -> Self {
        Self {
            no: self.no,
            state: if self.state == GPIOPinState::High {
                GPIOPinState::Low
            } else {
                GPIOPinState::High
            },
        }
    }
}
