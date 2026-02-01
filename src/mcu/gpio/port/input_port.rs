use super::{GPIO, GPIOId};
use crate::mcu::{FAKE_ADDR, GPIOA_IDR_ADDR, GPIOA_MODER_ADDR, MCUError, gpio::gpio_mode::GPIOMode, rcc::RCCUtil, register::Register};
use super::port_mode_util::PortModeUtil;

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
     * Since we are only returning ports actually used in our
     * system, we must indicate request for unused ports as
     * Err(MCUError::InvalidPort)
     */
    pub fn new(gpio: GPIO) -> Result<Self, MCUError> {
        let moder_addr = match gpio.id {
            GPIOId::A => GPIOA_MODER_ADDR,
            _ => FAKE_ADDR,
        };

        let idr_addr = match gpio.id {
            GPIOId::A => GPIOA_IDR_ADDR,
            _ => FAKE_ADDR,
        };

        if moder_addr == FAKE_ADDR || idr_addr == FAKE_ADDR {
            return Err(MCUError::InvalidGPIO);
        }

        let mode_reg = Register::new(moder_addr);
        let inp_data_reg = Register::new(idr_addr);

        Ok(Self {
            gpio,
            mode_reg,
            inp_data_reg,
        })
    }

    /**
     * Initialize the port by manipulating the appropriate MCU registers
     **/
    pub unsafe fn init(&self) -> Result<(), MCUError> {
        unsafe {
            self.enable_peripheral_clock(self.gpio.id)?;
            self.set_mode(&self.mode_reg, self.gpio, GPIOMode::Input)?
        }

        Ok(())
    }
}
