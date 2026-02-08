use super::mcu::OutputPushPull;


pub struct Led {
    port: OutputPushPull,
}

impl Led {

    pub fn new(pin: OutputPushPull) -> Self {
        Self {
            port: pin
        }
    }

    pub fn on(&self) {
        self.port.set_high();
    }

    pub fn off(&self) {
        self.port.set_low();
    }

    pub fn toggle(&self) {
        todo!();
    }
}
