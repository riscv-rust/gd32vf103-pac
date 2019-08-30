#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL0 {
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
pub struct CTL7R {
    bits: u8,
}
impl CTL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD7R {
    bits: u8,
}
impl MD7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL6R {
    bits: u8,
}
impl CTL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD6R {
    bits: u8,
}
impl MD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL5R {
    bits: u8,
}
impl CTL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD5R {
    bits: u8,
}
impl MD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL4R {
    bits: u8,
}
impl CTL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD4R {
    bits: u8,
}
impl MD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL3R {
    bits: u8,
}
impl CTL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD3R {
    bits: u8,
}
impl MD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL2R {
    bits: u8,
}
impl CTL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD2R {
    bits: u8,
}
impl MD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL1R {
    bits: u8,
}
impl CTL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD1R {
    bits: u8,
}
impl MD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL0R {
    bits: u8,
}
impl CTL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MD0R {
    bits: u8,
}
impl MD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CTL7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL7W<'a> {
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
pub struct _MD7W<'a> {
    w: &'a mut W,
}
impl<'a> _MD7W<'a> {
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
pub struct _CTL6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL6W<'a> {
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
pub struct _MD6W<'a> {
    w: &'a mut W,
}
impl<'a> _MD6W<'a> {
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
pub struct _CTL5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL5W<'a> {
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
pub struct _MD5W<'a> {
    w: &'a mut W,
}
impl<'a> _MD5W<'a> {
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
pub struct _CTL4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL4W<'a> {
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
pub struct _MD4W<'a> {
    w: &'a mut W,
}
impl<'a> _MD4W<'a> {
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
pub struct _CTL3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL3W<'a> {
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
pub struct _MD3W<'a> {
    w: &'a mut W,
}
impl<'a> _MD3W<'a> {
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
pub struct _CTL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL2W<'a> {
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
pub struct _MD2W<'a> {
    w: &'a mut W,
}
impl<'a> _MD2W<'a> {
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
pub struct _CTL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL1W<'a> {
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
pub struct _MD1W<'a> {
    w: &'a mut W,
}
impl<'a> _MD1W<'a> {
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
pub struct _CTL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTL0W<'a> {
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
pub struct _MD0W<'a> {
    w: &'a mut W,
}
impl<'a> _MD0W<'a> {
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
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline]
    pub fn ctl7(&self) -> CTL7R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL7R { bits }
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline]
    pub fn md7(&self) -> MD7R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD7R { bits }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline]
    pub fn ctl6(&self) -> CTL6R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL6R { bits }
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline]
    pub fn md6(&self) -> MD6R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD6R { bits }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline]
    pub fn ctl5(&self) -> CTL5R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL5R { bits }
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline]
    pub fn md5(&self) -> MD5R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD5R { bits }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline]
    pub fn ctl4(&self) -> CTL4R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL4R { bits }
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline]
    pub fn md4(&self) -> MD4R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD4R { bits }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline]
    pub fn ctl3(&self) -> CTL3R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL3R { bits }
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline]
    pub fn md3(&self) -> MD3R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD3R { bits }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline]
    pub fn ctl2(&self) -> CTL2R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL2R { bits }
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline]
    pub fn md2(&self) -> MD2R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD2R { bits }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline]
    pub fn ctl1(&self) -> CTL1R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL1R { bits }
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline]
    pub fn md1(&self) -> MD1R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD1R { bits }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline]
    pub fn ctl0(&self) -> CTL0R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL0R { bits }
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline]
    pub fn md0(&self) -> MD0R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MD0R { bits }
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
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline]
    pub fn ctl7(&mut self) -> _CTL7W {
        _CTL7W { w: self }
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline]
    pub fn md7(&mut self) -> _MD7W {
        _MD7W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline]
    pub fn ctl6(&mut self) -> _CTL6W {
        _CTL6W { w: self }
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline]
    pub fn md6(&mut self) -> _MD6W {
        _MD6W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline]
    pub fn ctl5(&mut self) -> _CTL5W {
        _CTL5W { w: self }
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline]
    pub fn md5(&mut self) -> _MD5W {
        _MD5W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline]
    pub fn ctl4(&mut self) -> _CTL4W {
        _CTL4W { w: self }
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline]
    pub fn md4(&mut self) -> _MD4W {
        _MD4W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline]
    pub fn ctl3(&mut self) -> _CTL3W {
        _CTL3W { w: self }
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline]
    pub fn md3(&mut self) -> _MD3W {
        _MD3W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline]
    pub fn ctl2(&mut self) -> _CTL2W {
        _CTL2W { w: self }
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline]
    pub fn md2(&mut self) -> _MD2W {
        _MD2W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline]
    pub fn ctl1(&mut self) -> _CTL1W {
        _CTL1W { w: self }
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline]
    pub fn md1(&mut self) -> _MD1W {
        _MD1W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline]
    pub fn ctl0(&mut self) -> _CTL0W {
        _CTL0W { w: self }
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline]
    pub fn md0(&mut self) -> _MD0W {
        _MD0W { w: self }
    }
}
