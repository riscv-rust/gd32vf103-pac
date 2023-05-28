#[doc = "Register `mtimecmp_lo` reader"]
pub struct R(crate::R<MTIMECMP_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIMECMP_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIMECMP_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIMECMP_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mtimecmp_lo` writer"]
pub struct W(crate::W<MTIMECMP_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIMECMP_LO_SPEC>;
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
impl From<crate::W<MTIMECMP_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIMECMP_LO_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer comparison value (lower half)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtimecmp_lo](index.html) module"]
pub struct MTIMECMP_LO_SPEC;
impl crate::RegisterSpec for MTIMECMP_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtimecmp_lo::R](R) reader structure"]
impl crate::Readable for MTIMECMP_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtimecmp_lo::W](W) writer structure"]
impl crate::Writable for MTIMECMP_LO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mtimecmp_lo to value 0xffff_ffff"]
impl crate::Resettable for MTIMECMP_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
