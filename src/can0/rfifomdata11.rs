#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RFIFOMDATA11 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DB7R {
    bits: u8,
}
impl DB7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DB6R {
    bits: u8,
}
impl DB6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DB5R {
    bits: u8,
}
impl DB5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DB4R {
    bits: u8,
}
impl DB4R {
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
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline]
    pub fn db7(&self) -> DB7R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB7R { bits }
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline]
    pub fn db6(&self) -> DB6R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB6R { bits }
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline]
    pub fn db5(&self) -> DB5R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB5R { bits }
    }
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline]
    pub fn db4(&self) -> DB4R {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DB4R { bits }
    }
}
