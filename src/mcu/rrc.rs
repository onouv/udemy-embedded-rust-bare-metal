use super::gpio::{GPIO, GPIOPort};
use crate::mcu::error::MCU_ERR_INVALID_PIN;

#[derive(Clone, Copy)]
pub struct PeripheralClock<'a> {
    // TODO: use an associative data structure, map. etc.
    // TODO: create via new(..) in startup routine
    pub gpios: [Option<&'a GPIO<'a>>; 6],
}

impl PeripheralClock<'_> {
    pub fn attach(&self, gpio: &GPIO) -> Result<(), MCUErrorCode> {
        if self.gpio_pre_registered(gpio.port) {
            return Err(MCU_ERR_INVALID_PIN);
        }

        self.gpios[self.get_index(gpio.port)] = Some(gpio);

        Ok(())
    }

    fn gpio_pre_registered(&self, port: GPIOPort) -> bool {
        match self.gpios[self.get_index(port)] {
            Some(_) => true,
            None() => false,
        }
    }

    fn get_index(&self, port: GPIOPort) -> usize {
        match gpio.port {
            GPIOPort::A => 0,
            GPIOPort::B => 1,
            GPIOPort::C => 2,
            GPIOPort::D => 3,
            GPIOPort::E => 4,
            GPIOPort::F => 5,
        }
    }

    pub fn enable(port: GPIOPort) -> Result<(), MCUErrorCode> {
        // TODO: write into GPIOx_AHBENR

        Ok(())
    }
}
