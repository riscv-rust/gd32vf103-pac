#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CHCTL0_INPUT {
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
pub struct CH1CAPFLTR {
    bits: u8,
}
impl CH1CAPFLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH1CAPPSCR {
    bits: u8,
}
impl CH1CAPPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH1MSR {
    bits: u8,
}
impl CH1MSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH0CAPFLTR {
    bits: u8,
}
impl CH0CAPFLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH0CAPPSCR {
    bits: u8,
}
impl CH0CAPPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH0MSR {
    bits: u8,
}
impl CH0MSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH1CAPFLTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1CAPFLTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1CAPPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1CAPPSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1MSW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1MSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0CAPFLTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0CAPFLTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0CAPPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0CAPPSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0MSW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0MSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline]
    pub fn ch1capflt(&self) -> CH1CAPFLTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH1CAPFLTR { bits }
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline]
    pub fn ch1cappsc(&self) -> CH1CAPPSCR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH1CAPPSCR { bits }
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline]
    pub fn ch1ms(&self) -> CH1MSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH1MSR { bits }
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline]
    pub fn ch0capflt(&self) -> CH0CAPFLTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH0CAPFLTR { bits }
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline]
    pub fn ch0cappsc(&self) -> CH0CAPPSCR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH0CAPPSCR { bits }
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline]
    pub fn ch0ms(&self) -> CH0MSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH0MSR { bits }
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline]
    pub fn ch1capflt(&mut self) -> _CH1CAPFLTW {
        _CH1CAPFLTW { w: self }
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline]
    pub fn ch1cappsc(&mut self) -> _CH1CAPPSCW {
        _CH1CAPPSCW { w: self }
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline]
    pub fn ch1ms(&mut self) -> _CH1MSW {
        _CH1MSW { w: self }
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline]
    pub fn ch0capflt(&mut self) -> _CH0CAPFLTW {
        _CH0CAPFLTW { w: self }
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline]
    pub fn ch0cappsc(&mut self) -> _CH0CAPPSCW {
        _CH0CAPPSCW { w: self }
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline]
    pub fn ch0ms(&mut self) -> _CH0MSW {
        _CH0MSW { w: self }
    }
}
