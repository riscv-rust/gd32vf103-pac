#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFIFOMI0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SFID_EFIDR {
    bits: u16,
}
impl SFID_EFIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFIDR {
    bits: u32,
}
impl EFIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FFR {
    bits: bool,
}
impl FFR {
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
pub struct FTR {
    bits: bool,
}
impl FTR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline]
    pub fn sfid_efid(&self) -> SFID_EFIDR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SFID_EFIDR { bits }
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline]
    pub fn efid(&self) -> EFIDR {
        let bits = {
            const MASK: u32 = 0x0003_ffff;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        EFIDR { bits }
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline]
    pub fn ff(&self) -> FFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FFR { bits }
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline]
    pub fn ft(&self) -> FTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTR { bits }
    }
}
