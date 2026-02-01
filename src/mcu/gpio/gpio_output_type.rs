use crate::mcu::bitwise::Bitwise;

#[derive(Clone, Copy)]
pub enum GPIOOutputType {
    PushPull,
    OpenDrain,
}

impl Bitwise for GPIOOutputType {
    fn as_bit_pattern(&self) -> (u32, u32) {
        match self {
            GPIOOutputType::OpenDrain => (0x1, 0x1),
            GPIOOutputType::PushPull => (0x0, 0x1),
        }
    }
}