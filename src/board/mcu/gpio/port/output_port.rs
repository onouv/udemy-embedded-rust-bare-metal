use crate::mcu::{
    MCUError,
    gpio::{
        GPIO, GpioId,
        port::{OutputType, OutputTypeUtil, PortMode, PortModeUtil},
    },
    gpio_addresses::*,
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
     */
    pub fn new(gpio: GPIO) -> Self {
        let moder_addr = match gpio.id {
            GpioId::A => GPIOA_MODER_ADDR,
            GpioId::B => GPIOB_MODER_ADDR,
            GpioId::C => GPIOC_MODER_ADDR,
            GpioId::D => GPIOC_MODER_ADDR,
            GpioId::E => GPIOC_MODER_ADDR,
            GpioId::F => GPIOF_MODER_ADDR,
        };

        let odr_addr = match gpio.id {
            GpioId::A => GPIOA_ODR_ADDR,
            GpioId::B => GPIOB_ODR_ADDR,
            GpioId::C => GPIOC_ODR_ADDR,
            GpioId::D => GPIOC_ODR_ADDR,
            GpioId::E => GPIOC_ODR_ADDR,
            GpioId::F => GPIOF_ODR_ADDR,
        };

        let idr_addr = match gpio.id {
            GpioId::A => GPIOA_IDR_ADDR,
            GpioId::B => GPIOB_IDR_ADDR,
            GpioId::C => GPIOC_IDR_ADDR,
            GpioId::D => GPIOC_IDR_ADDR,
            GpioId::E => GPIOC_IDR_ADDR,
            GpioId::F => GPIOF_IDR_ADDR,
        };
        let otyper_addr = match gpio.id {
            GpioId::A => GPIOA_OTYPER_ADDR,
            GpioId::B => GPIOB_OTYPER_ADDR,
            GpioId::C => GPIOC_OTYPER_ADDR,
            GpioId::D => GPIOC_OTYPER_ADDR,
            GpioId::E => GPIOC_OTYPER_ADDR,
            GpioId::F => GPIOF_OTYPER_ADDR,
        };

        let bsrr_addr = match gpio.id {
            GpioId::A => GPIOA_BSRR_ADDR,
            GpioId::B => GPIOB_BSRR_ADDR,
            GpioId::C => GPIOC_BSRR_ADDR,
            GpioId::D => GPIOC_BSRR_ADDR,
            GpioId::E => GPIOC_BSRR_ADDR,
            GpioId::F => GPIOF_BSRR_ADDR,
        };

        let mode_reg = Register::new(moder_addr);
        let outp_data_reg = Register::new(odr_addr);
        let outp_type_reg = Register::new(otyper_addr);
        let bit_set_reset_reg = Register::new(bsrr_addr);

        Self {
            gpio,
            mode_reg,
            outp_type_reg,
            outp_data_reg,
            bit_set_reset_reg,
        }
    }

    /**
     * Initialize the port by manipulating the GPIO Port
     **/
    pub unsafe fn init(&self, otype: OutputType) -> Result<(), MCUError> {
        // println!("OutputPort {:?} does the bitwise reg manips to init for '{:?}'", self.gpio, otype);
        unsafe {
            self.enable_peripheral_clock(self.gpio.id)?;
            self.set_mode(&self.mode_reg, self.gpio, PortMode::Input)?;
        }

        Ok(())
    }

    pub fn pin_high(&self, pin: u32) -> Result<(), BoardError> {
        self.bit_set_reset_reg.    }

}
