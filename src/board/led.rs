use crate::mcu::gpio::{
    self, GPIO, GPIOMode, GPIOOutputType, GPIOPinState, GPIOPinStateRequest,
};

#[derive(Clone, Copy)]
pub struct Led {
    pub io: GPIO,  // TODO: this is public, only so GPIO can be const (use a new method from the startup code instead)

}

impl Led {
    pub fn new(io: GPIO) -> Self {
        Self { io }
    }

    pub unsafe fn init(&self) {
        unsafe {
            self.io.set_pin_mode(GPIOMode::Output);
            self.io.set_pin_output_type(GPIOOutputType::PushPull);
        }
    }

    pub fn on(&self) {
        unsafe {
            self.io
                .set_pin_state(GPIOPinStateRequest::Set(GPIOPinState::High));
        }
    }

    pub fn off(&self) {
        unsafe {
            self.io
                .set_pin_state(GPIOPinStateRequest::Set(GPIOPinState::Low));
        }
    }

    pub fn toggle(&self) {
        unsafe {
            self.io.set_pin_state(GPIOPinStateRequest::Toggle);
        }
    }
}
