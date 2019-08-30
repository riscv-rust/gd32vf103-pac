#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLICINFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NUM_INTERRUPTR {
    bits: u16,
}
impl NUM_INTERRUPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VERSIONR {
    bits: u8,
}
impl VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLICINTCTLBITSR {
    bits: u8,
}
impl CLICINTCTLBITSR {
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
    #[doc = "Bits 0:12 - NUM_INTERRUPT"]
    #[inline]
    pub fn num_interrupt(&self) -> NUM_INTERRUPTR {
        let bits = {
            const MASK: u16 = 0x1fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NUM_INTERRUPTR { bits }
    }
    #[doc = "Bits 13:20 - VERSION"]
    #[inline]
    pub fn version(&self) -> VERSIONR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERSIONR { bits }
    }
    #[doc = "Bits 21:24 - CLICINTCTLBITS"]
    #[inline]
    pub fn clicintctlbits(&self) -> CLICINTCTLBITSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLICINTCTLBITSR { bits }
    }
}
