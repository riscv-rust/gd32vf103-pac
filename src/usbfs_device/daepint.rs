#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DAEPINT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IEPITBR {
    bits: u8,
}
impl IEPITBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OEPITBR {
    bits: u8,
}
impl OEPITBR {
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
    #[doc = "Bits 0:3 - Device all IN endpoint interrupt bits"]
    #[inline]
    pub fn iepitb(&self) -> IEPITBR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IEPITBR { bits }
    }
    #[doc = "Bits 16:19 - Device all OUT endpoint interrupt bits"]
    #[inline]
    pub fn oepitb(&self) -> OEPITBR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OEPITBR { bits }
    }
}
