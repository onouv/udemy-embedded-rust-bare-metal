mod button;
mod led;
mod mcu;

pub use button::Button;
pub use led::Led;

use crate::board::mcu::{DisabledOutput, GpioId, MCU, MCUError};

#[derive(Debug)]
pub enum BoardError {
    ResourceTaken,
    ResourceUnsupported,
}

pub struct Board;

impl Board {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn take_led(&self, id: u8) -> Result<Led, BoardError> {
        match get_port(id) {
            Ok(p) => match p.into_pushpull() {
                Ok(port) => {
                    let led = Led::new(port);
                    Ok(led)
                }
                _ => {
                    Err(BoardError::ResourceUnsupported)
                }
            },
            Err(MCUError::ResourceTaken) => Err(BoardError::ResourceTaken),
            _ => Err(BoardError::ResourceUnsupported),
        }
    }
}

fn get_port(led: u8) -> Result<DisabledOutput, MCUError> {
    unsafe {
        #[allow(static_mut_refs)] // MCU implements a singleton pattern for the GPIO
        match led {
            3 => MCU.take_output(&GpioId::E, 9),
            4 => MCU.take_output(&GpioId::E, 8),
            5 => MCU.take_output(&GpioId::E, 10),
            6 => MCU.take_output(&GpioId::E, 15),
            7 => MCU.take_output(&GpioId::E, 11),
            8 => MCU.take_output(&GpioId::E, 14),
            9 => MCU.take_output(&GpioId::E, 12),
            10 => MCU.take_output(&GpioId::E, 13),
            _ => Err(MCUError::ResourceUnsupported),
        }
    }
}

pub static mut BOARD: Board = Board {
    // Initialize the board with all resources available.
};
