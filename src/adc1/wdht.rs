#[doc = "Register `WDHT` reader"]
pub struct R(crate::R<WDHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDHT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDHT` writer"]
pub struct W(crate::W<WDHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDHT_SPEC>;
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
impl From<crate::W<WDHT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDHT` reader - Analog watchdog higher threshold"]
pub type WDHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDHT` writer - Analog watchdog higher threshold"]
pub type WDHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDHT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn wdht(&self) -> WDHT_R {
        WDHT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdht(&mut self) -> WDHT_W<0> {
        WDHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog higher threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdht](index.html) module"]
pub struct WDHT_SPEC;
impl crate::RegisterSpec for WDHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdht::R](R) reader structure"]
impl crate::Readable for WDHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdht::W](W) writer structure"]
impl crate::Writable for WDHT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDHT to value 0x0fff"]
impl crate::Resettable for WDHT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
