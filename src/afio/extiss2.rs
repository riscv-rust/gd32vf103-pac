#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTISS2 {
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
pub struct EXTI11_SSR {
    bits: u8,
}
impl EXTI11_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI10_SSR {
    bits: u8,
}
impl EXTI10_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI9_SSR {
    bits: u8,
}
impl EXTI9_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI8_SSR {
    bits: u8,
}
impl EXTI8_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EXTI11_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI11_SSW<'a> {
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
pub struct _EXTI10_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI10_SSW<'a> {
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
pub struct _EXTI9_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI9_SSW<'a> {
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
pub struct _EXTI8_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI8_SSW<'a> {
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
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline]
    pub fn exti11_ss(&self) -> EXTI11_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI11_SSR { bits }
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline]
    pub fn exti10_ss(&self) -> EXTI10_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI10_SSR { bits }
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline]
    pub fn exti9_ss(&self) -> EXTI9_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI9_SSR { bits }
    }
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline]
    pub fn exti8_ss(&self) -> EXTI8_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI8_SSR { bits }
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
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline]
    pub fn exti11_ss(&mut self) -> _EXTI11_SSW {
        _EXTI11_SSW { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline]
    pub fn exti10_ss(&mut self) -> _EXTI10_SSW {
        _EXTI10_SSW { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline]
    pub fn exti9_ss(&mut self) -> _EXTI9_SSW {
        _EXTI9_SSW { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline]
    pub fn exti8_ss(&mut self) -> _EXTI8_SSW {
        _EXTI8_SSW { w: self }
    }
}
