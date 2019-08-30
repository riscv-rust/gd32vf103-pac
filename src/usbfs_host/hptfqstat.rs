#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HPTFQSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PTXFSR {
    bits: u16,
}
impl PTXFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PTXREQSR {
    bits: u8,
}
impl PTXREQSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PTXREQTR {
    bits: u8,
}
impl PTXREQTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline]
    pub fn ptxfs(&self) -> PTXFSR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PTXFSR { bits }
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline]
    pub fn ptxreqs(&self) -> PTXREQSR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTXREQSR { bits }
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline]
    pub fn ptxreqt(&self) -> PTXREQTR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTXREQTR { bits }
    }
}
