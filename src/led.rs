use crate::mcu::*;

#[derive(Clone, Copy)]
pub struct Led {
    pub port: GPIOPortName,
    pub pin: u32,
}

impl Led {
    pub fn new(port: GPIOPortName, pin: u32) -> Self {
        Self { port, pin }
    }

    pub unsafe fn init(&self) {
        unsafe {
            gpio_set_pin_mode(self.port, self.pin, GPIOMode::Output);
            gpio_set_output_type(self.port, self.pin, GPIOOutputType::PushPull);
        }
    }

    pub fn on(&self) {
        // Implementation to turn the LED on
    }

    pub fn off(&self) {
        // Implementation to turn the LED off
    }

    pub fn toggle(&self) {
        // Implementation to toggle the LED state
    }
}
