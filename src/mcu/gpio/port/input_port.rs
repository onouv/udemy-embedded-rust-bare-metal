use super::PortModeUtil;
use super::{GPIO, GPIOId};
use crate::mcu::{
    MCUError, gpio::port::PortMode, gpio_addresses::*, rcc::RCCUtil, register::Register,
};

pub(crate) struct InputPort {
    pub(crate) gpio: GPIO,
    pub(crate) mode_reg: Register,
    pub(crate) inp_data_reg: Register,
}

impl RCCUtil for InputPort {}
impl PortModeUtil for InputPort {}
impl InputPort {
    /**
     * Create a new InputPort
     *
     * Safely setup all data and addresses needed to initialize
     * a GPIO port later. Note this is the only way to create
     * an OutputPort.
     *
     */
    pub fn new(gpio: GPIO) -> Self {
        let moder_addr = match gpio.id {
            GPIOId::A => GPIOA_MODER_ADDR,
            GPIOId::B => GPIOB_MODER_ADDR,
            GPIOId::C => GPIOC_MODER_ADDR,
            GPIOId::D => GPIOD_MODER_ADDR,
            GPIOId::E => GPIOE_MODER_ADDR,
            GPIOId::F => GPIOF_MODER_ADDR,
        };

        let idr_addr = match gpio.id {
            GPIOId::A => GPIOA_IDR_ADDR,
            GPIOId::B => GPIOB_IDR_ADDR,
            GPIOId::C => GPIOC_IDR_ADDR,
            GPIOId::D => GPIOC_IDR_ADDR,
            GPIOId::E => GPIOC_IDR_ADDR,
            GPIOId::F => GPIOF_IDR_ADDR,
        };

        let mode_reg = Register::new(moder_addr);
        let inp_data_reg = Register::new(idr_addr);

        Self {
            gpio,
            mode_reg,
            inp_data_reg,
        }
    }

    /**
     * Initialize the port by manipulating the appropriate MCU registers
     **/
    pub unsafe fn init(&self) -> Result<(), MCUError> {
        unsafe {
            self.enable_peripheral_clock(self.gpio.id)?;
            self.set_mode(&self.mode_reg, self.gpio, PortMode::Input)?
        }

        Ok(())
    }
}
