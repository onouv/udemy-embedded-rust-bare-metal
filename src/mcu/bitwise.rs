pub trait Bitwise {
    /** Return (b, n) where
     *  b: bit pattern
     *  n: number of bits
     */
    fn as_bit_pattern(&self) -> (u32, u32);
}
