#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SPSTR {
    bits: bool,
}
impl SPSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ESR {
    bits: u8,
}
impl ESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FNRSOFR {
    bits: u16,
}
impl FNRSOFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Suspend status"]
    #[inline]
    pub fn spst(&self) -> SPSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPSTR { bits }
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline]
    pub fn es(&self) -> ESR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ESR { bits }
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline]
    pub fn fnrsof(&self) -> FNRSOFR {
        let bits = {
            const MASK: u16 = 0x3fff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FNRSOFR { bits }
    }
}
