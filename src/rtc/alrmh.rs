#[doc = "Register `ALRMH` writer"]
pub struct W(crate::W<ALRMH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMH_SPEC>;
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
impl From<crate::W<ALRMH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRM` writer - Alarm value high"]
pub type ALRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMH_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Alarm value high"]
    #[inline(always)]
    #[must_use]
    pub fn alrm(&mut self) -> ALRM_W<0> {
        ALRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm high register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmh](index.html) module"]
pub struct ALRMH_SPEC;
impl crate::RegisterSpec for ALRMH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alrmh::W](W) writer structure"]
impl crate::Writable for ALRMH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRMH to value 0xffff"]
impl crate::Resettable for ALRMH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
