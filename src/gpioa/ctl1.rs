#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL1 {
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
pub struct CTL15R {
    bits: u8,
}
impl CTL15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD15R {
    bits: u8,
}
impl MD15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL14R {
    bits: u8,
}
impl CTL14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD14R {
    bits: u8,
}
impl MD14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL13R {
    bits: u8,
}
impl CTL13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD13R {
    bits: u8,
}
impl MD13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL12R {
    bits: u8,
}
impl CTL12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD12R {
    bits: u8,
}
impl MD12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL11R {
    bits: u8,
}
impl CTL11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD11R {
    bits: u8,
}
impl MD11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL10R {
    bits: u8,
}
impl CTL10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD10R {
    bits: u8,
}
impl MD10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL9R {
    bits: u8,
}
impl CTL9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD9R {
    bits: u8,
}
impl MD9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL8R {
    bits: u8,
}
impl CTL8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD8R {
    bits: u8,
}
impl MD8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CTL15W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD15W<'a> {
    w: &'a mut W,
}
impl<'a> _MD15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL14W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD14W<'a> {
    w: &'a mut W,
}
impl<'a> _MD14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL13W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD13W<'a> {
    w: &'a mut W,
}
impl<'a> _MD13W<'a> {
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
pub struct _CTL12W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD12W<'a> {
    w: &'a mut W,
}
impl<'a> _MD12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL11W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD11W<'a> {
    w: &'a mut W,
}
impl<'a> _MD11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL10W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD10W<'a> {
    w: &'a mut W,
}
impl<'a> _MD10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL9W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD9W<'a> {
    w: &'a mut W,
}
impl<'a> _MD9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL8W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MD8W<'a> {
    w: &'a mut W,
}
impl<'a> _MD8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
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
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline]
    pub fn ctl15(&self) -> CTL15R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL15R { bits }
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline]
    pub fn md15(&self) -> MD15R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD15R { bits }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline]
    pub fn ctl14(&self) -> CTL14R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL14R { bits }
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline]
    pub fn md14(&self) -> MD14R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD14R { bits }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline]
    pub fn ctl13(&self) -> CTL13R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL13R { bits }
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline]
    pub fn md13(&self) -> MD13R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD13R { bits }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline]
    pub fn ctl12(&self) -> CTL12R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL12R { bits }
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline]
    pub fn md12(&self) -> MD12R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD12R { bits }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline]
    pub fn ctl11(&self) -> CTL11R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL11R { bits }
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline]
    pub fn md11(&self) -> MD11R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD11R { bits }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline]
    pub fn ctl10(&self) -> CTL10R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL10R { bits }
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline]
    pub fn md10(&self) -> MD10R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD10R { bits }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline]
    pub fn ctl9(&self) -> CTL9R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL9R { bits }
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline]
    pub fn md9(&self) -> MD9R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD9R { bits }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline]
    pub fn ctl8(&self) -> CTL8R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL8R { bits }
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline]
    pub fn md8(&self) -> MD8R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD8R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x4444_4444 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline]
    pub fn ctl15(&mut self) -> _CTL15W {
        _CTL15W { w: self }
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline]
    pub fn md15(&mut self) -> _MD15W {
        _MD15W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline]
    pub fn ctl14(&mut self) -> _CTL14W {
        _CTL14W { w: self }
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline]
    pub fn md14(&mut self) -> _MD14W {
        _MD14W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline]
    pub fn ctl13(&mut self) -> _CTL13W {
        _CTL13W { w: self }
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline]
    pub fn md13(&mut self) -> _MD13W {
        _MD13W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline]
    pub fn ctl12(&mut self) -> _CTL12W {
        _CTL12W { w: self }
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline]
    pub fn md12(&mut self) -> _MD12W {
        _MD12W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline]
    pub fn ctl11(&mut self) -> _CTL11W {
        _CTL11W { w: self }
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline]
    pub fn md11(&mut self) -> _MD11W {
        _MD11W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline]
    pub fn ctl10(&mut self) -> _CTL10W {
        _CTL10W { w: self }
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline]
    pub fn md10(&mut self) -> _MD10W {
        _MD10W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline]
    pub fn ctl9(&mut self) -> _CTL9W {
        _CTL9W { w: self }
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline]
    pub fn md9(&mut self) -> _MD9W {
        _MD9W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline]
    pub fn ctl8(&mut self) -> _CTL8W {
        _CTL8W { w: self }
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline]
    pub fn md8(&mut self) -> _MD8W {
        _MD8W { w: self }
    }
}
