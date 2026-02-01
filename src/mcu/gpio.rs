pub mod port;
pub mod gpio_mode;
mod gpio_output_type;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GPIOId {
    A,
    B,
    C,
    D,
    E,
    F
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)] // term from STM32 reference manual 
pub struct GPIO {
    pub id: GPIOId,
    pub pin: u32,
}
