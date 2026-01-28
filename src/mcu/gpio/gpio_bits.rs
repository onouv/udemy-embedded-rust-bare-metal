pub trait GPIOBits {
    fn as_bit_value(&self) -> u32;

    // overwrite this, if the bits must take more than one bit
    fn bit_mask() -> u32 {
        0x1
    }

    // overwrite this, if the bits must take more than one bit
    fn bit_len() -> u32 {
        0x1
    }
}