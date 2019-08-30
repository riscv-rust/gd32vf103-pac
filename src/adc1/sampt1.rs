#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAMPT1 {
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
pub struct SPT0R {
    bits: u8,
}
impl SPT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT1R {
    bits: u8,
}
impl SPT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT2R {
    bits: u8,
}
impl SPT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT3R {
    bits: u8,
}
impl SPT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT4R {
    bits: u8,
}
impl SPT4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT5R {
    bits: u8,
}
impl SPT5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT6R {
    bits: u8,
}
impl SPT6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT7R {
    bits: u8,
}
impl SPT7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT8R {
    bits: u8,
}
impl SPT8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPT9R {
    bits: u8,
}
impl SPT9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SPT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT0W<'a> {
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
pub struct _SPT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT1W<'a> {
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
pub struct _SPT2W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT2W<'a> {
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
pub struct _SPT3W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT3W<'a> {
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
pub struct _SPT4W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT4W<'a> {
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
pub struct _SPT5W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT5W<'a> {
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
pub struct _SPT6W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT6W<'a> {
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
pub struct _SPT7W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT7W<'a> {
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
#[doc = r" Proxy"]
pub struct _SPT8W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPT9W<'a> {
    w: &'a mut W,
}
impl<'a> _SPT9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline]
    pub fn spt0(&self) -> SPT0R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT0R { bits }
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline]
    pub fn spt1(&self) -> SPT1R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT1R { bits }
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline]
    pub fn spt2(&self) -> SPT2R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT2R { bits }
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline]
    pub fn spt3(&self) -> SPT3R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT3R { bits }
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline]
    pub fn spt4(&self) -> SPT4R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT4R { bits }
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline]
    pub fn spt5(&self) -> SPT5R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT5R { bits }
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline]
    pub fn spt6(&self) -> SPT6R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT6R { bits }
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline]
    pub fn spt7(&self) -> SPT7R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT7R { bits }
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline]
    pub fn spt8(&self) -> SPT8R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT8R { bits }
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline]
    pub fn spt9(&self) -> SPT9R {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPT9R { bits }
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
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline]
    pub fn spt0(&mut self) -> _SPT0W {
        _SPT0W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline]
    pub fn spt1(&mut self) -> _SPT1W {
        _SPT1W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline]
    pub fn spt2(&mut self) -> _SPT2W {
        _SPT2W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline]
    pub fn spt3(&mut self) -> _SPT3W {
        _SPT3W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline]
    pub fn spt4(&mut self) -> _SPT4W {
        _SPT4W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline]
    pub fn spt5(&mut self) -> _SPT5W {
        _SPT5W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline]
    pub fn spt6(&mut self) -> _SPT6W {
        _SPT6W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline]
    pub fn spt7(&mut self) -> _SPT7W {
        _SPT7W { w: self }
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline]
    pub fn spt8(&mut self) -> _SPT8W {
        _SPT8W { w: self }
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline]
    pub fn spt9(&mut self) -> _SPT9W {
        _SPT9W { w: self }
    }
}
