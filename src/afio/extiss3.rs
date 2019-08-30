#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTISS3 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct EXTI15_SSR {
    bits: u8,
}
impl EXTI15_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI14_SSR {
    bits: u8,
}
impl EXTI14_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI13_SSR {
    bits: u8,
}
impl EXTI13_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI12_SSR {
    bits: u8,
}
impl EXTI12_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EXTI15_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI15_SSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI14_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI14_SSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI13_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI13_SSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI12_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI12_SSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline]
    pub fn exti15_ss(&self) -> EXTI15_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI15_SSR { bits }
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline]
    pub fn exti14_ss(&self) -> EXTI14_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI14_SSR { bits }
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline]
    pub fn exti13_ss(&self) -> EXTI13_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI13_SSR { bits }
    }
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline]
    pub fn exti12_ss(&self) -> EXTI12_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI12_SSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline]
    pub fn exti15_ss(&mut self) -> _EXTI15_SSW {
        _EXTI15_SSW { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline]
    pub fn exti14_ss(&mut self) -> _EXTI14_SSW {
        _EXTI14_SSW { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline]
    pub fn exti13_ss(&mut self) -> _EXTI13_SSW {
        _EXTI13_SSW { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline]
    pub fn exti12_ss(&mut self) -> _EXTI12_SSW {
        _EXTI12_SSW { w: self }
    }
}
