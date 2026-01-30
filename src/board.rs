pub mod led;
pub mod button;

use button::Button;
use led::Led;
use crate::mcu::{ gpio::{self, GPIOPort, GPIO}, rrc::PeripheralClock  };

pub enum BoardErrorCode {
    ResourceTaken
}

pub struct Board {
    rcc: Option<PeripheralClock>,
    gpios: [Option<GPIOPort>; 2],
} 

impl Board {
    
    pub fn take_rcc(&mut self) -> Result<PeripheralClock, BoardErrorCode> {
        if self.rcc.is_none() {
            return Err(BoardErrorCode::ResourceTaken);
        }

        Ok(self.rcc.take().unwrap())
    }

    fn take_port(&mut self, id: GPIO) -> Result<GPIOPort, BoardErrorCode> {
        let mut gopt: Option<GPIOPort>;
        let mut idx: usize = 0;

        while idx < self.gpios.len() {
            let p = &self.gpios[idx];
            if let Some(port) = p.as_ref() {
                if port.id() == id {
                    return Ok(self.gpios[idx].take().unwrap());
                }
            }
        }

        Err(BoardErrorCode::ResourceTaken)
        
    }
}


static mut BOARD: Board = Board {
    rcc: Some(PeripheralClock::new()),
    gpios: [Some(GPIOPort::new(GPIO::A, 0)), Some(GPIOPort::new(GPIO::E, 8))],
};

//======================================================
//  GPIOE
//======================================================
pub const PE8: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 8,
};

pub const PE9: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 9,
};

pub const PE10: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 10,
};

pub const PE11: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 11,
};

pub const PE12: GPIO = GPIO {
    port: GPIOPort::A,
    pin: 12,
};

pub const PE13: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 13,
};

pub const PE14: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 14,
};

pub const PE15: GPIO = GPIO {
    port: GPIOPort::E,
    pin: 15,
};


//======================================================
//  GPIOA
//======================================================

pub const PA0: GPIO = GPIO {
    port: GPIOPort::A,
    pin: 0
};


//======================================================
// USER LED 
//======================================================

/** Red LED connected to I/O PE9 */
pub const LD3: Led = Led {
   io: PE9
};

/** Blue LED connected to I/O PE8 */
pub const LD4: Led = Led {
   io: PE8
};

/** Orange LED connected to I/O PE10 */
pub const LD5: Led = Led {
   io: PE10
};

/** Green LED connected to I/O PE15 */
pub const LD6: Led = Led {
   io: PE15
};

/** Green LED connected to I/O PE11 */
pub const LD7: Led = Led {
   io: PE11
};

/** Orange LED connected to I/O PE14 */
pub const LD8: Led = Led {
   io: PE14
};

/** Blue LED connected to I/O PE12 */
pub const LD9: Led = Led {
   io: PE12
};

/** Red LED connected to I/O PE13 */
pub const LD10: Led = Led {
   io: PE13
};


//======================================================
// Buttons 
//======================================================

/** User button on the discovery board */
pub const B1_USER: Button = Button {
    port: PA0 
};