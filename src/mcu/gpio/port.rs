mod input_port;
mod output_port;
mod port_mode_util;
mod output_type_util;

use crate::mcu::{
    MCUError, 
    gpio::{
        GPIO, GPIOId, port::{
            input_port::InputPort, 
            output_port::OutputPort,
        }
    }
};

#[derive(Clone, Copy)]
pub struct Port {
    pub gpio: GPIO,
}

impl Port {
    pub fn to_input(self) -> Result<InputPort, MCUError> {
        InputPort::new(self.gpio)
    }

    pub fn to_output(self) -> Result<OutputPort, MCUError> {
        OutputPort::new(self.gpio)
    }
}
