use super::gpio::GPIO;
use crate::mcu::{
    RCC_AHBENR_ADDR, error::{MCU_ERR_INVALID_PIN, MCU_ERR_PORT_UNREGISTERED}, register::{Register, RegisterAddress}
};

//#[derive(Clone, Copy)]
pub struct PeripheralClock {
    ahbenr: Register,
}

impl PeripheralClock {
    pub const fn new() -> Self {
        Self { ahbenr: Register::new(RCC_AHBENR_ADDR) }
    }

    pub fn enable(&self, port: &GPIO) -> Result<(), MCUErrorCode> {
        // TODO: write into GPIOx_AHBENR
        let register: RegisterAddress = RCC_AHBENR_ADDR;
        let bit_pos = Self::get_rcc_ahbenr_bitpos(p);

        Ok(())
    }

    /**
     * Return bit position within the AHB peripheral clock enable register RCC_AHBENR
     */
    pub fn get_rcc_ahbenr_bitpos(port: &Port) -> u32 {
        match self {
            GPIOPort::A => 17,
            GPIOPort::B => 18,
            GPIOPort::C => 19,
            GPIOPort::D => 20,
            GPIOPort::E => 21,
            GPIOPort::F => 22,
        }
    }
}
