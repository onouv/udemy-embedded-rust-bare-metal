use crate::mcu::{
    error::MCUErrorCode,
    gpio::{GPIO, gpio_mode::GPIOMode, gpio_output_type::GPIOOutputType, gpio_pin_state::*},
};

#[derive(Clone, Copy)]
pub struct Led {
    pub io: GPIO, // TODO: this is public, only so GPIO can be const (use a new method from the startup code instead)
}

impl Led {
    pub const fn new(io: GPIO) -> Self {
        Self { io }
    }

    pub unsafe fn init(&self) -> Result<(), MCUErrorCode> {
        unsafe {
            self.io.enable_clock()?;
            self.io.set_pin_mode(GPIOMode::Output)?;
            self.io.set_pin_output_type(GPIOOutputType::PushPull)?;
        }

        Ok(())
    }

    pub fn on(&self) -> Result<(), MCUErrorCode> {
        unsafe {
            self.io
                .set_pin_state(GPIOPinStateRequest::Set(GPIOPinState::High))?;
        }

        Ok(())
    }

    pub fn off(&self) -> Result<(), MCUErrorCode> {
        unsafe {
            self.io
                .set_pin_state(GPIOPinStateRequest::Set(GPIOPinState::Low))?;
        }

        Ok(())
    }

    pub fn toggle(&self) -> Result<(), MCUErrorCode> {
        unsafe {
            self.io.set_pin_state(GPIOPinStateRequest::Toggle)?;
        }
        Ok(())
    }
}
