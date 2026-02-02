use crate::mcu::{
    FAKE_ADDR, GPIOA_BSRR_ADDR, GPIOA_MODER_ADDR, GPIOA_ODR_ADDR, GPIOA_OTYPER_ADDR, MCUError,
    gpio::{
        GPIO, GPIOId, gpio_mode::GPIOMode, gpio_output_type::GPIOOutputType,
        port::port_mode_util::PortModeUtil,
    },
    rcc::RCCUtil,
    register::{Address, Register},
};

pub struct OutputPort {
    gpio: GPIO,
    mode_reg: Register,
    outp_type_reg: Register,
    outp_data_reg: Register,
    bit_set_reset_reg: Register,
}

impl RCCUtil for OutputPort {}

impl PortModeUtil for OutputPort {}

impl OutputPort {
    /**
     * Create a new OutputPort
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
            _ => FAKE_ADDR as Address, // fake for demo
        };

        let odr_addr = match gpio.id {
            GPIOId::A => GPIOA_ODR_ADDR,
            _ => FAKE_ADDR, // fake for demo
        };

        let otyper_addr = match gpio.id {
            GPIOId::A => GPIOA_OTYPER_ADDR,
            _ => FAKE_ADDR,
        };

        let bsrr_addr = match gpio.id {
            GPIOId::A => GPIOA_BSRR_ADDR,
            _ => FAKE_ADDR,
        };

        let mode_reg = Register::new(moder_addr);
        let outp_data_reg = Register::new(odr_addr);
        let outp_type_reg = Register::new(otyper_addr);
        let bit_set_reset_reg = Register::new(bsrr_addr);

        if moder_addr == FAKE_ADDR
            || odr_addr == FAKE_ADDR
            || otyper_addr == FAKE_ADDR
            || bsrr_addr == FAKE_ADDR
        {
            return Err(MCUError::InvalidGPIO);
        }

        Ok(Self {
            gpio,
            mode_reg,
            outp_type_reg,
            outp_data_reg,
            bit_set_reset_reg,
        })
    }

    /**
     * Initialize the port by manipulating the GPIO Port
     **/
    pub unsafe fn init(&self, otype: GPIOOutputType) -> Result<(), MCUError> {
        // println!("OutputPort {:?} does the bitwise reg manips to init for '{:?}'", self.gpio, otype);
        unsafe {
            self.enable_peripheral_clock(self.gpio.id)?;
            self.set_mode(&self.mode_reg, self.gpio, GPIOMode::Input)?;
        }

        Ok(())
    }
}
