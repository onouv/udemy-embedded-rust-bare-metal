use crate::board::mcu::{MCUError, rcc::RCCUtil};

use super::{GpioId, register::*};

pub struct Port<Direction, PinMode, OutputType, RegisterBlock> {
    gpio: GpioId,
    pin: u8,
    direction: Direction,
    pin_mode: PinMode,
    otype: OutputType,
    registers: RegisterBlock,
}

pub type DisabledInput = Port<DirInput, DontCare, DontCare, InputRegisterBlock>;
pub type InputPullDown = Port<DirInput, PinPulledDown, DontCare, InputRegisterBlock>;
pub type InputPullUp = Port<DirInput, PinPulledUp, DontCare, InputRegisterBlock>;
pub type InputFloating = Port<DirInput, PinFloating, DontCare, InputRegisterBlock>;

pub type DisabledOutput = Port<DirOutput, DontCare, DontCare, OutputRegisterBlock>;
pub type OutputOpenDrain = Port<DirOutput, DontCare, OTypeOpenDrain, OutputRegisterBlock>;
pub type OutputPushPullPulledUp = Port<DirOutput, PinPulledUp, OTypePushPull, OutputRegisterBlock>;
pub type OutputPushPullPulledDown = Port<DirOutput, PinPulledDown, OTypePushPull, OutputRegisterBlock>;
pub type OutputOpenDrainPulledUp = Port<DirOutput, PinPulledUp, OTypeOpenDrain, OutputRegisterBlock>;
pub type OutputOpenDrainPulledDown = Port<DirOutput, PinPulledDown, OTypeOpenDrain, OutputRegisterBlock>;

pub struct DirInput;
pub struct DirOutput;
pub struct DontCare;
pub struct OTypePushPull;
pub struct OTypeOpenDrain;
pub struct PinPulledUp;
pub struct PinPulledDown;
pub struct PinFloating;

// Marker trait to indicate the input pin mode is configured (not `DontCare`).
pub trait ConfiguredInput {}
impl ConfiguredInput for PinPulledUp {}
impl ConfiguredInput for PinPulledDown {}
impl ConfiguredInput for PinFloating {}

// marker trait to indicate output pins are configured (not `DontCare`).
pub trait ConfiguredOutput {}
impl ConfiguredOutput for OTypePushPull {}
impl ConfiguredOutput for OTypeOpenDrain {}
impl ConfiguredOutput for PinPulledUp {}
impl ConfiguredOutput for PinPulledDown {}

pub fn new_output(gpio: &GpioId, pin: u8) -> Result<DisabledOutput, MCUError> {

    let port_mode: u32 = 0b01; // general purpose output
    let offset: u32 = (pin * 2) as u32;
    let port = Port {
        gpio: *gpio,
        pin,
        direction: DirOutput,
        pin_mode: DontCare,
        otype: DontCare,
        registers: OutputRegisterBlock::new(),
    };

    port.registers.moder.set_bits(port.gpio, port_mode, offset, 2)?;

    Ok(port)
}

pub type OutputPushPull = Port<DirOutput, DontCare, OTypePushPull, OutputRegisterBlock>;

impl RCCUtil for OutputPushPull {}

impl<OTYPE> Port<DirOutput, DontCare, OTYPE, OutputRegisterBlock> {

    pub fn into_pushpull(self) -> Result<OutputPushPull, MCUError> {

        self.registers.otyper.clear_bit(self.gpio, self.pin)?;

        let port: OutputPushPull = Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: DirOutput,
            pin_mode: DontCare,
            otype: OTypePushPull,
            registers: self.registers,
        };

        port.enable_peripheral_clock(self.gpio)?;

        Ok(port)
    }
    
    pub fn into_open_drain(self) -> Result<OutputPushPull, MCUError> {
        
        self.registers.otyper.set_bit(self.gpio, self.pin)?;

        Ok(Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: DirOutput,
            pin_mode: DontCare,
            otype: OTypePushPull,
            registers: OutputRegisterBlock::new()
        })
    }
}
    
impl<PINMOD> Port<DirOutput, PINMOD, OTypePushPull, OutputRegisterBlock> {
    pub fn into_pulled_down(self) -> Result<OutputPushPullPulledDown, MCUError> {
        // see Reference Manual sect. 11.4.4 
        let pin_mode: u32 = 0b10;
        let offset: u32 = (self.pin * 2) as u32; 
        self.registers.pupdr.set_bits(self.gpio, pin_mode, offset, 2)?;

        Ok(Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: DirOutput,
            pin_mode: PinPulledDown,
            otype: OTypePushPull,
            registers: self.registers,
        })
    }

    pub fn into_pulled_up(self) -> Result<OutputPushPullPulledUp, MCUError> {
        // see Reference Manual sect. 11.4.4
        let pin_mode: u32 = 0b01;
        let offset: u32 = (self.pin * 2) as u32; 
        self.registers.pupdr.set_bits(self.gpio, pin_mode, offset, 2)?;

        Ok(Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: DirOutput,
            pin_mode: PinPulledUp,
            otype: OTypePushPull,
            registers: self.registers,
        })
    }
}

impl<PINMOD> Port<DirOutput, PINMOD, OTypeOpenDrain, OutputRegisterBlock> {
    pub fn into_pulled_down(self) -> Result<OutputOpenDrainPulledDown, MCUError> {

        // see Reference Manual sect. 11.4.4 
        let pin_mode: u32 = 0b10;
        let offset: u32 = (self.pin * 2) as u32; 
        self.registers.pupdr.set_bits(self.gpio, pin_mode, offset, 2)?;

        Ok(Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: DirOutput,
            pin_mode: PinPulledDown,
            otype: OTypeOpenDrain,
            registers: self.registers,
        })
    }

    pub fn into_pulled_up(self) -> Result<OutputOpenDrainPulledUp, MCUError> {
        
        // see Reference Manual sect. 11.4.4
        let pin_mode: u32 = 0b01;
        let offset: u32 = (self.pin * 2) as u32; 
        self.registers.pupdr.set_bits(self.gpio, pin_mode, offset, 2)?;

        Ok(Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: DirOutput,
            pin_mode: PinPulledUp,
            otype: OTypeOpenDrain,
            registers: self.registers,
        })
    }
}

pub fn new_input(gpio: &GpioId, pin: u8) -> Result<DisabledInput, MCUError> {
    
    let port_mode: u32 = 0b00; // input state 
    let offset: u32 = (pin * 2) as u32;
    
    let port = Port {
        direction: DirInput,
        pin_mode: DontCare,
        otype: DontCare,
        registers: InputRegisterBlock::new(),
        pin,
        gpio: *gpio,
    };    
    port.registers.moder.set_bits(port.gpio, port_mode, offset, 2)?;

    Ok(port)
}

impl<PINMOD> Port<DirInput, PINMOD, DontCare, InputRegisterBlock> {
    pub fn into_floating(self) -> Result<InputFloating, MCUError> {
        // see Reference Manual sect. 11.4.4
        let pin_mode: u32 = 0b00; // no pull-up, pull-down
        let offset: u32 = (self.pin * 2) as u32; 
        self.registers.pupdr.set_bits(self.gpio, pin_mode, offset, 2)?;
        
        Ok(Port {
            direction: DirInput,
            pin_mode: PinFloating,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        })
    }

    pub fn into_pulled_up(self) -> Result<InputPullUp, MCUError> {
        
        Ok(Port {
            direction: DirInput,
            pin_mode: PinPulledUp,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        })
    }

    pub fn into_pulled_down(self) -> InputPullDown {
        Port {
            direction: DirInput,
            pin_mode: PinPulledDown,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        }
    }
}

impl<PINMOD: ConfiguredInput> Port<DirInput, PINMOD, DontCare, InputRegisterBlock> {
    pub fn pin_is_high(&self) -> bool {
        // Read the input data register for the configured pin.
        // Actual register-read logic not yet implemented; keep placeholder.
        true
    }
}

impl <PINMOD, OTYPE: ConfiguredOutput> Port<DirOutput, PINMOD, OTYPE, OutputRegisterBlock> {
    pub fn set_high(&self) {
        // set pin bit in BS[pin] GPIOx_BSRR[15:0] 
        self.registers.bsrr.set_bit(self.gpio, self.pin);
    }

    pub fn set_low(&self) {
        // set pin bit in BR[pin] GPIOx_BSRR[31:0] 
        self.registers.bsrr.set_bit(self.gpio, self.pin + 16);
    }
}



pub struct InputRegisterBlock {
    ahbenr: super::register::RCC_AHBENR,
    moder: GPIOx_MODER,
    otyper: GPIOx_OTYPER,
    pupdr: GPIOx_PUPDR,
    idr: GPIOx_IDR,
}

impl InputRegisterBlock {
    pub fn new() -> Self {
        Self {
            ahbenr: RCC_AHBENR {},
            moder: GPIOx_MODER {},
            otyper: GPIOx_OTYPER {},
            pupdr: GPIOx_PUPDR,
            idr: GPIOx_IDR {},
        }
    }
}

pub struct OutputRegisterBlock {
    ahbenr: RCC_AHBENR,
    moder: GPIOx_MODER,
    otyper: GPIOx_OTYPER,
    pupdr: GPIOx_PUPDR,
    odr: GPIOx_ODR,
    bsrr: GPIOx_BSRR,
}

impl OutputRegisterBlock {
    pub fn new() -> Self {
        Self {
            ahbenr: RCC_AHBENR,
            moder: GPIOx_MODER,
            otyper: GPIOx_OTYPER,
            pupdr: GPIOx_PUPDR,
            odr: GPIOx_ODR,
            bsrr: GPIOx_BSRR,
        }
    }
}