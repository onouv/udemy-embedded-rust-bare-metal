mod bitwise;
mod gpio;
mod rcc;
mod register;

pub(crate) use gpio::{InputPort, OutputPort, Port, OutputType };
pub(crate) use gpio::{GPIO, GPIOId};
pub(crate) use register::Register;

//===============================================
// Device-specific constants, so these can be
// adapted to other MCU in one place easier
//===============================================

// AHB Peripheral Clock Enable Register
pub const RCC_BASE_ADDR: u32 = 0x4002_1000;
pub const RCC_AHBENR_ADDR: Address = (RCC_BASE_ADDR + 0x14) as Address;

mod gpio_addresses {
    use super::Address;
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
    pub const GPIOB_MODER_ADDR: Address = GPIOB_BASE_ADDR; // port mode register
    pub const GPIOB_OTYPER_ADDR: Address = 0x4800_0404 as Address;
    pub const GPIOB_OSPEEDR_ADDR: Address = 0x4800_0408 as Address; // output speed register
    pub const GPIOB_PUPDR_ADDR: Address = 0x4800_040C as Address; // pull-up/-down register
    pub const GPIOB_IDR_ADDR: Address = 0x4800_0410 as Address; // input data register    
    pub const GPIOB_ODR_ADDR: Address = 0x4800_0414 as Address; // output data register
    pub const GPIOB_BSRR_ADDR: Address = 0x4800_0418 as Address; // bit set/reset register


    pub const GPIOC_BASE_ADDR: Address = 0x4800_0800 as Address;
    pub const GPIOC_MODER_ADDR: Address = GPIOC_BASE_ADDR; // port mode register
    pub const GPIOC_OTYPER_ADDR: Address = 0x4800_0804 as Address;
    pub const GPIOC_OSPEEDR_ADDR: Address = 0x4800_0808 as Address; // output speed register
    pub const GPIOC_PUPDR_ADDR: Address = 0x4800_080C as Address; // pull-up/-down register
    pub const GPIOC_IDR_ADDR: Address = 0x4800_0810 as Address; // input data register    
    pub const GPIOC_ODR_ADDR: Address = 0x4800_0814 as Address; // output data register
    pub const GPIOC_BSRR_ADDR: Address = 0x4800_0818 as Address; // bit set/reset register


    pub const GPIOD_BASE_ADDR: Address = 0x4800_0C00 as Address;
    pub const GPIOD_MODER_ADDR: Address = GPIOD_BASE_ADDR; // port mode register
    pub const GPIOD_OTYPER_ADDR: Address = 0x4800_0C04 as Address;
    pub const GPIOD_OSPEEDR_ADDR: Address = 0x4800_0C08 as Address; // output speed register
    pub const GPIOD_PUPDR_ADDR: Address = 0x4800_0C0C as Address; // pull-up/-down register
    pub const GPIOD_IDR_ADDR: Address = 0x4800_0C10 as Address; // input data register    
    pub const GPIOD_ODR_ADDR: Address = 0x4800_0C14 as Address; // output data register
    pub const GPIOD_BSRR_ADDR: Address = 0x4800_0C18 as Address; // bit set/reset register

    pub const GPIOE_BASE_ADDR: Address = 0x4800_1000 as Address;
    pub const GPIOE_MODER_ADDR: Address = GPIOE_BASE_ADDR; // port mode register
    pub const GPIOE_OTYPER_ADDR: Address = 0x4800_1004 as Address; // output type register
    pub const GPIOE_OSPEEDR_ADDR: Address = 0x4800_1008 as Address; // output speed register
    pub const GPIOE_PUPDR_ADDR: Address = 0x4800_100C as Address; // pull-up/-down register
    pub const GPIOE_IDR_ADDR: Address = 0x4800_1010 as Address; // input data register    
    pub const GPIOE_ODR_ADDR: Address = 0x4800_1014 as Address; // output data register
    pub const GPIOE_BSRR_ADDR: Address = 0x4800_1018 as Address; // bit set/reset register

    pub const GPIOF_BASE_ADDR: Address = 0x4800_1400 as Address;
    pub const GPIOF_MODER_ADDR: Address = GPIOF_BASE_ADDR; // port mode register
    pub const GPIOF_OTYPER_ADDR: Address = 0x4800_1404 as Address; // output type register
    pub const GPIOF_OSPEEDR_ADDR: Address = 0x4800_1408 as Address; // output speed register
    pub const GPIOF_PUPDR_ADDR: Address = 0x4800_140C as Address; // pull-up/-down register
    pub const GPIOF_IDR_ADDR: Address = 0x4800_1410 as Address; // input data register    
    pub const GPIOF_ODR_ADDR: Address = 0x4800_1414 as Address; // output data register
    pub const GPIOF_BSRR_ADDR: Address = 0x4800_1418 as Address; // bit set/reset register

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
    PortPreviouslyTaken,
}
