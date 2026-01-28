mod gpio_bits;
pub mod gpio_mode;
pub mod gpio_output_type;
pub mod gpio_pin_state;
pub mod gpio_port;

pub use gpio_port::*;

use gpio_bits::GPIOBits;
use gpio_mode::*;
use gpio_output_type::*;
use gpio_pin_state::*;

use super::error::*;
use super::register::{self, RegisterAddress};
use crate::utils::bits;

// Offsets onto port x base address for GPIOx control registers (x = A to D)
const GPIO_MODER_OFFSET: u32 = 0; // port mode register (RM3016 11.4.1)
const GPIO_OTYPER_OFFSET: u32 = 0x04; // port output type register (RM3016 11.4.2)
const GPIO_OSPEEDR_OFFSET: u32 = 0x08; // port output speed register (RM3016 11.4.3)
const GPIO_PUPDR_OFFSET: u32 = 0x0C; // port output pull-up/pull-down register (RM3016 11.4.4)

// Offsets onto port x base address for GPIOx data registers (x = A to D)
const GPIO_IDR_OFFSET: u32 = 0x10; // input data register (RM0316 11.4.5)
const GPIO_ODR_OFFSET: u32 = 0x14; // output data register (RM0316 11.4.6)
const GPIO_BSRR_OFFSET: u32 = 0x18; // port bit set/reset register (RM0316 11.4.7)

#[derive(Clone, Copy)]
#[allow(clippy::upper_case_acronyms)] // term from STM32 reference manual 
pub struct GPIO {
    pub port: GPIOPort, // TODO: these are public, only so GPIO can be const (use a new method from the startup code instead)
    pub pin: u32,
}

impl GPIO {
    pub fn new(port: GPIOPort, pin: u32) -> Result<Self, MCUErrorCode> {
        if pin > 15 {
            return Err(MCU_ERR_INVALID_PIN);
        }

        Ok(Self { port, pin })
    }

    pub unsafe fn set_pin_mode(&self, mode: GPIOMode) -> Result<(), MCUErrorCode> {
        let gpio_moder_addr =
            (self.port.as_ahb2_base_address() as u32 + GPIO_MODER_OFFSET) as RegisterAddress;
        let bit_len = GPIOMode::bit_len();
        let pin_bit_position: u32 = self.pin * bit_len;
        let mode_value: u32 = mode.as_bit_value() << pin_bit_position;

        unsafe { register::set_bits(gpio_moder_addr, mode_value, pin_bit_position, bit_len) }
    }

    pub unsafe fn set_pin_output_type(&self, otype: GPIOOutputType) -> Result<(), MCUErrorCode> {
        let gpio_otyper_addr: RegisterAddress =
            (self.port.as_ahb2_base_address() as u32 + GPIO_OTYPER_OFFSET) as RegisterAddress;
        let otype_value: u32 = otype.as_bit_value() << self.pin;

        unsafe {
            register::set_bits(
                gpio_otyper_addr,
                otype_value,
                self.pin,
                GPIOOutputType::bit_len(),
            )
        }
    }

    pub unsafe fn set_pin_state(&self, request: GPIOPinStateRequest) -> Result<(), MCUErrorCode> {
        // High bits in upper BSRR word RESET corresponding ODR bits
        // GPIOx_BSRR [31:16] 0000 0100 0000 0000 0000 0000 0000 0000
        // GPIOx_ODR  [15:0]  0100 0100 0000 0000 1000 0000 0000 0000
        //                          |                |
        // GPIOx_BSRR [31:16] 0000 0100 0000 0000 0001 0000 0000 0000
        //                          |                |
        // GPIOx_ODR  [15:0]  0100 0000 0000 0000 1000 0000 0000 0000

        // High bits in lower BSRR word SET corresponding ODR bits
        // GPIOx_BSRR [15:0]  0000 0100 0000 0000 0000 0000 0000 0000
        // GPIOx_ODR  [15:0]  0100 0100 0000 0000 1000 0000 0000 0000
        //                          |                |
        // GPIOx_BSRR [15:0]  0000 0100 0000 0000 0001 0000 0000 0000
        //                          |                |
        // GPIOx_ODR  [15:0]  0100 0100 0000 0000 1001 0000 0000 0000

        unsafe {
            match request {
                GPIOPinStateRequest::Set(state) => {
                    let gpio_bsrr_addr = (self.port.as_ahb2_base_address() as u32
                        + GPIO_BSRR_OFFSET)
                        as RegisterAddress;
                    let gpio_bsrr_value = match state {
                        GPIOPinState::High => state.as_bit_value() << self.pin,
                        GPIOPinState::Low => state.as_bit_value() << (self.pin + 16),
                    };
                    register::write(gpio_bsrr_addr, state.as_bit_value() << self.pin);
                }

                GPIOPinStateRequest::Toggle => {
                    let gpio_bsrr_addr = (self.port.as_ahb2_base_address() as u32
                        + GPIO_BSRR_OFFSET)
                        as RegisterAddress;
                    let mut gpio_odr_value = register::read(gpio_bsrr_addr);
                    let gpio_pin_mask = GPIOPinState::bit_mask() << self.pin;
                    let pin_bit = bits::get(gpio_odr_value, gpio_pin_mask, self.pin);

                    match pin_bit {
                        Ok(bit) => {
                            if bit == 0x1 {
                                // RESET pin in GPIOx_BSRR [31:16]
                                let address = (gpio_bsrr_addr as u32 + 16_u32) as RegisterAddress;
                                return register::set_bits(address, 1, self.pin, 1);
                            } else {
                                // SET pin bit in GPIO_BSRR [15:00]
                                return register::set_bits(gpio_bsrr_addr, 1, self.pin, 1);
                            }
                        }
                        Err(()) => {
                            return Err(MCU_ERR_INVALID_PIN as MCUErrorCode);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub unsafe fn enable_clock(&self) -> Result<(), MCUErrorCode> {
        unsafe {
            register::set_bit(
                self.port.as_ahb2_base_address(),
                self.port.as_rcc_ahbenr_bitpos(),
                true,
            )?;
        }

        Ok(())
    }
}
