use crate::mcu::register::RegisterAddress;

#[derive(Clone, Copy)]
pub enum GPIOPort {
    A,
    B,
    C,
    D,
    E,
    F
}

impl GPIOPort {
    /**
     * Translate to a valid base address as per RM0316, Table 4
     * Note inconsistency of the RM0315 with UM1507 re. Port E.
     */
    pub fn as_ahb2_base_address(&self) -> RegisterAddress {
        match self {
            GPIOPort::A => 0x48000000 as RegisterAddress,
            GPIOPort::B => 0x48000400 as RegisterAddress, 
            GPIOPort::C => 0x48000800 as RegisterAddress,
            GPIOPort::D => 0x48000C00 as RegisterAddress,
            GPIOPort::E => 0x48001000 as RegisterAddress, // see comments above
            GPIOPort::F => 0x48001400 as RegisterAddress
        }
    }

    /**
     * Return bit position within the AHB peripheral clock endable register RCC_AHBENR 
     */
    pub fn as_rcc_ahbenr_bitpos(&self) -> u32 {
        match self {
                    GPIOPort::A => 17,
                    GPIOPort::B => 18,
                    GPIOPort::C => 19,
                    GPIOPort::D => 20,
                    GPIOPort::E => 21,
                    GPIOPort::F => 22
                }
    }

    /**
     * Return address of the AHB peripheral clock endable register RCC_AHBENR 
     */
    pub fn as_rcc_ahbenr_addr(&self) -> RegisterAddress {
        (0x4002_1000 + 0x14) as RegisterAddress
    }
}
