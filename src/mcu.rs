use core::ptr;

pub type MCUErrorCode = u32;
pub const MCU_ERR_INVALID_PIN: MCUErrorCode = 0;
pub const MCU_ERR_INVALID_PORTNAME: MCUErrorCode = 1;

// Offsets for GPIOx control registers
const GPIO_MODER_OFFSET: u32 = 0;
const GPIO_OTYPER_OFFSET: u32 = 0x04;
const GPIO_OSPEEDR_OFFSET: u32 = 0x08;
const GPIO_PUPDR_OFFSET: u32 = 0x0C;

pub unsafe fn read_register(address: *const u32) -> u32 {
    unsafe { ptr::read_volatile(address) }
}

pub unsafe fn write_register(address: *mut u32, value: u32) {
    unsafe {
        ptr::write_volatile(address, value);
    }
}

#[derive(Clone, Copy)]
pub enum GPIOPortName {
    A, B, C, D
}

impl GPIOPortName {

    /**
     * Translate to a valid base address as per RM0316, Table 4
     */
    pub fn to_base_address(&self) -> Result<u32, MCUErrorCode> {
        let gpio_base: u32 = match self {
            GPIOPortName::A => 0x48000000,
            GPIOPortName::B => 0x48000400,
            GPIOPortName::C => 0x48000800,
            GPIOPortName::D => 0x48000C00,
            _ => return Err(MCU_ERR_INVALID_PORTNAME)
        };

        Ok(gpio_base)
    }
}
 

pub enum GPIOPinMode {
    Output,
    Input,
}

impl GPIOPinMode {
    fn to_bit_mask(&self) -> u32 {
        match self {
            GPIOPinMode::Input => 0x00,
            GPIOPinMode::Output => 0x01
        }
    }
    fn bit_size() -> u32 {
        2 // each MODER(pin) has 2 bits
    }
}

pub enum GPIOOutputType {
    PushPull,
    OpenDrain,
    Disabled
}

pub unsafe fn gpio_set_output_type(port: u32, pin: u32, otype: GPIOOutputType) {

}

pub unsafe fn gpio_set_pin_mode(port: GPIOPortName, pin: u32, mode: GPIOPinMode) -> Result<(), MCUErrorCode> {
    if pin > 15 {
        return Err(MCU_ERR_INVALID_PIN);
    }

    let gpio_moder_addr = (port.to_base_address()? + GPIO_MODER_OFFSET as u32) as *mut u32;
    let pin_bit_position: u32 = pin * GPIOPinMode::bit_size(); 
    let mode_value: u32 =  mode.to_bit_mask() << pin_bit_position; 
    let mode_mask:  u32 = 0x03 << pin_bit_position;

    unsafe {
        let mut gpio_moder_value = read_register(gpio_moder_addr);
        gpio_moder_value = clear_bits(gpio_moder_value, mode_mask);
        gpio_moder_value = set_bits(gpio_moder_value, mode_value);
        write_register(gpio_moder_addr, gpio_moder_value);
    }

    Ok(())
}

fn clear_bits(value: u32, mask: u32) -> u32 {
    value & !mask
}

fn set_bits(value: u32, mask: u32) -> u32 {
    value | mask
}