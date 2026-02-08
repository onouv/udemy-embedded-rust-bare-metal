mod port;
mod register;
mod rcc;

pub use port::*;

//===============================================
// Device-specific constants, so these can be
// adapted to other MCU in one place easier
//===============================================

mod gpio_addresses {
    // AHB Peripheral Clock Enable Register
    pub const RCC_BASE_ADDR: u32 = 0x4002_1000;
    pub const RCC_AHBENR_OFFSET: u32 = 0x14;

    // GPIO port control registers
    pub const GPIO_BASE_ADDR: u32 = 0x4800_0000;
    pub const GPIO_MODER_OFFSET: u32 = 0;
    pub const GPIO_OTYPER_OFFSET: u32 = 0x4;
    pub const GPIO_SPEEDR_OFFSER: u32 = 0x8;
    pub const GPIO_PUPDR_OFFSET: u32 = 0xC;
    pub const GPIO_IDR_OFFSET: u32 = 0x10;
    pub const GPIO_ODR_OFFSET: u32 = 0x14;
    pub const GPIO_BSRR_OFFSET: u32 = 0x18;
}

const NUM_PINS: usize = 16;
const NUM_GPIOX: usize = 6;
pub struct MCU {
    // static-safe flag matrix
    // 896 usize :-0 for the fixed-sized associative array (abandoned)
    //      ports: FnvIndexMap<GpioId, FnvIndexMap<u8, Option<()>, NUM_PINS>, NUM_GPIOX>,
    // 96 usize :-) but be REALLY careful when indexing this
    ports: [[Option<()>; NUM_PINS]; NUM_GPIOX],
}

impl MCU {
    pub fn take_input(&mut self, gpio: &GpioId, pin: u8) -> Result<DisabledInput, MCUError> {
        let (g, p): (usize, usize) = get_port_indices(gpio, pin)?;
        // check the flag matrix and mark as taken, if not already taken
        unsafe {
            let mut port = self.ports[p][g];
            if port.is_none() {
                return Err(MCUError::ResourceTaken);
            }
            port.take();
        }

        port::new_input(gpio, pin)
    }

    pub fn take_output(&mut self, gpio: &GpioId, pin: u8) -> Result<DisabledOutput, MCUError> {
        let (g, p): (usize, usize) = get_port_indices(gpio, pin)?;

        unsafe {
            let mut port = self.ports[g][p];
            if port.is_none() {
                return Err(MCUError::ResourceTaken);
            }
            port.take();
        }

        port::new_output(gpio, pin)
    }
}

pub static mut MCU: MCU = MCU {
    ports: [[Some(()); NUM_PINS]; NUM_GPIOX],
};

fn get_port_indices(gpio: &GpioId, pin: u8) -> Result<(usize, usize), MCUError> {
    match (gpio, pin) {
        // Note the indices for gpio are 0-based,
        // while the pin numbers are 1-based.
        // This is intentional to simplify indexing into the flag matrix.
        (GpioId::A, 0) => Ok((0, 0)),
        (GpioId::A, 1) => Ok((0, 1)),
        (GpioId::A, 2) => Ok((0, 2)),
        (GpioId::E, 0) => Ok((4, 0)),
        (GpioId::E, 1) => Ok((4, 1)),
        (GpioId::E, 2) => Ok((4, 2)),
        (GpioId::E, 3) => Ok((4, 3)),
        (GpioId::E, 4) => Ok((4, 4)),
        (GpioId::E, 4) => Ok((4, 4)),
        (GpioId::E, 5) => Ok((4, 5)),
        (GpioId::E, 6) => Ok((4, 6)),
        (GpioId::E, 7) => Ok((4, 7)),
        (GpioId::E, 8) => Ok((4, 8)),
        (GpioId::E, 9) => Ok((4, 9)),
        (GpioId::E, 10) => Ok((4, 10)),
        (GpioId::E, 11) => Ok((4, 11)),
        (GpioId::E, 12) => Ok((4, 12)),
        (GpioId::E, 13) => Ok((4, 13)),
        (GpioId::E, 14) => Ok((4, 14)),
        (GpioId::E, 15) => Ok((4, 15)),
        _ => Err(MCUError::ResourceUnsupported),
    }
}

//===============================================
// Common types
//===============================================

pub enum Level {
    High,
    Low,
}

pub type Address = *mut u32;

#[derive(Debug)]
pub enum MCUError {
    InvalidGPIO,
    InvalidOffset,
    ResourceTaken,
    ResourceUnsupported,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GpioId {
    A,
    B,
    C,
    D,
    E,
    F,
}
