#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSQ2 {
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
pub struct RSQ5R {
    bits: u8,
}
impl RSQ5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSQ4R {
    bits: u8,
}
impl RSQ4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSQ3R {
    bits: u8,
}
impl RSQ3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSQ2R {
    bits: u8,
}
impl RSQ2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSQ1R {
    bits: u8,
}
impl RSQ1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSQ0R {
    bits: u8,
}
impl RSQ0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RSQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _RSQ5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _RSQ4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _RSQ3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _RSQ2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _RSQ1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _RSQ0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
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
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline]
    pub fn rsq5(&self) -> RSQ5R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSQ5R { bits }
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline]
    pub fn rsq4(&self) -> RSQ4R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSQ4R { bits }
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline]
    pub fn rsq3(&self) -> RSQ3R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSQ3R { bits }
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline]
    pub fn rsq2(&self) -> RSQ2R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSQ2R { bits }
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline]
    pub fn rsq1(&self) -> RSQ1R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSQ1R { bits }
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline]
    pub fn rsq0(&self) -> RSQ0R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSQ0R { bits }
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
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline]
    pub fn rsq5(&mut self) -> _RSQ5W {
        _RSQ5W { w: self }
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline]
    pub fn rsq4(&mut self) -> _RSQ4W {
        _RSQ4W { w: self }
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline]
    pub fn rsq3(&mut self) -> _RSQ3W {
        _RSQ3W { w: self }
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline]
    pub fn rsq2(&mut self) -> _RSQ2W {
        _RSQ2W { w: self }
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline]
    pub fn rsq1(&mut self) -> _RSQ1W {
        _RSQ1W { w: self }
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline]
    pub fn rsq0(&mut self) -> _RSQ0W {
        _RSQ0W { w: self }
    }
}
