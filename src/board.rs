pub mod led;
pub mod button;

use button::Button;
use led::Led;
use crate::mcu::gpio::{ GPIO, GPIOPort };


// TODO: make these thread safe

pub const PA8: GPIO = GPIO {
    port: GPIOPort::A,
    pin: 8,
};

pub const PC13: GPIO = GPIO {
    port: GPIOPort::C,
    pin: 13
};

pub const BLUE_LED: Led = Led {
   io: PA8
};

pub const USER_BTN: Button = Button {
    port: PC13 
};