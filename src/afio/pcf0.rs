#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCF0 {
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
pub struct TIMER1ITI1_REMAPR {
    bits: bool,
}
impl TIMER1ITI1_REMAPR {
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
pub struct SPI2_REMAPR {
    bits: bool,
}
impl SPI2_REMAPR {
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
pub struct SWJ_CFGR {
    bits: u8,
}
impl SWJ_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAN1_REMAPR {
    bits: bool,
}
impl CAN1_REMAPR {
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
pub struct TIMER4CH3_IREMAPR {
    bits: bool,
}
impl TIMER4CH3_IREMAPR {
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
pub struct PD01_REMAPR {
    bits: bool,
}
impl PD01_REMAPR {
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
pub struct CAN0_REMAPR {
    bits: u8,
}
impl CAN0_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMER3_REMAPR {
    bits: bool,
}
impl TIMER3_REMAPR {
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
pub struct TIMER2_REMAPR {
    bits: u8,
}
impl TIMER2_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMER1_REMAPR {
    bits: u8,
}
impl TIMER1_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMER0_REMAPR {
    bits: u8,
}
impl TIMER0_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART2_REMAPR {
    bits: u8,
}
impl USART2_REMAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART1_REMAPR {
    bits: bool,
}
impl USART1_REMAPR {
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
pub struct USART0_REMAPR {
    bits: bool,
}
impl USART0_REMAPR {
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
pub struct I2C0_REMAPR {
    bits: bool,
}
impl I2C0_REMAPR {
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
pub struct SPI0_REMAPR {
    bits: bool,
}
impl SPI0_REMAPR {
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
pub struct _TIMER1ITI1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1ITI1_REMAPW<'a> {
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
#[doc = r" Proxy"]
pub struct _SPI2_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2_REMAPW<'a> {
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
pub struct _SWJ_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWJ_CFGW<'a> {
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
pub struct _CAN1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1_REMAPW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMER4CH3_IREMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER4CH3_IREMAPW<'a> {
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
pub struct _PD01_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _PD01_REMAPW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAN0_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN0_REMAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMER3_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER3_REMAPW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMER2_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER2_REMAPW<'a> {
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
pub struct _TIMER1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_REMAPW<'a> {
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
pub struct _TIMER0_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_REMAPW<'a> {
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
pub struct _USART2_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2_REMAPW<'a> {
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
pub struct _USART1_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1_REMAPW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USART0_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _USART0_REMAPW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C0_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_REMAPW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPI0_REMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0_REMAPW<'a> {
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
    #[doc = "Bit 29 - TIMER1 internal trigger 1 remapping"]
    #[inline]
    pub fn timer1iti1_remap(&self) -> TIMER1ITI1_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMER1ITI1_REMAPR { bits }
    }
    #[doc = "Bit 28 - SPI2/I2S2 remapping"]
    #[inline]
    pub fn spi2_remap(&self) -> SPI2_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPI2_REMAPR { bits }
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline]
    pub fn swj_cfg(&self) -> SWJ_CFGR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SWJ_CFGR { bits }
    }
    #[doc = "Bit 22 - CAN1 I/O remapping"]
    #[inline]
    pub fn can1_remap(&self) -> CAN1_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAN1_REMAPR { bits }
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline]
    pub fn timer4ch3_iremap(&self) -> TIMER4CH3_IREMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMER4CH3_IREMAPR { bits }
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline]
    pub fn pd01_remap(&self) -> PD01_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PD01_REMAPR { bits }
    }
    #[doc = "Bits 13:14 - CAN0 alternate interface remapping"]
    #[inline]
    pub fn can0_remap(&self) -> CAN0_REMAPR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAN0_REMAPR { bits }
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline]
    pub fn timer3_remap(&self) -> TIMER3_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMER3_REMAPR { bits }
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline]
    pub fn timer2_remap(&self) -> TIMER2_REMAPR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER2_REMAPR { bits }
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline]
    pub fn timer1_remap(&self) -> TIMER1_REMAPR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER1_REMAPR { bits }
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline]
    pub fn timer0_remap(&self) -> TIMER0_REMAPR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMER0_REMAPR { bits }
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline]
    pub fn usart2_remap(&self) -> USART2_REMAPR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART2_REMAPR { bits }
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline]
    pub fn usart1_remap(&self) -> USART1_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USART1_REMAPR { bits }
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline]
    pub fn usart0_remap(&self) -> USART0_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USART0_REMAPR { bits }
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline]
    pub fn i2c0_remap(&self) -> I2C0_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C0_REMAPR { bits }
    }
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline]
    pub fn spi0_remap(&self) -> SPI0_REMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPI0_REMAPR { bits }
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
    #[doc = "Bit 29 - TIMER1 internal trigger 1 remapping"]
    #[inline]
    pub fn timer1iti1_remap(&mut self) -> _TIMER1ITI1_REMAPW {
        _TIMER1ITI1_REMAPW { w: self }
    }
    #[doc = "Bit 28 - SPI2/I2S2 remapping"]
    #[inline]
    pub fn spi2_remap(&mut self) -> _SPI2_REMAPW {
        _SPI2_REMAPW { w: self }
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline]
    pub fn swj_cfg(&mut self) -> _SWJ_CFGW {
        _SWJ_CFGW { w: self }
    }
    #[doc = "Bit 22 - CAN1 I/O remapping"]
    #[inline]
    pub fn can1_remap(&mut self) -> _CAN1_REMAPW {
        _CAN1_REMAPW { w: self }
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline]
    pub fn timer4ch3_iremap(&mut self) -> _TIMER4CH3_IREMAPW {
        _TIMER4CH3_IREMAPW { w: self }
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline]
    pub fn pd01_remap(&mut self) -> _PD01_REMAPW {
        _PD01_REMAPW { w: self }
    }
    #[doc = "Bits 13:14 - CAN0 alternate interface remapping"]
    #[inline]
    pub fn can0_remap(&mut self) -> _CAN0_REMAPW {
        _CAN0_REMAPW { w: self }
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline]
    pub fn timer3_remap(&mut self) -> _TIMER3_REMAPW {
        _TIMER3_REMAPW { w: self }
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline]
    pub fn timer2_remap(&mut self) -> _TIMER2_REMAPW {
        _TIMER2_REMAPW { w: self }
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline]
    pub fn timer1_remap(&mut self) -> _TIMER1_REMAPW {
        _TIMER1_REMAPW { w: self }
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline]
    pub fn timer0_remap(&mut self) -> _TIMER0_REMAPW {
        _TIMER0_REMAPW { w: self }
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline]
    pub fn usart2_remap(&mut self) -> _USART2_REMAPW {
        _USART2_REMAPW { w: self }
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline]
    pub fn usart1_remap(&mut self) -> _USART1_REMAPW {
        _USART1_REMAPW { w: self }
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline]
    pub fn usart0_remap(&mut self) -> _USART0_REMAPW {
        _USART0_REMAPW { w: self }
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline]
    pub fn i2c0_remap(&mut self) -> _I2C0_REMAPW {
        _I2C0_REMAPW { w: self }
    }
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline]
    pub fn spi0_remap(&mut self) -> _SPI0_REMAPW {
        _SPI0_REMAPW { w: self }
    }
}
