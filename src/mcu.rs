mod gpio;
mod rcc;
mod register;
mod bitwise;

pub(crate) use gpio::{GPIO, GPIOId};
pub(crate) use gpio::port::{Port, InputPort, OutputPort};

//===============================================
// Device-specific constants, so these can be 
// adapted to other MCU in one place easier
//===============================================

// AHB Peripheral Clock Enable Register
pub const RCC_BASE_ADDR: u32 = 0x4002_1000;
pub const RCC_AHBENR_ADDR: Address = (RCC_BASE_ADDR + 0x14 ) as Address;


//
// GPIO Control Registers 
//
pub const GPIOA_BASE_ADDR: Address = 0x4800_0000 as Address;
pub const GPIOA_MODER_ADDR: Address = GPIOA_BASE_ADDR; // port mode register
pub const GPIOA_OTYPER_ADDR: Address = 0x4800_0004 as Address; 
pub const GPIOA_OSPEEDR_ADDR: Address = 0x4800_0008 as Address; // output speed register
pub const GPIOA_PUPDR_ADDR: Address = 0x4800_000C as Address; // pull-up/-down register
pub const GPIOA_IDR_ADDR: Address = 0x4800_0010 as Address; // input data register    
pub const GPIOA_ODR_ADDR: Address = 0x4800_0014 as Address; // output data register
pub const GPIOA_BSRR_ADDR: Address = 0x4800_0018 as Address; // bit set/reset register

pub const GPIOB_BASE_ADDR: Address = 0x4800_0400 as Address;

pub const GPIOC_BASE_ADDR: Address = 0x4800_0800 as Address;

pub const GPIOD_BASE_ADDR: Address = 0x4800_0C00 as Address;

pub const GPIOE_BASE_ADDR: Address = 0x4800_1000 as Address;
pub const GPIOE_MODER_ADDR: Address = GPIOE_BASE_ADDR; // port mode register
pub const GPIOE_OTYPER_ADDR: Address = 0x4800_1004 as Address; // output type register
pub const GPIOE_OSPEEDR_ADDR: Address = 0x4800_1008 as Address; // output speed register
pub const GPIOE_PUPDR_ADDR: Address = 0x4800_100C as Address; // pull-up/-down register
pub const GPIOE_IDR_ADDR: Address = 0x4800_1010 as Address; // input data register    
pub const GPIOE_ODR_ADDR: Address = 0x4800_1014 as Address; // output data register
pub const GPIOE_BSRR_ADDR: Address = 0x4800_0018 as Address; // bit set/reset register

pub const GPIOF_BASE_ADDR: Address = 0x48001400 as Address;

//===============================================
// Common types
//===============================================

pub enum Level {
    High,
    Low
}

pub type Address = *mut u32;

const FAKE_ADDR: Address = 0xFFFF_FFFF as Address;

#[derive(Debug)]
pub enum MCUError {
    InvalidGPIO,
    InvalidOffset,
    PortPreviouslyTaken,
}

