use crate::button::Button;
use crate::led::Led;
use crate::mcu;

pub const BLUE_LED: Led = Led {
    port: mcu::GPIOPortName::A,
    pin: 8,
};

pub const USER_BUTTON: Button = Button {
    port: mcu::GPIOPortName::A,
    pin: 0,
};