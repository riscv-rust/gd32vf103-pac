#[doc = "Register `CHCTL1_Input` reader"]
pub type R = crate::R<CHCTL1_INPUT_SPEC>;
#[doc = "Register `CHCTL1_Input` writer"]
pub type W = crate::W<CHCTL1_INPUT_SPEC>;
#[doc = "Field `CH2MS` reader - Channel 2 mode selection"]
pub type CH2MS_R = crate::FieldReader;
#[doc = "Field `CH2MS` writer - Channel 2 mode selection"]
pub type CH2MS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2CAPPSC` reader - Channel 2 input capture prescaler"]
pub type CH2CAPPSC_R = crate::FieldReader;
#[doc = "Field `CH2CAPPSC` writer - Channel 2 input capture prescaler"]
pub type CH2CAPPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2CAPFLT` reader - Channel 2 input capture filter control"]
pub type CH2CAPFLT_R = crate::FieldReader;
#[doc = "Field `CH2CAPFLT` writer - Channel 2 input capture filter control"]
pub type CH2CAPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH3MS` reader - Channel 3 mode selection"]
pub type CH3MS_R = crate::FieldReader;
#[doc = "Field `CH3MS` writer - Channel 3 mode selection"]
pub type CH3MS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH3CAPPSC` reader - Channel 3 input capture prescaler"]
pub type CH3CAPPSC_R = crate::FieldReader;
#[doc = "Field `CH3CAPPSC` writer - Channel 3 input capture prescaler"]
pub type CH3CAPPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH3CAPFLT` reader - Channel 3 input capture filter control"]
pub type CH3CAPFLT_R = crate::FieldReader;
#[doc = "Field `CH3CAPFLT` writer - Channel 3 input capture filter control"]
pub type CH3CAPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&self) -> CH2CAPPSC_R {
        CH2CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&self) -> CH2CAPFLT_R {
        CH2CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&self) -> CH3CAPPSC_R {
        CH3CAPPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&self) -> CH3CAPFLT_R {
        CH3CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ms(&mut self) -> CH2MS_W<CHCTL1_INPUT_SPEC> {
        CH2MS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cappsc(&mut self) -> CH2CAPPSC_W<CHCTL1_INPUT_SPEC> {
        CH2CAPPSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch2capflt(&mut self) -> CH2CAPFLT_W<CHCTL1_INPUT_SPEC> {
        CH2CAPFLT_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ms(&mut self) -> CH3MS_W<CHCTL1_INPUT_SPEC> {
        CH3MS_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cappsc(&mut self) -> CH3CAPPSC_W<CHCTL1_INPUT_SPEC> {
        CH3CAPPSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch3capflt(&mut self) -> CH3CAPFLT_W<CHCTL1_INPUT_SPEC> {
        CH3CAPFLT_W::new(self, 12)
    }
}
#[doc = "Channel control register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL1_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_INPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl1_input::R`](R) reader structure"]
impl crate::Readable for CHCTL1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl1_input::W`](W) writer structure"]
impl crate::Writable for CHCTL1_INPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CHCTL1_Input to value 0"]
impl crate::Resettable for CHCTL1_INPUT_SPEC {
    const RESET_VALUE: u16 = 0;
}
