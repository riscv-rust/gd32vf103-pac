#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SLP_HOLD_R = crate::BitReader;
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SLP_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DSLP_HOLD_R = crate::BitReader;
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DSLP_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type STB_HOLD_R = crate::BitReader;
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type STB_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold bit"]
pub type FWDGT_HOLD_R = crate::BitReader;
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold bit"]
pub type FWDGT_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold bit"]
pub type WWDGT_HOLD_R = crate::BitReader;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold bit"]
pub type WWDGT_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER0_HOLD` reader - TIMER 0 hold bit"]
pub type TIMER0_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER0_HOLD` writer - TIMER 0 hold bit"]
pub type TIMER0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1_HOLD` reader - TIMER 1 hold bit"]
pub type TIMER1_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER1_HOLD` writer - TIMER 1 hold bit"]
pub type TIMER1_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER2_HOLD` reader - TIMER 2 hold bit"]
pub type TIMER2_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER2_HOLD` writer - TIMER 2 hold bit"]
pub type TIMER2_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER3_HOLD` reader - TIMER 23 hold bit"]
pub type TIMER3_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER3_HOLD` writer - TIMER 23 hold bit"]
pub type TIMER3_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN0_HOLD` reader - CAN0 hold bit"]
pub type CAN0_HOLD_R = crate::BitReader;
#[doc = "Field `CAN0_HOLD` writer - CAN0 hold bit"]
pub type CAN0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold bit"]
pub type I2C0_HOLD_R = crate::BitReader;
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold bit"]
pub type I2C0_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold bit"]
pub type I2C1_HOLD_R = crate::BitReader;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold bit"]
pub type I2C1_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER4_HOLD` reader - TIMER4_HOLD"]
pub type TIMER4_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER4_HOLD` writer - TIMER4_HOLD"]
pub type TIMER4_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER5_HOLD` reader - TIMER 5 hold bit"]
pub type TIMER5_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER5_HOLD` writer - TIMER 5 hold bit"]
pub type TIMER5_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER6_HOLD` reader - TIMER 6 hold bit"]
pub type TIMER6_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER6_HOLD` writer - TIMER 6 hold bit"]
pub type TIMER6_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1_HOLD` reader - CAN1 hold bit"]
pub type CAN1_HOLD_R = crate::BitReader;
#[doc = "Field `CAN1_HOLD` writer - CAN1 hold bit"]
pub type CAN1_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SLP_HOLD_R {
        SLP_HOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DSLP_HOLD_R {
        DSLP_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    pub fn stb_hold(&self) -> STB_HOLD_R {
        STB_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FWDGT_HOLD_R {
        FWDGT_HOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WWDGT_HOLD_R {
        WWDGT_HOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> TIMER0_HOLD_R {
        TIMER0_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    pub fn timer1_hold(&self) -> TIMER1_HOLD_R {
        TIMER1_HOLD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> TIMER2_HOLD_R {
        TIMER2_HOLD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    pub fn timer3_hold(&self) -> TIMER3_HOLD_R {
        TIMER3_HOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    pub fn can0_hold(&self) -> CAN0_HOLD_R {
        CAN0_HOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2C0_HOLD_R {
        I2C0_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2C1_HOLD_R {
        I2C1_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER4_HOLD"]
    #[inline(always)]
    pub fn timer4_hold(&self) -> TIMER4_HOLD_R {
        TIMER4_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMER 5 hold bit"]
    #[inline(always)]
    pub fn timer5_hold(&self) -> TIMER5_HOLD_R {
        TIMER5_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMER 6 hold bit"]
    #[inline(always)]
    pub fn timer6_hold(&self) -> TIMER6_HOLD_R {
        TIMER6_HOLD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    pub fn can1_hold(&self) -> CAN1_HOLD_R {
        CAN1_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn slp_hold(&mut self) -> SLP_HOLD_W<CTL_SPEC, 0> {
        SLP_HOLD_W::new(self)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W<CTL_SPEC, 1> {
        DSLP_HOLD_W::new(self)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn stb_hold(&mut self) -> STB_HOLD_W<CTL_SPEC, 2> {
        STB_HOLD_W::new(self)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn fwdgt_hold(&mut self) -> FWDGT_HOLD_W<CTL_SPEC, 8> {
        FWDGT_HOLD_W::new(self)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgt_hold(&mut self) -> WWDGT_HOLD_W<CTL_SPEC, 9> {
        WWDGT_HOLD_W::new(self)
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_hold(&mut self) -> TIMER0_HOLD_W<CTL_SPEC, 10> {
        TIMER0_HOLD_W::new(self)
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_hold(&mut self) -> TIMER1_HOLD_W<CTL_SPEC, 11> {
        TIMER1_HOLD_W::new(self)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_hold(&mut self) -> TIMER2_HOLD_W<CTL_SPEC, 12> {
        TIMER2_HOLD_W::new(self)
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_hold(&mut self) -> TIMER3_HOLD_W<CTL_SPEC, 13> {
        TIMER3_HOLD_W::new(self)
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn can0_hold(&mut self) -> CAN0_HOLD_W<CTL_SPEC, 14> {
        CAN0_HOLD_W::new(self)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_hold(&mut self) -> I2C0_HOLD_W<CTL_SPEC, 15> {
        I2C0_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_hold(&mut self) -> I2C1_HOLD_W<CTL_SPEC, 16> {
        I2C1_HOLD_W::new(self)
    }
    #[doc = "Bit 18 - TIMER4_HOLD"]
    #[inline(always)]
    #[must_use]
    pub fn timer4_hold(&mut self) -> TIMER4_HOLD_W<CTL_SPEC, 18> {
        TIMER4_HOLD_W::new(self)
    }
    #[doc = "Bit 19 - TIMER 5 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_hold(&mut self) -> TIMER5_HOLD_W<CTL_SPEC, 19> {
        TIMER5_HOLD_W::new(self)
    }
    #[doc = "Bit 20 - TIMER 6 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer6_hold(&mut self) -> TIMER6_HOLD_W<CTL_SPEC, 20> {
        TIMER6_HOLD_W::new(self)
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn can1_hold(&mut self) -> CAN1_HOLD_W<CTL_SPEC, 21> {
        CAN1_HOLD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
