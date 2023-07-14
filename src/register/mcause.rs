//! mcause register

#[cfg(feature = "clic-sifive")]
use bit_field::BitField;

/// mcause register
#[derive(Clone, Copy, Debug)]
pub struct Mcause {
    bits: usize,
}

/// Trap Cause
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadMisaligned,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    SupervisorEnvCall,
    MachineEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}

impl Interrupt {
    #[inline]
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Interrupt::UserSoft,
            1 => Interrupt::SupervisorSoft,
            3 => Interrupt::MachineSoft,
            4 => Interrupt::UserTimer,
            5 => Interrupt::SupervisorTimer,
            7 => Interrupt::MachineTimer,
            8 => Interrupt::UserExternal,
            9 => Interrupt::SupervisorExternal,
            11 => Interrupt::MachineExternal,
            _ => Interrupt::Unknown,
        }
    }
}

impl Exception {
    #[inline]
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Exception::InstructionMisaligned,
            1 => Exception::InstructionFault,
            2 => Exception::IllegalInstruction,
            3 => Exception::Breakpoint,
            4 => Exception::LoadMisaligned,
            5 => Exception::LoadFault,
            6 => Exception::StoreMisaligned,
            7 => Exception::StoreFault,
            8 => Exception::UserEnvCall,
            9 => Exception::SupervisorEnvCall,
            11 => Exception::MachineEnvCall,
            12 => Exception::InstructionPageFault,
            13 => Exception::LoadPageFault,
            15 => Exception::StorePageFault,
            _ => Exception::Unknown,
        }
    }
}
impl Mcause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    #[inline]
    pub fn code(&self) -> usize {
        match () {
            #[cfg(target_pointer_width = "32")]
            () => self.bits & !(1 << 31),
            #[cfg(target_pointer_width = "64")]
            () => self.bits & !(1 << 63),
            #[cfg(target_pointer_width = "128")]
            () => self.bits & !(1 << 127),
        }
    }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        match () {
            #[cfg(target_pointer_width = "32")]
            () => self.bits & (1 << 31) == 1 << 31,
            #[cfg(target_pointer_width = "64")]
            () => self.bits & (1 << 63) == 1 << 63,
            #[cfg(target_pointer_width = "128")]
            () => self.bits & (1 << 127) == 1 << 127,
        }
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }

    /// Machine Previous Interrupt Enable field from the mstatus register
    /// 
    /// This field is only available in CLIC mode
    #[cfg(feature = "clic-sifive")]
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits.get_bit(27)
    }

    /// Supervisor Previous Privilege Mode field from the mstatus register
    ///
    /// This field is only available in CLIC mode
    #[cfg(feature = "clic-sifive")]
    #[inline]
    pub fn mpp(&self) -> super::mstatus::MPP {
        match self.bits.get_bits(28..30) {
            0b00 => super::mstatus::MPP::User,
            0b01 => super::mstatus::MPP::Supervisor,
            0b11 => super::mstatus::MPP::Machine,
            _ => unreachable!(),
        }
    }

    /// Hardware vectoring is in progress when set
    /// 
    /// This field is only available in CLIC mode
    #[cfg(feature = "clic-sifive")]
    #[inline]
    pub fn minhv(&self) -> bool {
        self.bits.get_bit(30)
    }
}

read_csr_as!(Mcause, 0x342);
