#[doc = "Register `PSCH` writer"]
pub struct W(crate::W<PSCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PSCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` writer - RTC prescaler value high"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSCH_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bits 0:3 - RTC prescaler value high"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC prescaler high register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psch](index.html) module"]
pub struct PSCH_SPEC;
impl crate::RegisterSpec for PSCH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [psch::W](W) writer structure"]
impl crate::Writable for PSCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCH to value 0"]
impl crate::Resettable for PSCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
