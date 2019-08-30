#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GRSTATP_HOST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CNUMR {
    bits: u8,
}
impl CNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BCOUNTR {
    bits: u16,
}
impl BCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DPIDR {
    bits: u8,
}
impl DPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RPCKSTR {
    bits: u8,
}
impl RPCKSTR {
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
    #[doc = "Bits 0:3 - Channel number"]
    #[inline]
    pub fn cnum(&self) -> CNUMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CNUMR { bits }
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline]
    pub fn bcount(&self) -> BCOUNTR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BCOUNTR { bits }
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline]
    pub fn dpid(&self) -> DPIDR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DPIDR { bits }
    }
    #[doc = "Bits 17:20 - Reivece packet status"]
    #[inline]
    pub fn rpckst(&self) -> RPCKSTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RPCKSTR { bits }
    }
}
