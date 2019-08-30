#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFIFOMP0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TSR {
    bits: u16,
}
impl TSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIR {
    bits: u8,
}
impl FIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLENCR {
    bits: u8,
}
impl DLENCR {
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
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline]
    pub fn ts(&self) -> TSR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSR { bits }
    }
    #[doc = "Bits 8:15 - Filtering index"]
    #[inline]
    pub fn fi(&self) -> FIR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIR { bits }
    }
    #[doc = "Bits 0:3 - Data length code"]
    #[inline]
    pub fn dlenc(&self) -> DLENCR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLENCR { bits }
    }
}
