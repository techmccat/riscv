use volatile_register::RW;

/// Register block.
#[repr(C)]
pub struct RegisterBlock {
    /// `0x0000..=0x0003` - Machine Software Interrupt Pending register.
    pub msip: RW<u32>,
    /// `0x0004..=0x3FFF` - Reserved.
    _reserved1: [u8; 0x3ffc],
    /// `0x4000..=0x4007` - MTIMECMP register.
    pub mtimecmp: RW<u64>,
    /// `0x4008..=0xBFF7` - Reserved.
    _reserved2: [u8; 0x7ff0],
    /// `0xBFF8..=0xBFFF` - Timer register.
    pub mtime: RW<u64>,
}
