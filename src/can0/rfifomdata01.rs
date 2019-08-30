#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFIFOMDATA01 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DB3R {
    bits: u8,
}
impl DB3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DB2R {
    bits: u8,
}
impl DB2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DB1R {
    bits: u8,
}
impl DB1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DB0R {
    bits: u8,
}
impl DB0R {
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
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline]
    pub fn db3(&self) -> DB3R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB3R { bits }
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline]
    pub fn db2(&self) -> DB2R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB2R { bits }
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline]
    pub fn db1(&self) -> DB1R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB1R { bits }
    }
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline]
    pub fn db0(&self) -> DB0R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB0R { bits }
    }
}
