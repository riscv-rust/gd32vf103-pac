#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold register"]
pub type SLP_HOLD_R = crate::BitReader;
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold register"]
pub type SLP_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold register"]
pub type DSLP_HOLD_R = crate::BitReader;
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold register"]
pub type DSLP_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB_HOLD` reader - Standby mode hold register"]
pub type STB_HOLD_R = crate::BitReader;
#[doc = "Field `STB_HOLD` writer - Standby mode hold register"]
pub type STB_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold bit"]
pub type FWDGT_HOLD_R = crate::BitReader;
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold bit"]
pub type FWDGT_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold bit"]
pub type WWDGT_HOLD_R = crate::BitReader;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold bit"]
pub type WWDGT_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_HOLD` reader - TIMER 0 hold bit"]
pub type TIMER0_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER0_HOLD` writer - TIMER 0 hold bit"]
pub type TIMER0_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_HOLD` reader - TIMER 1 hold bit"]
pub type TIMER1_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER1_HOLD` writer - TIMER 1 hold bit"]
pub type TIMER1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_HOLD` reader - TIMER 2 hold bit"]
pub type TIMER2_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER2_HOLD` writer - TIMER 2 hold bit"]
pub type TIMER2_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3_HOLD` reader - TIMER 23 hold bit"]
pub type TIMER3_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER3_HOLD` writer - TIMER 23 hold bit"]
pub type TIMER3_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0_HOLD` reader - CAN0 hold bit"]
pub type CAN0_HOLD_R = crate::BitReader;
#[doc = "Field `CAN0_HOLD` writer - CAN0 hold bit"]
pub type CAN0_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold bit"]
pub type I2C0_HOLD_R = crate::BitReader;
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold bit"]
pub type I2C0_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold bit"]
pub type I2C1_HOLD_R = crate::BitReader;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold bit"]
pub type I2C1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4_HOLD` reader - TIMER4_HOLD"]
pub type TIMER4_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER4_HOLD` writer - TIMER4_HOLD"]
pub type TIMER4_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER5_HOLD` reader - TIMER 5 hold bit"]
pub type TIMER5_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER5_HOLD` writer - TIMER 5 hold bit"]
pub type TIMER5_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6_HOLD` reader - TIMER 6 hold bit"]
pub type TIMER6_HOLD_R = crate::BitReader;
#[doc = "Field `TIMER6_HOLD` writer - TIMER 6 hold bit"]
pub type TIMER6_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1_HOLD` reader - CAN1 hold bit"]
pub type CAN1_HOLD_R = crate::BitReader;
#[doc = "Field `CAN1_HOLD` writer - CAN1 hold bit"]
pub type CAN1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn slp_hold(&mut self) -> SLP_HOLD_W<CTL_SPEC> {
        SLP_HOLD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn dslp_hold(&mut self) -> DSLP_HOLD_W<CTL_SPEC> {
        DSLP_HOLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Standby mode hold register"]
    #[inline(always)]
    #[must_use]
    pub fn stb_hold(&mut self) -> STB_HOLD_W<CTL_SPEC> {
        STB_HOLD_W::new(self, 2)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn fwdgt_hold(&mut self) -> FWDGT_HOLD_W<CTL_SPEC> {
        FWDGT_HOLD_W::new(self, 8)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgt_hold(&mut self) -> WWDGT_HOLD_W<CTL_SPEC> {
        WWDGT_HOLD_W::new(self, 9)
    }
    #[doc = "Bit 10 - TIMER 0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_hold(&mut self) -> TIMER0_HOLD_W<CTL_SPEC> {
        TIMER0_HOLD_W::new(self, 10)
    }
    #[doc = "Bit 11 - TIMER 1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_hold(&mut self) -> TIMER1_HOLD_W<CTL_SPEC> {
        TIMER1_HOLD_W::new(self, 11)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_hold(&mut self) -> TIMER2_HOLD_W<CTL_SPEC> {
        TIMER2_HOLD_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIMER 23 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_hold(&mut self) -> TIMER3_HOLD_W<CTL_SPEC> {
        TIMER3_HOLD_W::new(self, 13)
    }
    #[doc = "Bit 14 - CAN0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn can0_hold(&mut self) -> CAN0_HOLD_W<CTL_SPEC> {
        CAN0_HOLD_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_hold(&mut self) -> I2C0_HOLD_W<CTL_SPEC> {
        I2C0_HOLD_W::new(self, 15)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_hold(&mut self) -> I2C1_HOLD_W<CTL_SPEC> {
        I2C1_HOLD_W::new(self, 16)
    }
    #[doc = "Bit 18 - TIMER4_HOLD"]
    #[inline(always)]
    #[must_use]
    pub fn timer4_hold(&mut self) -> TIMER4_HOLD_W<CTL_SPEC> {
        TIMER4_HOLD_W::new(self, 18)
    }
    #[doc = "Bit 19 - TIMER 5 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_hold(&mut self) -> TIMER5_HOLD_W<CTL_SPEC> {
        TIMER5_HOLD_W::new(self, 19)
    }
    #[doc = "Bit 20 - TIMER 6 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer6_hold(&mut self) -> TIMER6_HOLD_W<CTL_SPEC> {
        TIMER6_HOLD_W::new(self, 20)
    }
    #[doc = "Bit 21 - CAN1 hold bit"]
    #[inline(always)]
    #[must_use]
    pub fn can1_hold(&mut self) -> CAN1_HOLD_W<CTL_SPEC> {
        CAN1_HOLD_W::new(self, 21)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
