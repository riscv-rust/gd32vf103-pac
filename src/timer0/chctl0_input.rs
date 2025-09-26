#[doc = "Register `CHCTL0_Input` reader"]
pub type R = crate::R<CHCTL0_INPUT_SPEC>;
#[doc = "Register `CHCTL0_Input` writer"]
pub type W = crate::W<CHCTL0_INPUT_SPEC>;
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
pub type CH0MS_R = crate::FieldReader;
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
pub type CH0MS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0CAPPSC` reader - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_R = crate::FieldReader;
#[doc = "Field `CH0CAPPSC` writer - Channel 0 input capture prescaler"]
pub type CH0CAPPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH0CAPFLT` reader - Channel 0 input capture filter control"]
pub type CH0CAPFLT_R = crate::FieldReader;
#[doc = "Field `CH0CAPFLT` writer - Channel 0 input capture filter control"]
pub type CH0CAPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1MS` reader - Channel 1 mode selection"]
pub type CH1MS_R = crate::FieldReader;
#[doc = "Field `CH1MS` writer - Channel 1 mode selection"]
pub type CH1MS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1CAPPSC` reader - Channel 1 input capture prescaler"]
pub type CH1CAPPSC_R = crate::FieldReader;
#[doc = "Field `CH1CAPPSC` writer - Channel 1 input capture prescaler"]
pub type CH1CAPPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1CAPFLT` reader - Channel 1 input capture filter control"]
pub type CH1CAPFLT_R = crate::FieldReader;
#[doc = "Field `CH1CAPFLT` writer - Channel 1 input capture filter control"]
pub type CH1CAPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> CH0CAPPSC_R {
        CH0CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> CH0CAPFLT_R {
        CH0CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&self) -> CH1CAPPSC_R {
        CH1CAPPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    pub fn ch1capflt(&self) -> CH1CAPFLT_R {
        CH1CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ms(&mut self) -> CH0MS_W<CHCTL0_INPUT_SPEC> {
        CH0MS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cappsc(&mut self) -> CH0CAPPSC_W<CHCTL0_INPUT_SPEC> {
        CH0CAPPSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch0capflt(&mut self) -> CH0CAPFLT_W<CHCTL0_INPUT_SPEC> {
        CH0CAPFLT_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ms(&mut self) -> CH1MS_W<CHCTL0_INPUT_SPEC> {
        CH1MS_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cappsc(&mut self) -> CH1CAPPSC_W<CHCTL0_INPUT_SPEC> {
        CH1CAPPSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch1capflt(&mut self) -> CH1CAPFLT_W<CHCTL0_INPUT_SPEC> {
        CH1CAPFLT_W::new(self, 12)
    }
}
#[doc = "Channel control register 0 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL0_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL0_INPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl0_input::R`](R) reader structure"]
impl crate::Readable for CHCTL0_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl0_input::W`](W) writer structure"]
impl crate::Writable for CHCTL0_INPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CHCTL0_Input to value 0"]
impl crate::Resettable for CHCTL0_INPUT_SPEC {
    const RESET_VALUE: u16 = 0;
}
