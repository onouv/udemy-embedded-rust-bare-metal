use crate::mcu;
use crate::led::Led;

pub const BLUE_LED: Led = Led { port: mcu::GPIOPortName::A, pin: 1 };

