use crate::mcu::GPIOPortName;

pub enum ButtonName {
    User,
    Reset,
}

#[derive(Clone, Copy)]
pub struct Button {
    pub port: GPIOPortName,
    pub pin: u32,
}

impl Button {
    pub fn init(&self) {}
}
