#[doc = "Register `CLICINTCTL` reader"]
pub struct R(crate::R<CLICINTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLICINTCTL` writer"]
pub struct W(crate::W<CLICINTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLICINTCTL_SPEC>;
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
impl From<crate::W<CLICINTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLICINTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL_PRIORITY` reader - LEVEL_PRIORITY"]
pub type LEVEL_PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVEL_PRIORITY` writer - LEVEL_PRIORITY"]
pub type LEVEL_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CLICINTCTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LEVEL_PRIORITY"]
    #[inline(always)]
    pub fn level_priority(&self) -> LEVEL_PRIORITY_R {
        LEVEL_PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LEVEL_PRIORITY"]
    #[inline(always)]
    #[must_use]
    pub fn level_priority(&mut self) -> LEVEL_PRIORITY_W<0> {
        LEVEL_PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clicintctl Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicintctl](index.html) module"]
pub struct CLICINTCTL_SPEC;
impl crate::RegisterSpec for CLICINTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clicintctl::R](R) reader structure"]
impl crate::Readable for CLICINTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clicintctl::W](W) writer structure"]
impl crate::Writable for CLICINTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTCTL to value 0"]
impl crate::Resettable for CLICINTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
