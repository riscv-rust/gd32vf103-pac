#[doc = "Register `PSCH` writer"]
pub type W = crate::W<PSCH_SPEC>;
#[doc = "Field `PSC` writer - RTC prescaler value high"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC prescaler value high"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSCH_SPEC> {
        PSC_W::new(self, 0)
    }
}
#[doc = "RTC prescaler high register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCH_SPEC;
impl crate::RegisterSpec for PSCH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`psch::W`](W) writer structure"]
impl crate::Writable for PSCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCH to value 0"]
impl crate::Resettable for PSCH_SPEC {
    const RESET_VALUE: u32 = 0;
}
