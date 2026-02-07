use super::{GpioId, register::*};

pub struct Port<Direction, PinMode, OutputType, RegisterBlock> {
    gpio: GpioId,
    pin: u8,
    direction: Direction,
    pin_mode: PinMode,
    otype: OutputType,
    registers: RegisterBlock,
}

pub type DisabledInput = Port<Input, DontCare, DontCare, InputRegisterBlock>;
pub type InputPullDown = Port<Input, PinPullDown, DontCare, InputRegisterBlock>;
pub type InputPullUp = Port<Input, PinPullUp, DontCare, InputRegisterBlock>;
pub type InputFloating = Port<Input, PinFloating, DontCare, InputRegisterBlock>;

pub type DisabledOutput = Port<Output, DontCare, DontCare, OutputRegisterBlock>;
pub type OutputPushPullPullUp = Port<Output, PinPullUp, PushPull, OutputRegisterBlock>;
pub type OutputPushPullPullDown = Port<Output, PinPullDown, PushPull, OutputRegisterBlock>;
pub type OutputOpenDrainPullUp = Port<Output, PinPullUp, OpenDrain, OutputRegisterBlock>;
pub type OutputOpenDrainPullDown = Port<Output, PinPullDown, OpenDrain, OutputRegisterBlock>;

pub struct Input;
pub struct Output;
pub struct Enabled;
pub struct DontCare;
pub struct PushPull;
pub struct OpenDrain;
pub struct PinPullUp;
pub struct PinPullDown;
pub struct PinFloating;

// Marker trait to indicate the input pin mode is configured (not `DontCare`).
pub trait ConfiguredInput {}
impl ConfiguredInput for PinPullUp {}
impl ConfiguredInput for PinPullDown {}
impl ConfiguredInput for PinFloating {}

// marker trait to indicate output pins are configured (not `DontCare`).
pub trait ConfiguredOutput {}
impl ConfiguredOutput for PushPull {}
impl ConfiguredOutput for OpenDrain {}
impl ConfiguredOutput for PinPullUp {}
impl ConfiguredOutput for PinPullDown {}

enum PinMode {
    PullUp,
    PullDown,
    Floating,
}

enum OutputType {
    PushPull,
    OpenDrain,
}

pub fn new_output(gpio: &GpioId, pin: u8) -> DisabledOutput {
    Port {
        gpio: *gpio,
        pin,
        direction: Output,
        pin_mode: DontCare,
        otype: DontCare,
        registers: OutputRegisterBlock::new(),
    }
}

impl<PINMOD, OTYPE> Port<Output, PINMOD, OTYPE, OutputRegisterBlock> {
    pub fn into_pushpull_pulled_up(self) -> OutputPushPullPullUp {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullUp,
            otype: PushPull,
            registers: self.registers,
        }
    }

    pub fn into_pushpull_pulled_down(self) -> OutputPushPullPullDown {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullDown,
            otype: PushPull,
            registers: self.registers,
        }
    }

    pub fn into_open_drain_pull_up(self) -> OutputOpenDrainPullUp {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullUp,
            otype: OpenDrain,
            registers: self.registers,
        }
    }

    pub fn into_open_drain_pull_down(self) -> OutputOpenDrainPullDown {
        Port {
            gpio: self.gpio,
            pin: self.pin,
            direction: Output,
            pin_mode: PinPullDown,
            otype: OpenDrain,
            registers: self.registers,
        }
    }
}

pub fn new_input(gpio: &GpioId, pin: u8) -> DisabledInput {
    Port {
        direction: Input,
        pin_mode: DontCare,
        otype: DontCare,
        registers: InputRegisterBlock::new(),
        pin,
        gpio: *gpio,
    }
}

impl<PINMOD> Port<Input, PINMOD, DontCare, InputRegisterBlock> {
    pub fn into_floating(self) -> InputFloating {
        Port {
            direction: Input,
            pin_mode: PinFloating,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        }
    }

    pub fn into_pulled_up(self) -> InputPullUp {
        Port {
            direction: Input,
            pin_mode: PinPullUp,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        }
    }

    pub fn into_pulled_down(self) -> InputPullDown {
        Port {
            direction: Input,
            pin_mode: PinPullDown,
            otype: DontCare,
            registers: self.registers,
            gpio: self.gpio,
            pin: self.pin,
        }
    }
}

impl<PINMOD: ConfiguredInput> Port<Input, PINMOD, DontCare, InputRegisterBlock> {
    pub fn pin_is_high(&self) -> bool {
        // Read the input data register for the configured pin.
        // Actual register-read logic not yet implemented; keep placeholder.
        true
    }
}

impl<PINMOD, OTYPE: ConfiguredOutput> Port<Output, PINMOD, OTYPE, OutputRegisterBlock> {
    pub fn set_high(&self) {
        // Write to the output data register for the configured pin.
        // Actual register-write logic not yet implemented; keep placeholder.
    }

    pub fn set_low(&self) {
        // Write to the output data register for the configured pin.
        // Actual register-write logic not yet implemented; keep placeholder.
    }
}
pub struct InputRegisterBlock {
    ahbenr: super::register::RCC_AHBENR,
    moder: GPIOx_MODER,
    otyper: GPIOx_OTYPER,
    idr: GPIOx_IDR,
}

impl InputRegisterBlock {
    pub fn new() -> Self {
        Self {
            ahbenr: RCC_AHBENR {},
            moder: GPIOx_MODER {},
            otyper: GPIOx_OTYPER {},
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
