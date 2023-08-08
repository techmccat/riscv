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
#[repr(usize)]
pub enum Interrupt {
    SupervisorSoft = 1,
    MachineSoft = 3,
    SupervisorTimer = 5,
    MachineTimer = 7,
    SupervisorExternal = 9,
    MachineExternal = 11,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Exception {
    InstructionMisaligned = 0,
    InstructionFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadMisaligned = 4,
    LoadFault = 5,
    StoreMisaligned = 6,
    StoreFault = 7,
    UserEnvCall = 8,
    SupervisorEnvCall = 9,
    MachineEnvCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
    Unknown,
}

impl From<usize> for Interrupt {
    #[inline]
    fn from(nr: usize) -> Self {
        match nr {
            1 => Self::SupervisorSoft,
            3 => Self::MachineSoft,
            5 => Self::SupervisorTimer,
            7 => Self::MachineTimer,
            9 => Self::SupervisorExternal,
            11 => Self::MachineExternal,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Interrupt> for usize {
    type Error = Interrupt;

    #[inline]
    fn try_from(value: Interrupt) -> Result<Self, Self::Error> {
        match value {
            Interrupt::Unknown => Err(Self::Error::Unknown),
            _ => Ok(value as Self),
        }
    }
}

impl From<usize> for Exception {
    #[inline]
    fn from(nr: usize) -> Self {
        match nr {
            0 => Self::InstructionMisaligned,
            1 => Self::InstructionFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadMisaligned,
            5 => Self::LoadFault,
            6 => Self::StoreMisaligned,
            7 => Self::StoreFault,
            8 => Self::UserEnvCall,
            9 => Self::SupervisorEnvCall,
            11 => Self::MachineEnvCall,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Exception> for usize {
    type Error = Exception;

    #[inline]
    fn try_from(value: Exception) -> Result<Self, Self::Error> {
        match value {
            Exception::Unknown => Err(Self::Error::Unknown),
            _ => Ok(value as Self),
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
        self.bits & !(1 << (usize::BITS as usize - 1))
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
        self.bits & (1 << (usize::BITS as usize - 1)) != 0
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
