use crate::mcu::bitwise::Bitwise;

#[derive(Clone, Copy)]
pub enum GPIOMode {
    Output,
    Input,
    AlternateFunction,
    AnalogMode,
}

impl Bitwise for GPIOMode {
    fn as_bit_pattern(&self) -> (u32, u32) {
        match self {
            GPIOMode::Input => (0x00, 2),
            GPIOMode::Output => (0x01, 2),
            GPIOMode::AlternateFunction => (0x10, 2),
            GPIOMode::AnalogMode => (0x11, 2)
        }
    }
}