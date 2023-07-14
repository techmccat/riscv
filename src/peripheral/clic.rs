//! Core-Level Interrupt Controller (CLIC) peripheral.

use crate::peripheral::common::{unsafe_peripheral, RW};
use super::{CLIC, CLINT};

impl CLIC {
    /// Creates a new `CLIC` peripheral.
    ///
    /// Takes a base address for the shared region and a base
    /// address for the hart0 region.
    ///
    /// # Safety
    ///
    /// Base addresses mnust point to a valid CLIC peripheral.
    pub unsafe fn new(shared: usize, hart0: usize) -> Self {
        Self {
            shared: CLINT::new(shared),
            hart0: HartLocal::new(hart0),
        }
    }
}

/// Trait for enums of interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available local interrupts for a specific core.
/// Each variant must convert to a `u16` of its interrupt number.
///
/// # Note
///
/// Interrupt numbers from 0 to 15 are reserved for software, timer and external interrupts:
/// - ID 3: software interrupt
/// - ID 7: timer interrupt
/// - ID 11: external interrupt
/// - ID 12: CLIC software interrupt
///
/// # Safety
///
/// This trait must only be implemented on enums of local interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
/// All the interrupt numbers must be less than or equal to `MAX_INTERRUPT_NUMBER`.
/// `MAX_INTERRUPT_NUMBER` must coincide with the highest allowed interrupt number.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source as of version 20180831 of the spec
    const MAX_INTERRUPT_NUMBER: u16 = 256;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    /// If the conversion fails, it returns an error with the number back.
    fn try_from(value: u16) -> Result<Self, u16>;
}
/// Registers local to a hart
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HartLocal {
    // only index 3 and 12 are writable by software
    // maybe add separate msip_pending and csip_pending fields?
    pub interrupt_pending: ClicIntIP,
    pub interrupt_enabled: ClicIntIE,
    pub interrupt_config: ClicIntCfg,
    pub config: ClicCfg,
}

unsafe_peripheral!(ClicIntIP, u8, RW);
unsafe_peripheral!(ClicIntIE, u8, RW);
unsafe_peripheral!(ClicIntCfg, u8, RW);
unsafe_peripheral!(ClicCfg, u8, RW);

/// Register block specific to a hart
impl HartLocal {
    pub unsafe fn new(address: usize) -> Self { 
        Self {
            interrupt_pending: ClicIntIP::new(address),
            interrupt_enabled: ClicIntIE::new(address + 0x400),
            interrupt_config: ClicIntCfg::new(address + 0x800),
            config: ClicCfg::new(address + 0xC00)
        } 
    }

    /// Disable `interrupt`
    pub fn mask<I: InterruptNumber>(&self, interrupt: I) {
        let nr = interrupt.number() as isize;
        unsafe { self.interrupt_enabled.get_ptr().offset(nr).write(0u8) }
    }

    /// Enable `interrupt`
    pub unsafe fn unmask<I: InterruptNumber>(&self, interrupt: I) {
        let nr = interrupt.number() as isize;
        self.interrupt_enabled.get_ptr().offset(nr).write(1u8)
    }

    /// Check if `interrupt` is enabled
    pub fn is_enabled<I: InterruptNumber>(&self, interrupt: I) -> bool {
        let nr = interrupt.number() as isize;
        unsafe { self.interrupt_enabled.get_ptr().offset(nr).read() != 0 }
    }

    /// Set an interrupt as pending.
    /// Only software interrupts may be pended this way
    pub fn pend<I: InterruptNumber>(&self, interrupt: I) {
        let nr = interrupt.number() as isize;
        match nr {
            3 | 12 => unsafe { self.interrupt_pending.get_ptr().offset(nr).write(1u8) },
            _ => ()
        }
    }

    /// Unpend a software interrupt (only works with interrups 3 and 12)
    pub fn unpend<I: InterruptNumber>(&self, interrupt: I) {
        let nr = interrupt.number() as isize;
        match nr {
            3 | 12 => unsafe { self.interrupt_pending.get_ptr().offset(nr).write(0u8) },
            _ => ()
        }
    }

    /// Check if `interrupt` is enabled
    pub fn is_pending<I: InterruptNumber>(&self, interrupt: I) -> bool {
        let nr = interrupt.number() as isize;
        unsafe { self.interrupt_pending.get_ptr().offset(nr).read() != 0 }
    }

    /// Sets the preemption level of `interrupt`
    ///
    /// Valid levels are between 0 and 15
    pub unsafe fn set_level<I: InterruptNumber>(&self, interrupt: I, level: u8) {
        let nr = interrupt.number() as isize;
        self.interrupt_config.get_ptr().offset(nr).write((level & 0xf) << 4)
    }

    /// Get the preemption level of `interrupt`
    pub fn get_level<I: InterruptNumber>(&self, interrupt: I) -> u8 {
        let nr = interrupt.number() as isize;
        unsafe { self.interrupt_config.get_ptr().offset(nr).read() >> 4 }
    }
}
