mod button;
mod led;

use crate::mcu::{GPIO, GPIOId};
use super::mcu::Port;

pub use button::Button;
pub use led::Led;

#[derive(Debug)]
pub enum BoardError {
    InvalidGPIO,
    ResourceTaken,
}
pub struct Board {
    ports: [Option<Port>; 2],
}

impl Board {
    pub fn take_port(&mut self, port: GPIOId, pin: u8) -> Result<Port, BoardError> {
        let idx = match (port, pin) {
            // to save memory, match only all the permutations you actually need
            (GPIOId::A, 8) => 0,
            (GPIOId::B, 13) => 1,
            _ => {
                return Err(BoardError::InvalidGPIO);
            }
        };

        if self.ports[idx].is_none() {
            return Err(BoardError::ResourceTaken);
        }
        Ok(self.ports[idx].take().unwrap())
    }
}

pub static mut BOARD: Board = Board {
    // to save memory, match only all the permutations you actually need
    ports: [
        Some(Port {
            gpio: GPIO {
                id: GPIOId::A,
                pin: 8,
            },
        }),
        Some(Port {
            gpio: GPIO {
                id: GPIOId::A,
                pin: 13,
            },
        }),
    ],
};

