#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISQ {
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
pub struct ILR {
    bits: u8,
}
impl ILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISQ3R {
    bits: u8,
}
impl ISQ3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISQ2R {
    bits: u8,
}
impl ISQ2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISQ1R {
    bits: u8,
}
impl ISQ1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ISQ0R {
    bits: u8,
}
impl ISQ0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ILW<'a> {
    w: &'a mut W,
}
impl<'a> _ILW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _ISQ3W<'a> {
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
pub struct _ISQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _ISQ2W<'a> {
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
pub struct _ISQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _ISQ1W<'a> {
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
pub struct _ISQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _ISQ0W<'a> {
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
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline]
    pub fn il(&self) -> ILR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ILR { bits }
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline]
    pub fn isq3(&self) -> ISQ3R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ISQ3R { bits }
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline]
    pub fn isq2(&self) -> ISQ2R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ISQ2R { bits }
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline]
    pub fn isq1(&self) -> ISQ1R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ISQ1R { bits }
    }
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline]
    pub fn isq0(&self) -> ISQ0R {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ISQ0R { bits }
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
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline]
    pub fn il(&mut self) -> _ILW {
        _ILW { w: self }
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline]
    pub fn isq3(&mut self) -> _ISQ3W {
        _ISQ3W { w: self }
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline]
    pub fn isq2(&mut self) -> _ISQ2W {
        _ISQ2W { w: self }
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline]
    pub fn isq1(&mut self) -> _ISQ1W {
        _ISQ1W { w: self }
    }
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline]
    pub fn isq0(&mut self) -> _ISQ0W {
        _ISQ0W { w: self }
    }
}
