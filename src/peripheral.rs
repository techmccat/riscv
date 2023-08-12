//! RISC-V peripherals
use core::marker::PhantomData;

// Platform-Level Interrupt Controller
#[cfg(feature = "plic")]
pub mod plic;

/// Interface for a context of the PLIC peripheral.
///
/// # Note
///
/// This structure requires the `plic` feature.
///
/// The RISC-V standard does not specify a fixed location for the PLIC.
/// Thus, we use const generics to map a PLIC to the desired memory location.
/// Each platform must specify the base address of the PLIC on the platform.
///
/// The PLIC standard allows up to 15_872 different contexts for interfacing the PLIC.
/// Usually, each HART uses a dedicated context. In this way, they do not interfere
/// with each other when attending to external interruptions.
///
/// You can use the [`crate::plic_context`] macro to generate a specific structure
/// for interfacing every PLIC context of your platform. The resulting structure
/// replaces generic types with the specific types of your target.
#[allow(clippy::upper_case_acronyms)]
#[cfg(feature = "plic")]
#[derive(Default)]
pub struct PLIC<const BASE: usize, const CONTEXT: usize> {
    _marker: PhantomData<*const ()>,
}

#[cfg(feature = "plic")]
impl<const BASE: usize, const CONTEXT: usize> PLIC<BASE, CONTEXT> {
    /// Pointer to the register block
    pub const PTR: *const self::plic::RegisterBlock = BASE as *const _;

    /// Creates a new interface for the PLIC peripheral. PACs can use this
    /// function to add a PLIC interface to their `Peripherals` struct.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

// Core Local Interruptor
#[cfg(any(feature = "clint", feature = "clic-sifive"))]
pub mod clint;

// Core-level Interrupt Controller
#[cfg(feature = "clic-sifive")]
pub mod clic;

/// Interface for a CLIC peripheral
///
/// # Note
///
/// You need to set the `clic-sifive` feature to enable this peripheral
#[allow(clippy::upper_case_acronyms)]
#[cfg(feature = "clic-sifive")]
#[derive(Default)]
pub struct CLIC<const SHARED: usize, const HART: usize> {
    _marker: PhantomData<*const ()>,
}

#[cfg(feature = "clic-sifive")]
impl<const SHARED: usize, const HART: usize> CLIC<SHARED, HART> {
    /// Pointer to the shared CLINT register block
    pub const SHARED: *const self::clint::RegisterBlock = SHARED as *const _;
    /// Pointer to the HART-specific register block
    pub const HART: *const self::clic::hartlocal::RegisterBlock = HART as *const _;

    /// Creates a new interface for the CLIC peripheral. PACs can use this
    /// function to add a CLIC interface to their `Peripherals` struct.
    pub fn new() -> Self {
        // initializes nlBits to 4, allocating all 4 writable bits in clicIntCfg to the interrupt
        // preemption level.
        // TODO: some safe way of setting both levels and priorities
        unsafe { (*Self::HART).clic_cfg.write(4 << 1) };
        Self {
            _marker: PhantomData,
        }
    }
}
