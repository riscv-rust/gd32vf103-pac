#[doc = "Register `ALRML` writer"]
pub type W = crate::W<ALRML_SPEC>;
#[doc = "Field `ALRM` writer - alarm value low"]
pub type ALRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - alarm value low"]
    #[inline(always)]
    #[must_use]
    pub fn alrm(&mut self) -> ALRM_W<ALRML_SPEC, 0> {
        ALRM_W::new(self)
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
#[doc = "RTC alarm low register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrml::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRML_SPEC;
impl crate::RegisterSpec for ALRML_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrml::W`](W) writer structure"]
impl crate::Writable for ALRML_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRML to value 0xffff"]
impl crate::Resettable for ALRML_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
