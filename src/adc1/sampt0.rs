#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAMPT0 {
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
pub struct SPT10R {
    bits: u8,
}
impl SPT10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT11R {
    bits: u8,
}
impl SPT11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT12R {
    bits: u8,
}
impl SPT12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT13R {
    bits: u8,
}
impl SPT13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT14R {
    bits: u8,
}
impl SPT14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT15R {
    bits: u8,
}
impl SPT15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT16R {
    bits: u8,
}
impl SPT16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT17R {
    bits: u8,
}
impl SPT17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SPT10W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT11W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT12W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT13W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT14W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT15W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT16W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT17W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT17W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline]
    pub fn spt10(&self) -> SPT10R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT10R { bits }
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline]
    pub fn spt11(&self) -> SPT11R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT11R { bits }
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline]
    pub fn spt12(&self) -> SPT12R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT12R { bits }
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline]
    pub fn spt13(&self) -> SPT13R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT13R { bits }
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline]
    pub fn spt14(&self) -> SPT14R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT14R { bits }
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline]
    pub fn spt15(&self) -> SPT15R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT15R { bits }
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline]
    pub fn spt16(&self) -> SPT16R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT16R { bits }
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline]
    pub fn spt17(&self) -> SPT17R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT17R { bits }
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
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline]
    pub fn spt10(&mut self) -> _SPT10W {
        _SPT10W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline]
    pub fn spt11(&mut self) -> _SPT11W {
        _SPT11W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline]
    pub fn spt12(&mut self) -> _SPT12W {
        _SPT12W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline]
    pub fn spt13(&mut self) -> _SPT13W {
        _SPT13W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline]
    pub fn spt14(&mut self) -> _SPT14W {
        _SPT14W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline]
    pub fn spt15(&mut self) -> _SPT15W {
        _SPT15W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline]
    pub fn spt16(&mut self) -> _SPT16W {
        _SPT16W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline]
    pub fn spt17(&mut self) -> _SPT17W {
        _SPT17W { w: self }
    }
}
