mod input_port;
mod output_port;
mod port_mode_util;
mod output_type_util;

use crate::mcu::{
    MCUError, 
    gpio::{
        GPIO, 
        GPIOId
    }
};

pub(crate) use input_port::InputPort;
pub(crate) use output_port::OutputPort;

#[derive(Clone, Copy)]
pub(crate) struct Port {
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
