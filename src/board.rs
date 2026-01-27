pub mod led;
pub mod button;

use button::Button;
use led::Led;
use crate::mcu::gpio::GPIOPortName;


// TODO: make these thread safe 

pub const BLUE_LED: Led = Led {
    port: GPIOPortName::A,
    pin: 8,
};

pub const USER_BUTTON: Button = Button {
    port: GPIOPortName::A,
    pin: 0,
};