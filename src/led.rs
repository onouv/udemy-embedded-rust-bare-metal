use crate::mcu::{self, GPIOPinMode};

#[derive(Clone, Copy)]
pub struct Led {
    pub port: mcu::GPIOPortName,
    pub pin: u32,
}

impl Led {
    pub fn new(port: mcu::GPIOPortName, pin: u32) -> Self {
        Self {
            port,
            pin
        }
    }
}
pub unsafe fn init(led: &Led){
    unsafe {
        mcu::gpio_set_pin_mode(led.port, led.pin, GPIOPinMode::Output);
    }
}

pub fn on(led: &Led) {
    // Implementation to turn the LED on
}

pub fn off(led: &Led) {
    // Implementation to turn the LED off
}

pub fn toggle(led: &Led) {
    // Implementation to toggle the LED state
}


