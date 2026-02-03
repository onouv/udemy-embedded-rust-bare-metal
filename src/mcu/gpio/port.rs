mod input_port;
mod output_port;
mod output_type;
mod port_mode;

use super::super::{
    MCUError,
    gpio::{GPIO, GPIOId},
};

pub(crate) use input_port::InputPort;
pub(crate) use output_port::OutputPort;
pub(crate) use output_type::{OutputType, OutputTypeUtil};
pub(crate) use port_mode::{PortMode, PortModeUtil};

#[derive(Clone, Copy)]
pub(crate) struct Port {
    pub gpio: GPIO,
}

impl Port {
    pub fn to_input(self) -> InputPort {
        InputPort::new(self.gpio)
    }

    pub fn to_output(self) -> OutputPort {
        OutputPort::new(self.gpio)
    }
}
