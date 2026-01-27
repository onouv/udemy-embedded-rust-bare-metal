use crate::mcu::gpio::{GPIO, GPIOPort};

pub enum ButtonName {
    User,
    Reset,
}

#[derive(Clone, Copy)]
pub struct Button {
    pub port: GPIO,
}

impl Button {
    pub fn init(&self) {}
}
