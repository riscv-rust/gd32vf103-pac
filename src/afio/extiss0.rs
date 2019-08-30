#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTISS0 {
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
pub struct EXTI3_SSR {
    bits: u8,
}
impl EXTI3_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI2_SSR {
    bits: u8,
}
impl EXTI2_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI1_SSR {
    bits: u8,
}
impl EXTI1_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI0_SSR {
    bits: u8,
}
impl EXTI0_SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EXTI3_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI3_SSW<'a> {
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
pub struct _EXTI2_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI2_SSW<'a> {
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
pub struct _EXTI1_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI1_SSW<'a> {
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
pub struct _EXTI0_SSW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI0_SSW<'a> {
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
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline]
    pub fn exti3_ss(&self) -> EXTI3_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI3_SSR { bits }
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline]
    pub fn exti2_ss(&self) -> EXTI2_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI2_SSR { bits }
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline]
    pub fn exti1_ss(&self) -> EXTI1_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI1_SSR { bits }
    }
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline]
    pub fn exti0_ss(&self) -> EXTI0_SSR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI0_SSR { bits }
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
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline]
    pub fn exti3_ss(&mut self) -> _EXTI3_SSW {
        _EXTI3_SSW { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline]
    pub fn exti2_ss(&mut self) -> _EXTI2_SSW {
        _EXTI2_SSW { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline]
    pub fn exti1_ss(&mut self) -> _EXTI1_SSW {
        _EXTI1_SSW { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline]
    pub fn exti0_ss(&mut self) -> _EXTI0_SSW {
        _EXTI0_SSW { w: self }
    }
}
