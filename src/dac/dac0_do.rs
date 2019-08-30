#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DAC0_DO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DAC0_DOR {
    bits: u16,
}
impl DAC0_DOR {
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
    #[doc = "Bits 0:11 - DAC0 data output"]
    #[inline]
    pub fn dac0_do(&self) -> DAC0_DOR {
        let bits = {
            const MASK: u16 = 0x0fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DAC0_DOR { bits }
    }
}
