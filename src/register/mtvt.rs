//! mtvt register

/// Machine trap vector table register
#[derive(Clone, Copy, Debug)]
pub struct Mtvt {
    bits: usize
}

impl Mtvt {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    #[inline]
    pub fn address(&self) -> usize {
        self.bits & !0b111111
    }
}

read_csr_as!(Mtvt, 0x307);
write_csr!(0x307);

/// Writes the CSR
#[inline]
pub unsafe fn write(addr: usize) {
    // low 6 bits are ignored since address is 64 byte aligned
    let bits = addr & !0b111111;
    _write(bits);
}
