use super::gpio::{GPIO, GPIOPort};
use crate::mcu::error::{MCU_ERR_INVALID_PIN, MCU_ERR_PORT_UNREGISTERED};

#[derive(Clone, Copy)]
pub struct PeripheralClock {
    // TODO: use an associative data structure, map. etc.
    // TODO: create via new(..) in startup routine
    pub gpios: [Option<GPIOPort>; 6],
}

impl PeripheralClock {
    pub fn attach(&self, gpio: &GPIO) -> Result<(), MCUErrorCode> {
        if self.gpio_pre_registered(gpio.port) {
            return Err(MCU_ERR_INVALID_PIN);
        }

        self.gpios[self.get_index(gpio.port)] = Some(gpio.port);

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

    pub fn enable(&self, port: GPIOPort) -> Result<(), MCUErrorCode> {
        let p = self.gpios[self.get_index(port)];

        if p.is_none() {
            return Err(MCU_ERR_PORT_UNREGISTERED);
        }

        // TODO: write into GPIOx_AHBENR

        Ok(())
    }
}
