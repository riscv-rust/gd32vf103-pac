#[doc = "Register `DATA40` reader"]
pub struct R(crate::R<DATA40_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA40_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA40_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA40_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA40` writer"]
pub struct W(crate::W<DATA40_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA40_SPEC>;
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
impl From<crate::W<DATA40_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA40_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Backup data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DATA40_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register 40\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data40](index.html) module"]
pub struct DATA40_SPEC;
impl crate::RegisterSpec for DATA40_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [data40::R](R) reader structure"]
impl crate::Readable for DATA40_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data40::W](W) writer structure"]
impl crate::Writable for DATA40_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA40 to value 0"]
impl crate::Resettable for DATA40_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
