#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DAC1_DO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DAC1_DOR {
    bits: u16,
}
impl DAC1_DOR {
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
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline]
    pub fn dac1_do(&self) -> DAC1_DOR {
        let bits = {
            const MASK: u16 = 0x0fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DAC1_DOR { bits }
    }
}
