#[doc = "Register `WS` reader"]
pub struct R(crate::R<WS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WS` writer"]
pub struct W(crate::W<WS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WS_SPEC>;
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
impl From<crate::W<WS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WSCNT` reader - wait state counter register"]
pub type WSCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WSCNT` writer - wait state counter register"]
pub type WSCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WSCNT_R {
        WSCNT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    #[must_use]
    pub fn wscnt(&mut self) -> WSCNT_W<0> {
        WSCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wait state counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ws](index.html) module"]
pub struct WS_SPEC;
impl crate::RegisterSpec for WS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ws::R](R) reader structure"]
impl crate::Readable for WS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ws::W](W) writer structure"]
impl crate::Writable for WS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WS to value 0"]
impl crate::Resettable for WS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
