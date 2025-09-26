#[doc = "Register `PSCL` writer"]
pub type W = crate::W<PSCL_SPEC>;
#[doc = "Field `PSC` writer - RTC prescaler value low"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC prescaler value low"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSCL_SPEC> {
        PSC_W::new(self, 0)
    }
}
#[doc = "RTC prescaler low register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCL_SPEC;
impl crate::RegisterSpec for PSCL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscl::W`](W) writer structure"]
impl crate::Writable for PSCL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCL to value 0x8000"]
impl crate::Resettable for PSCL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
