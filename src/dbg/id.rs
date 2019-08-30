#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ID_CODER {
    bits: u32,
}
impl ID_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - DBG ID code register"]
    #[inline]
    pub fn id_code(&self) -> ID_CODER {
        let bits = {
            const MASK: u32 = 0xffff_ffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ID_CODER { bits }
    }
}
