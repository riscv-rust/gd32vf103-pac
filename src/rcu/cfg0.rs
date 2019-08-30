#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG0 {
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
pub struct SCSR {
    bits: u8,
}
impl SCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCSSR {
    bits: u8,
}
impl SCSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AHBPSCR {
    bits: u8,
}
impl AHBPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct APB1PSCR {
    bits: u8,
}
impl APB1PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct APB2PSCR {
    bits: u8,
}
impl APB2PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCPSC_1_0R {
    bits: u8,
}
impl ADCPSC_1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLSELR {
    bits: bool,
}
impl PLLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PREDV0_LSBR {
    bits: bool,
}
impl PREDV0_LSBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLLMF_3_0R {
    bits: u8,
}
impl PLLMF_3_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBFSPSCR {
    bits: u8,
}
impl USBFSPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CKOUT0SELR {
    bits: u8,
}
impl CKOUT0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCPSC_2R {
    bits: bool,
}
impl ADCPSC_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLLMF_4R {
    bits: bool,
}
impl PLLMF_4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _SCSW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSW<'a> {
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
#[doc = r" Proxy"]
pub struct _AHBPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _AHBPSCW<'a> {
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
pub struct _APB1PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _APB1PSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APB2PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _APB2PSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCPSC_1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCPSC_1_0W<'a> {
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
pub struct _PLLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREDV0_LSBW<'a> {
    w: &'a mut W,
}
impl<'a> _PREDV0_LSBW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLMF_3_0W<'a> {
    w: &'a mut W,
}
impl<'a> _PLLMF_3_0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBFSPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _USBFSPSCW<'a> {
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
pub struct _CKOUT0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKOUT0SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADCPSC_2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCPSC_2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLMF_4W<'a> {
    w: &'a mut W,
}
impl<'a> _PLLMF_4W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline]
    pub fn scs(&self) -> SCSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCSR { bits }
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline]
    pub fn scss(&self) -> SCSSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCSSR { bits }
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline]
    pub fn ahbpsc(&self) -> AHBPSCR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AHBPSCR { bits }
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline]
    pub fn apb1psc(&self) -> APB1PSCR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APB1PSCR { bits }
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline]
    pub fn apb2psc(&self) -> APB2PSCR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APB2PSCR { bits }
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline]
    pub fn adcpsc_1_0(&self) -> ADCPSC_1_0R {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCPSC_1_0R { bits }
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline]
    pub fn pllsel(&self) -> PLLSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLSELR { bits }
    }
    #[doc = "Bit 17 - The LSB of PREDV0 division factor"]
    #[inline]
    pub fn predv0_lsb(&self) -> PREDV0_LSBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREDV0_LSBR { bits }
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline]
    pub fn pllmf_3_0(&self) -> PLLMF_3_0R {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLMF_3_0R { bits }
    }
    #[doc = "Bits 22:23 - USBFS clock prescaler selection"]
    #[inline]
    pub fn usbfspsc(&self) -> USBFSPSCR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBFSPSCR { bits }
    }
    #[doc = "Bits 24:27 - CKOUT0 Clock Source Selection"]
    #[inline]
    pub fn ckout0sel(&self) -> CKOUT0SELR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CKOUT0SELR { bits }
    }
    #[doc = "Bit 28 - Bit 2 of ADCPSC"]
    #[inline]
    pub fn adcpsc_2(&self) -> ADCPSC_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCPSC_2R { bits }
    }
    #[doc = "Bit 29 - Bit 4 of PLLMF"]
    #[inline]
    pub fn pllmf_4(&self) -> PLLMF_4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLMF_4R { bits }
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
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline]
    pub fn scs(&mut self) -> _SCSW {
        _SCSW { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline]
    pub fn ahbpsc(&mut self) -> _AHBPSCW {
        _AHBPSCW { w: self }
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline]
    pub fn apb1psc(&mut self) -> _APB1PSCW {
        _APB1PSCW { w: self }
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline]
    pub fn apb2psc(&mut self) -> _APB2PSCW {
        _APB2PSCW { w: self }
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline]
    pub fn adcpsc_1_0(&mut self) -> _ADCPSC_1_0W {
        _ADCPSC_1_0W { w: self }
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline]
    pub fn pllsel(&mut self) -> _PLLSELW {
        _PLLSELW { w: self }
    }
    #[doc = "Bit 17 - The LSB of PREDV0 division factor"]
    #[inline]
    pub fn predv0_lsb(&mut self) -> _PREDV0_LSBW {
        _PREDV0_LSBW { w: self }
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline]
    pub fn pllmf_3_0(&mut self) -> _PLLMF_3_0W {
        _PLLMF_3_0W { w: self }
    }
    #[doc = "Bits 22:23 - USBFS clock prescaler selection"]
    #[inline]
    pub fn usbfspsc(&mut self) -> _USBFSPSCW {
        _USBFSPSCW { w: self }
    }
    #[doc = "Bits 24:27 - CKOUT0 Clock Source Selection"]
    #[inline]
    pub fn ckout0sel(&mut self) -> _CKOUT0SELW {
        _CKOUT0SELW { w: self }
    }
    #[doc = "Bit 28 - Bit 2 of ADCPSC"]
    #[inline]
    pub fn adcpsc_2(&mut self) -> _ADCPSC_2W {
        _ADCPSC_2W { w: self }
    }
    #[doc = "Bit 29 - Bit 4 of PLLMF"]
    #[inline]
    pub fn pllmf_4(&mut self) -> _PLLMF_4W {
        _PLLMF_4W { w: self }
    }
}
