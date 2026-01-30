use crate::mcu::register::RegisterAddress;

pub mod error;
pub mod register;
pub mod gpio;
pub mod rrc;

//
// Device-specific constants, so these can be adapted to other MCU in one place easier
//

// AHB Peripheral Clock Enable Register
pub const RCC_BASE_ADDR: RegisterAddress = 0x4002_1000 as RegisterAddress;
pub const RCC_AHBENR_ADDR: RegisterAddress = (RCC_BASE_ADDR + 0x14) as RegisterAddress;


//
// GPIO Control Registers 
//
pub const GPIOA_BASE_ADDR: RegisterAddress = 0x48000000 as RegisterAddress;
pub const GPIOA_MODER_ADDR: RegisterAddress = GPIOA_BASE_ADDR; // port mode register
pub const GPIOA_OTYPER_ADDR: RegisterAddress = GPIOA_BASE_ADDR + 0x04; // output type register
pub const GPIOA_OSPEEDR_ADDR: RegisterAddress = GPIOA_BASE_ADDR + 0x08; // output speed register
pub const GPIOA_PUPDR_ADDR: RegisterAddress = GPIOA_BASE_ADDR + 0x0C; // pull-up/-down register
pub const GPIOA_IDR_ADDR: RegisterAddress = GPIOA_BASE_ADDR + 0x10; // input data register    
pub const GPIOA_ODR_ADDR: RegisterAddress = GPIOA_BASE_ADDR + 0x014; // output data register
pub const GPIOA_BSRR_ADDR: RegisterAddress = GPIOA_BASE_ADDR + 0x018; // bit set/reset register

pub const GPIOB_BASE_ADDR: RegisterAddress = 0x48000400 as RegisterAddress;
pub const GPIOB_MODER_ADDR: RegisterAddress = GPIOB_BASE_ADDR; // port mode register
pub const GPIOB_OTYPER_ADDR: RegisterAddress = GPIOB_BASE_ADDR + 0x04; // output type register
pub const GPIOB_OSPEEDR_ADDR: RegisterAddress = GPIOB_BASE_ADDR + 0x08; // output speed register
pub const GPIOB_PUPDR_ADDR: RegisterAddress = GPIOB_BASE_ADDR + 0x0C; // pull-up/-down register
pub const GPIOB_IDR_ADDR: RegisterAddress = GPIOB_BASE_ADDR + 0x10; // input data register    
pub const GPIOB_ODR_ADDR: RegisterAddress = GPIOB_BASE_ADDR + 0x014; // output data register
pub const GPIOB_BSRR_ADDR: RegisterAddress = GPIOB_BASE_ADDR + 0x018; // bit set/reset register

pub const GPIOC_BASE_ADDR: RegisterAddress = 0x48000800 as RegisterAddress;

pub const GPIOD_BASE_ADDR: RegisterAddress = 0x48000C00 as RegisterAddress;

pub const GPIOE_BASE_ADDR: RegisterAddress = 0x48001000 as RegisterAddress;
pub const GPIOE_MODER_ADDR: RegisterAddress = GPIOE_BASE_ADDR; // port mode register
pub const GPIOE_OTYPER_ADDR: RegisterAddress = GPIOE_BASE_ADDR + 0x04; // output type register
pub const GPIOE_OSPEEDR_ADDR: RegisterAddress = GPIOE_BASE_ADDR + 0x08; // output speed register
pub const GPIOE_PUPDR_ADDR: RegisterAddress = GPIOE_BASE_ADDR + 0x0C; // pull-up/-down register
pub const GPIOE_IDR_ADDR: RegisterAddress = GPIOE_BASE_ADDR + 0x10; // input data register    
pub const GPIOE_ODR_ADDR: RegisterAddress = GPIOE_BASE_ADDR + 0x014; // output data register
pub const GPIOE_BSRR_ADDR: RegisterAddress = GPIOE_BASE_ADDR + 0x018; // bit set/reset register

pub const GPIOF_BASE_ADDR: RegisterAddress = 0x48001400 as RegisterAddress;


//
// Common types
//

pub enum Level {
    High,
    Low
}

