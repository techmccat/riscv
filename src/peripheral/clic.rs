pub use super::CLIC;

pub unsafe trait InterruptNumber: Copy + TryFrom<u16> {
    fn number(self) -> i16;
}

pub mod hartlocal {
    use volatile_register::RW;
    const MAX_SOURCES: usize = 0x400;

    /// Register block.
    #[repr(C)]
    pub struct RegisterBlock {
        /// `0x0000..=0x03FF` - Interrupt Pending Register.
        pub clic_int_ip: [RW<u8>; MAX_SOURCES],
        /// `0x0400..=0x07FF` - Interrupt Enable Register.
        pub clic_int_ie: [RW<u8>; MAX_SOURCES],
        /// `0x0800..=0x0BFF` - Interrupt Configuration Register.
        pub clic_int_cfg: [RW<u8>; MAX_SOURCES],
        /// `0x0C00` - Configuration Register.
        pub clic_cfg: RW<u8>,
    }
}

// API modeled after cortex_m::peripheral::NVIC
impl<const SHARED: usize, const HART: usize> CLIC<SHARED, HART> {
    /// Disables `int`
    #[inline]
    pub fn mask<I: InterruptNumber>(int: I) {
        unsafe { (*Self::HART).clic_int_ie[int.number() as usize].write(0) }
    }

    /// Enables `int`
    ///
    /// This function is unsafe because it can break mask-based critical sections
    #[inline]
    pub unsafe fn unmask<I: InterruptNumber>(int: I) {
        (*Self::HART).clic_int_ie[int.number() as usize].write(1)
    }

    /// Checks if `int` is enabled
    #[inline]
    pub fn is_enabled<I: InterruptNumber>(int: I) -> bool {
        unsafe { (*Self::HART).clic_int_ie[int.number() as usize].read() == 1 }
    }

    /// Sets `int` as pending
    ///
    /// # Note
    ///
    /// Only software interrupts may be pended this way
    #[inline]
    pub fn pend<I: InterruptNumber>(int: I) {
        unsafe { (*Self::HART).clic_int_ip[int.number() as usize].write(0) }
    }

    /// Clears `int`'s pending state
    ///
    /// # Note
    ///
    /// Only software interrupts may be unpended this way
    #[inline]
    pub fn unpend<I: InterruptNumber>(int: I) {
        unsafe { (*Self::HART).clic_int_ip[int.number() as usize].write(1) }
    }

    /// Checks if `int` is pending
    #[inline]
    pub fn is_pending<I: InterruptNumber>(int: I) -> bool {
        unsafe { (*Self::HART).clic_int_ip[int.number() as usize].read() == 1 }
    }

    /// Sets `int`'s preemption level
    ///
    /// # Safety
    ///
    /// Changing preemption levels at runtime can compromise memory safety
    #[inline]
    pub unsafe fn set_level<I: InterruptNumber>(&mut self, int: I, level: u8) {
        (*Self::HART).clic_int_cfg[int.number() as usize].write(level << 4)
    }

    /// Returns `int`'s preemption level
    #[inline]
    pub fn get_level<I: InterruptNumber>(int: I) -> u8 {
        unsafe { (*Self::HART).clic_int_cfg[int.number() as usize].read() >> 4 }
    }
}
