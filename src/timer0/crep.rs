#[doc = "Register `CREP` reader"]
pub struct R(crate::R<CREP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CREP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CREP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CREP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CREP` writer"]
pub struct W(crate::W<CREP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CREP_SPEC>;
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
impl From<crate::W<CREP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CREP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CREP` reader - Counter repetition value"]
pub type CREP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CREP` writer - Counter repetition value"]
pub type CREP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CREP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    #[must_use]
    pub fn crep(&mut self) -> CREP_W<0> {
        CREP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter repetition register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crep](index.html) module"]
pub struct CREP_SPEC;
impl crate::RegisterSpec for CREP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crep::R](R) reader structure"]
impl crate::Readable for CREP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crep::W](W) writer structure"]
impl crate::Writable for CREP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CREP to value 0"]
impl crate::Resettable for CREP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
