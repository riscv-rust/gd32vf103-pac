#[doc = "Register `FDATA` reader"]
pub struct R(crate::R<FDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDATA` writer"]
pub struct W(crate::W<FDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDATA_SPEC>;
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
impl From<crate::W<FDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDATA` reader - Free Data Register bits"]
pub type FDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDATA` writer - Free Data Register bits"]
pub type FDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    pub fn fdata(&self) -> FDATA_R {
        FDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    #[must_use]
    pub fn fdata(&mut self) -> FDATA_W<0> {
        FDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Free data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdata](index.html) module"]
pub struct FDATA_SPEC;
impl crate::RegisterSpec for FDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdata::R](R) reader structure"]
impl crate::Readable for FDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdata::W](W) writer structure"]
impl crate::Writable for FDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDATA to value 0"]
impl crate::Resettable for FDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
