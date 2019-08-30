#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HNPTFQSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NPTXFSR {
    bits: u16,
}
impl NPTXFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPTXRQSR {
    bits: u8,
}
impl NPTXRQSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPTXRQTOPR {
    bits: u8,
}
impl NPTXRQTOPR {
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
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space"]
    #[inline]
    pub fn nptxfs(&self) -> NPTXFSR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NPTXFSR { bits }
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space"]
    #[inline]
    pub fn nptxrqs(&self) -> NPTXRQSR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPTXRQSR { bits }
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline]
    pub fn nptxrqtop(&self) -> NPTXRQTOPR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPTXRQTOPR { bits }
    }
}
