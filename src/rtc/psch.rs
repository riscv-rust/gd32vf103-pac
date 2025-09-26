#[doc = "Register `PSCH` writer"]
pub type W = crate::W<PSCH_SPEC>;
#[doc = "Field `PSC` writer - RTC prescaler value high"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl W {
    #[doc = "Bits 0:3 - RTC prescaler value high"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSCH_SPEC, 0> {
        PSC_W::new(self)
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
#[doc = "RTC prescaler high register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCH_SPEC;
impl crate::RegisterSpec for PSCH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`psch::W`](W) writer structure"]
impl crate::Writable for PSCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCH to value 0"]
impl crate::Resettable for PSCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
