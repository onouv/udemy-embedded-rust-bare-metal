use crate::mcu::gpio::*;

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
        unsafe {
            gpio_set_pin_state(
                self.port,
                self.pin,
                GPIOPinStateRequest::Set(GPIOPinState::High),
            );
        }
    }

    pub fn off(&self) {
        unsafe {
            gpio_set_pin_state(
                self.port,
                self.pin,
                GPIOPinStateRequest::Set(GPIOPinState::Low),
            );
        }
    }

    pub fn toggle(&self) {
        unsafe {
            gpio_set_pin_state(
                self.port,
                self.pin,
                GPIOPinStateRequest::Toggle
            );
        }
    }
}
