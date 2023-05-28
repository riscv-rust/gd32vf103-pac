#[doc = "Register `DMATB` reader"]
pub struct R(crate::R<DMATB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATB` writer"]
pub struct W(crate::W<DMATB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATB_SPEC>;
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
impl From<crate::W<DMATB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATB` reader - DMA transfer buffer"]
pub type DMATB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMATB` writer - DMA transfer buffer"]
pub type DMATB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMATB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&self) -> DMATB_R {
        DMATB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dmatb(&mut self) -> DMATB_W<0> {
        DMATB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA transfer buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatb](index.html) module"]
pub struct DMATB_SPEC;
impl crate::RegisterSpec for DMATB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmatb::R](R) reader structure"]
impl crate::Readable for DMATB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatb::W](W) writer structure"]
impl crate::Writable for DMATB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATB to value 0"]
impl crate::Resettable for DMATB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
