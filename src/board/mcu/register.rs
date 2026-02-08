use super::{Address, GpioId, MCUError, gpio_addresses::*};
use core::ptr;

pub trait GpioRegister {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError>;
    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError>;
    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError>;

    // TODO: there should be a reset() function
}

pub trait Register {
    fn set_bit(&self, bit_pos: u8) -> Result<(), MCUError>;
    fn clear_bit(&self, bit_pos: u8) -> Result<(), MCUError>;
    fn set_bits(&self, val: u32, offset: u32, val_bit_len: u32) -> Result<(), MCUError>;
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct RCC_AHBENR; // AHB Peripheral Clock Enable Register

impl Register for RCC_AHBENR {
    fn set_bit(&self, bit_pos: u8) -> Result<(), MCUError> {
        let address = (RCC_BASE_ADDR + RCC_AHBENR_OFFSET) as Address;
        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, bit_pos: u8) -> Result<(), MCUError> {
        let address = (RCC_BASE_ADDR + RCC_AHBENR_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (RCC_BASE_ADDR + RCC_AHBENR_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct GPIOx_MODER; // GPIO port mode register

impl GpioRegister for GPIOx_MODER {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_MODER_OFFSET) as Address;

        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_MODER_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_MODER_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct GPIOx_OTYPER; // GPIO port mode register

impl GpioRegister for GPIOx_OTYPER {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_OTYPER_OFFSET) as Address;

        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_OTYPER_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_OTYPER_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct GPIOx_PUPDR; // GPIO port mode register

impl GpioRegister for GPIOx_PUPDR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_PUPDR_OFFSET) as Address;

        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_PUPDR_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_PUPDR_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct GPIOx_IDR; // GPIO port mode register

impl GpioRegister for GPIOx_IDR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_IDR_OFFSET) as Address;

        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_IDR_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_IDR_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct GPIOx_ODR; // GPIO port mode register

impl GpioRegister for GPIOx_ODR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_ODR_OFFSET) as Address;

        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_ODR_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_ODR_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

#[allow(non_camel_case_types)] // term from STM32 reference manual
pub struct GPIOx_BSRR; // GPIO port mode register

impl GpioRegister for GPIOx_BSRR {
    fn set_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_BSRR_OFFSET) as Address;

        set_bit(address, bit_pos)
    }

    fn clear_bit(&self, gpio: GpioId, bit_pos: u8) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_BSRR_OFFSET) as Address;

        clear_bit(address, bit_pos)
    }

    fn set_bits(
        &self,
        gpio: GpioId,
        val: u32,
        offset: u32,
        val_bit_len: u32,
    ) -> Result<(), MCUError> {
        let address = (GPIO_BASE_ADDR + calc_gpio_offset(&gpio) + GPIO_BSRR_OFFSET) as Address;

        set_bits(address, val, offset, val_bit_len)
    }
}

fn clear_bit(address: Address, bit_pos: u8) -> Result<(), MCUError> {
    if bit_pos > 31 {
        return Err(MCUError::InvalidOffset);
    }

    unsafe {
        let reg_val = ptr::read_volatile(address);
        let update_val = reg_val & !(1 << bit_pos);
        ptr::write_volatile(address, update_val);
    }

    Ok(())
}

fn set_bit(address: Address, bit_pos: u8) -> Result<(), MCUError> {
    if bit_pos > 31 {
        return Err(MCUError::InvalidOffset);
    }

    unsafe {
        let reg_val = ptr::read_volatile(address);
        let update_val = reg_val | (1 << bit_pos);
        ptr::write_volatile(address, update_val);
    }

    Ok(())
}

fn set_bits(
    address: Address,
    value: u32,
    offset: u32,
    value_bit_length: u32,
) -> Result<(), MCUError> {
    if offset > 31 {
        return Err(MCUError::InvalidOffset);
    }

    if !(1..=32).contains(&value_bit_length) {
        return Err(MCUError::InvalidOffset);
    }

    unsafe {
        let reg_value = ptr::read_volatile(address);
        let mask = ((1 << value_bit_length) - 1) << offset;

        // Clear the relevant bits in the register and set the new value
        let update = (reg_value & !mask) | ((value << offset) & mask);

        ptr::write_volatile(address, update);
    }

    Ok(())
}

fn calc_gpio_offset(gpio: &GpioId) -> u32 {
    match gpio {
        GpioId::A => 0,
        GpioId::B => 0x400,
        GpioId::C => 0x800,
        GpioId::D => 0xC00,
        GpioId::E => 0x1000,
        GpioId::F => 0x1400,
    }
}
