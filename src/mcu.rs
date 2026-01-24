use core::ptr;

pub type MCUErrorCode = u32;
pub const MCU_ERR_INVALID_PIN: MCUErrorCode = 0;
pub const MCU_ERR_INVALID_PORTNAME: MCUErrorCode = 1;

// Offsets for GPIOx control registers
const GPIO_MODER_OFFSET: u32 = 0;
const GPIO_OTYPER_OFFSET: u32 = 0x04;
const GPIO_OSPEEDR_OFFSET: u32 = 0x08;
const GPIO_PUPDR_OFFSET: u32 = 0x0C;


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
 
trait GPIOBits {
    fn to_bit_value(&self) -> u32;
    fn bit_mask(&self) -> u32;
    fn bit_size() -> u32;    
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

    fn bit_mask(&self) -> u32 {
        0x03
    }

    fn bit_size() -> u32 {
        2 // each MODER(pin) has 2 bits
    }
}

pub unsafe fn gpio_set_pin_mode(port: GPIOPortName, pin_no: u32, mode: GPIOMode) -> Result<(), MCUErrorCode> {
    if pin_no > 15 {
        return Err(MCU_ERR_INVALID_PIN);
    }

    let gpio_moder_addr = (port.to_base_address()? + GPIO_MODER_OFFSET as u32) as *mut u32;
    let pin_bit_position: u32 = pin_no * GPIOMode::bit_size(); 
    let mode_value: u32 =  mode.to_bit_value() << pin_bit_position; 
    let mode_mask:  u32 = 0x03 << pin_bit_position;

    unsafe {
        let mut gpio_moder_value = read_register(gpio_moder_addr);
        gpio_moder_value = clear_bits(gpio_moder_value, mode_mask);
        gpio_moder_value = set_bits(gpio_moder_value, mode_value);
        write_register(gpio_moder_addr, gpio_moder_value);
    }

    Ok(())
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

    fn bit_mask(&self) -> u32 {
        0x1
    }

    fn bit_size() -> u32 {
        0x1
    }
}

pub unsafe fn gpio_set_output_type(port: GPIOPortName, pin_no: u32, otype: GPIOOutputType) -> Result<(), MCUErrorCode> {

    if pin_no > 15 {
        return Err(MCU_ERR_INVALID_PIN);
    }

    let gpio_otyper_addr: *mut u32 = (port.to_base_address()? + GPIO_OTYPER_OFFSET) as *mut u32;
    let otype_value: u32 = otype.to_bit_value() << pin_no;
    let otyper_mask: u32 = otype.bit_mask() << pin_no;

    unsafe {
        let otyper_value_old: u32 = read_register(gpio_otyper_addr);
        let otyper_value_masked: u32 = clear_bits(otyper_value_old, otyper_mask);
        let otyper_value_new: u32 = set_bits(otyper_value_masked, otype_value);
        write_register(gpio_otyper_addr, otyper_value_new);
    }    

    Ok(())
}


fn clear_bits(value: u32, mask: u32) -> u32 {
    let new_value = value & !mask;

    new_value
}

fn set_bits(value: u32, mask: u32) -> u32 {
    let new_value = value | mask;

    new_value
}

unsafe fn read_register(address: *const u32) -> u32 {
    unsafe { ptr::read_volatile(address) }
}

unsafe fn write_register(address: *mut u32, value: u32) {
    unsafe {
        ptr::write_volatile(address, value);
    }
}
