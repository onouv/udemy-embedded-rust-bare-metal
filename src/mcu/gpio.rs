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

        Ok(Self {
            port,
            pin,
        })
    }

    pub unsafe fn set_pin_mode(&self, mode: GPIOMode) -> Result<(), MCUErrorCode> {
        let gpio_moder_addr = (self.port.as_base_address() + GPIO_MODER_OFFSET) as RegisterAddress;
        let bit_len = GPIOMode::bit_len();
        let pin_bit_position: u32 = self.pin * bit_len;
        let mode_value: u32 = mode.to_bit_value() << pin_bit_position;

        unsafe { register::set_bits(gpio_moder_addr, mode_value, pin_bit_position, bit_len) }
    }

    pub unsafe fn set_pin_output_type(&self, otype: GPIOOutputType) -> Result<(), MCUErrorCode> {
        let gpio_otyper_addr: RegisterAddress =
            (self.port.as_base_address() + GPIO_OTYPER_OFFSET) as RegisterAddress;
        let otype_value: u32 = otype.to_bit_value() << self.pin;

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
                    let gpio_bsrr_addr =
                        (self.port.as_base_address() + GPIO_BSRR_OFFSET) as RegisterAddress;
                    let gpio_bsrr_value = match state {
                        GPIOPinState::High => state.to_bit_value() << self.pin,
                        GPIOPinState::Low => state.to_bit_value() << (self.pin + 16),
                    };
                    register::write(gpio_bsrr_addr, state.to_bit_value() << self.pin);
                }

                GPIOPinStateRequest::Toggle => {
                    let gpio_bsrr_addr =
                        (self.port.as_base_address() + GPIO_BSRR_OFFSET) as RegisterAddress;
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
}

#[derive(Clone, Copy)]
pub enum GPIOPort {
    A,
    B,
    C,
    D,
}

impl GPIOPort {
    /**
     * Translate to a valid base address as per RM0316, Table 4
     */
    pub fn as_base_address(&self) -> u32 {
        match self {
            GPIOPort::A => 0x48000000,
            GPIOPort::B => 0x48000400,
            GPIOPort::C => 0x48000800,
            GPIOPort::D => 0x48000C00,
        }
    }
}

trait GPIOBits {
    fn to_bit_value(&self) -> u32;

    // overwrite this, if the bits must take more than one bit
    fn bit_mask() -> u32 {
        0x1
    }

    // overwrite this, if the bits must take more than one bit
    fn bit_len() -> u32 {
        0x1
    }
}

pub enum GPIOMode {
    Output,
    Input,
    AlternateFunction,
    AnalogMode,
}

impl GPIOBits for GPIOMode {
    fn to_bit_value(&self) -> u32 {
        match self {
            GPIOMode::Input => 0x00,
            GPIOMode::Output => 0x01,
            GPIOMode::AlternateFunction => 0x10,
            GPIOMode::AnalogMode => 0x11,
        }
    }

    fn bit_mask() -> u32 {
        0x03
    }

    fn bit_len() -> u32 {
        2 // each MODER(pin) has 2 bits
    }
}

pub enum GPIOOutputType {
    PushPull,
    OpenDrain,
}

impl GPIOBits for GPIOOutputType {
    fn to_bit_value(&self) -> u32 {
        match self {
            GPIOOutputType::OpenDrain => 0x1,
            GPIOOutputType::PushPull => 0x0,
        }
    }
}

pub enum GPIOPinState {
    High,
    Low,
}

impl GPIOBits for GPIOPinState {
    fn to_bit_value(&self) -> u32 {
        match self {
            GPIOPinState::High => 0x01,
            GPIOPinState::Low => 0x00,
        }
    }
}

pub enum GPIOPinStateRequest {
    Set(GPIOPinState),
    Toggle,
}
