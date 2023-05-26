#[doc = "Register `IOFF2` reader"]
pub struct R(crate::R<IOFF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOFF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOFF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOFF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOFF2` writer"]
pub struct W(crate::W<IOFF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOFF2_SPEC>;
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
impl From<crate::W<IOFF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOFF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOFF` reader - Data offset for inserted channel 2"]
pub type IOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOFF` writer - Data offset for inserted channel 2"]
pub type IOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOFF2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Data offset for inserted channel 2"]
    #[inline(always)]
    pub fn ioff(&self) -> IOFF_R {
        IOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for inserted channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ioff(&mut self) -> IOFF_W<0> {
        IOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inserted channel data offset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioff2](index.html) module"]
pub struct IOFF2_SPEC;
impl crate::RegisterSpec for IOFF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioff2::R](R) reader structure"]
impl crate::Readable for IOFF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioff2::W](W) writer structure"]
impl crate::Writable for IOFF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFF2 to value 0"]
impl crate::Resettable for IOFF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
